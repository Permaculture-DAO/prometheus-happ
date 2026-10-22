"""Fabricated host identities only; NO actual physical cross-host evidence."""
import copy
import json
import subprocess
import sys
import tempfile
from pathlib import Path
import unittest
from g53_preflight import PIN, SCHEMA, PreflightRejected, validate_pair
SOURCE = "c" * 64
POLICY = dict(schema=SCHEMA, approved_test_source_sha=SOURCE,
              min_disk_free_bytes=6000000000, min_ram_available_bytes=3000000000,
              review_authorization_ref="TEST-fictional-authorization")
def host(role):
    return dict(schema=SCHEMA, role=role, test_source_sha=SOURCE, **PIN,
                host_id_sha256=("a" if role == "A" else "b") * 64,
                agent_pubkey_sha256=("1" if role == "A" else "2") * 64,
                state_root_sha256=("3" if role == "A" else "4") * 64,
                failure_domain_ref="TEST-domain-" + role,
                observation_ref="TEST-observation-" + role,
                free_disk_bytes=9000000000, available_ram_bytes=5000000000,
                production_runtime_touched=False, real_persistence_enabled=False)
class G53ContractTest(unittest.TestCase):
    def setUp(self):
        self.policy=copy.deepcopy(POLICY)
        self.a=host("A")
        self.b=host("B")
    def check(self):
        return validate_pair(self.policy,self.a,self.b)
    def rejects(self,text):
        with self.assertRaisesRegex(PreflightRejected,text):
            self.check()
    def test_fabricated_valid_schema_is_still_hold(self):
        self.assertIn("G53_MANIFEST_PAIR_SCHEMA=PASS",self.check())
        self.assertIn("G53_GO_NO_GO=HOLD",self.check())
    def test_same_host(self):
        self.b["host_id_sha256"]=self.a["host_id_sha256"]
        self.rejects("host_id_sha256:NOT_DISTINCT")
    def test_same_agent(self):
        self.b["agent_pubkey_sha256"]=self.a["agent_pubkey_sha256"]
        self.rejects("agent_pubkey_sha256:NOT_DISTINCT")
    def test_same_state(self):
        self.b["state_root_sha256"]=self.a["state_root_sha256"]
        self.rejects("state_root_sha256:NOT_DISTINCT")
    def test_same_domain(self):
        self.b["failure_domain_ref"]=self.a["failure_domain_ref"]
        self.rejects("failure_domain_ref:NOT_DISTINCT")
    def test_same_observation(self):
        self.b["observation_ref"]=self.a["observation_ref"]
        self.rejects("observation_ref:NOT_DISTINCT")
    def test_tampered_dna(self):
        self.b["dna_sha256"]="0"*64
        self.rejects("dna_sha256:PIN_MISMATCH")
    def test_source_mismatch(self):
        self.a["test_source_sha"]="f"*64
        self.rejects("SOURCE_MISMATCH")
    def test_disk_shortage(self):
        self.b["free_disk_bytes"]=100
        self.rejects("INSUFFICIENT_DISK")
    def test_ram_shortage(self):
        self.a["available_ram_bytes"]=1
        self.rejects("INSUFFICIENT_RAM")
    def test_production_violation(self):
        self.a["production_runtime_touched"]=True
        self.rejects("PRODUCTION_BOUNDARY")
    def test_real_gate_violation(self):
        self.b["real_persistence_enabled"]=True
        self.rejects("REAL_GATE_OPEN")
    def test_unknown_field(self):
        self.a["unknown_field"]=42
        self.rejects("KEY_SET_MISMATCH")
    def test_boolean_not_integer(self):
        self.a["free_disk_bytes"]=True
        self.rejects("NOT_POSITIVE_INT")
    def test_policy_wrong_source(self):
        self.policy["approved_test_source_sha"]="f"*64
        self.rejects("SOURCE_MISMATCH")

class G53CliBoundaryTest(unittest.TestCase):
    """Exercise the actual CLI/file decoder, not only the in-memory function."""
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.paths = {k: self.root / (k + ".json") for k in ("policy", "a", "b")}
        self.paths["policy"].write_text(json.dumps(POLICY), encoding="utf-8")
        self.paths["a"].write_text(json.dumps(host("A")), encoding="utf-8")
        self.paths["b"].write_text(json.dumps(host("B")), encoding="utf-8")

    def cli(self):
        return subprocess.run(
            [sys.executable, str(Path(__file__).with_name("g53_preflight.py")),
             "--policy", str(self.paths["policy"]),
             "--host-a", str(self.paths["a"]),
             "--host-b", str(self.paths["b"])],
            capture_output=True, text=True, timeout=5, check=False)

    def test_valid_fabricated_json_never_claims_physical_go(self):
        p = self.cli()
        self.assertEqual(p.returncode, 0, p.stderr)
        self.assertIn("G53_MANIFEST_PAIR_SCHEMA=PASS", p.stdout)
        self.assertIn("G53_GO_NO_GO=HOLD", p.stdout)
        self.assertIn("G53_PHYSICAL_INDEPENDENCE=NOT_PROVEN", p.stdout)
        self.assertNotIn("G53_GO_NO_GO=GO", p.stdout)

    def test_duplicate_host_field_fails_closed(self):
        path = self.paths["a"]
        obj = json.loads(path.read_text(encoding="utf-8"))
        raw = json.dumps(obj)
        raw = raw[:-1] + ', "role": "A"}'
        path.write_text(raw, encoding="utf-8")
        p = self.cli()
        self.assertEqual(p.returncode, 2)
        self.assertIn("INPUT:DUPLICATE_JSON_KEY:role", p.stderr)
        self.assertNotIn("SCHEMA=PASS", p.stdout)

    def test_duplicate_policy_field_fails_closed(self):
        path = self.paths["policy"]
        raw = path.read_text(encoding="utf-8")
        path.write_text(raw[:-1] + ', "schema": "PRM-G53-PREFLIGHT-v1"}', encoding="utf-8")
        p = self.cli()
        self.assertEqual(p.returncode, 2)
        self.assertIn("INPUT:DUPLICATE_JSON_KEY:schema", p.stderr)

    def test_truncated_json_fails_closed(self):
        self.paths["b"].write_text('{"schema":', encoding="utf-8")
        p = self.cli()
        self.assertEqual(p.returncode, 2)
        self.assertIn("G53_MANIFEST_PAIR_SCHEMA=FAIL", p.stderr)

    def test_non_finite_json_fails_closed(self):
        pth = self.paths["a"]
        raw = pth.read_text(encoding="utf-8")
        pth.write_text(raw[:-1] + ', "non_finite": NaN}', encoding="utf-8")
        p = self.cli()
        self.assertEqual(p.returncode, 2)
        self.assertIn("INPUT:NONFINITE_NUMBER", p.stderr)

    def test_oversized_json_fails_closed(self):
        self.paths["a"].write_text(" " * 32769, encoding="utf-8")
        p = self.cli()
        self.assertEqual(p.returncode, 2)
        self.assertIn("MISSING_OR_TOO_LARGE", p.stderr)

    def test_missing_manifest_fails_closed(self):
        self.paths["b"].unlink()
        p = self.cli()
        self.assertEqual(p.returncode, 2)
        self.assertIn("MISSING_OR_TOO_LARGE", p.stderr)

    def test_cli_detects_pinned_digest_mismatch(self):
        obj = json.loads(self.paths["b"].read_text(encoding="utf-8"))
        obj["coordinator_wasm_sha256"] = "0" * 64
        self.paths["b"].write_text(json.dumps(obj), encoding="utf-8")
        p = self.cli()
        self.assertEqual(p.returncode, 2)
        self.assertIn("coordinator_wasm_sha256:PIN_MISMATCH", p.stderr)

    def test_cli_never_treats_invalid_claim_as_go(self):
        obj = json.loads(self.paths["b"].read_text(encoding="utf-8"))
        obj["production_runtime_touched"] = True
        self.paths["b"].write_text(json.dumps(obj), encoding="utf-8")
        p = self.cli()
        self.assertEqual(p.returncode, 2)
        self.assertIn("PRODUCTION_BOUNDARY", p.stderr)
        self.assertNotIn("G53_GO_NO_GO=GO", p.stdout)
if __name__=="__main__":
    unittest.main()
