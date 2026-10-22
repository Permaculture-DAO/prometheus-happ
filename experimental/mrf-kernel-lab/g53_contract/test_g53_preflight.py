"""Fabricated host identities only; NO actual physical cross-host evidence."""
import copy
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
if __name__=="__main__":
    unittest.main()
