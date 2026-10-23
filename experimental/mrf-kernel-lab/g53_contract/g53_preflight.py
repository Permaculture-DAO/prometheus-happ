#!/usr/bin/env python3
"""Gate-5.3 read-only TEST manifest validator. Passing is NOT physical host proof."""
import argparse
import json
import re
import sys
from pathlib import Path
SCHEMA = "PRM-G53-PREFLIGHT-v1"
PIN = {
    "dna_sha256": "739d1fb6ffb1f61de14c187a27f926366ba9ed88fdd2057b72503c871ebc779f",
    "integrity_wasm_sha256": "633e905171d133a204c3bfca3e8ee15794c80676d22631c68c7d9f4dd9cb8581",
    "coordinator_wasm_sha256": "f490b2f89eb810de9913a5c8bd153402567032f2ba6c43f1109cf58c9064af7f",
    "upstream_lock_sha256": "294832bbea633e68b49fb2c97b841da214118f7672f78bd314600d1002c64464",
}
SHA = re.compile(r"^[0-9a-f]{64}$")
class PreflightRejected(ValueError):
    pass
def need(ok, reason):
    if not ok:
        raise PreflightRejected(reason)
def keys(o, fields, where):
    need(type(o) is dict and set(o) == set(fields), where + ":KEY_SET_MISMATCH")
def sha(v, name):
    need(type(v) is str and SHA.fullmatch(v) is not None, name + ":INVALID_SHA256")
def ref(v, name):
    need(type(v) is str and 4 <= len(v) <= 512 and not any(c in v for c in "\r\n\x00"), name + ":INVALID_REF")
def positive(v, name):
    need(type(v) is int and v > 0, name + ":NOT_POSITIVE_INT")
def validate_pair(policy, a, b):
    """Schema-only result: actual physical independence needs external witness."""
    keys(policy, ["schema", "approved_test_source_sha", "min_disk_free_bytes",
                  "min_ram_available_bytes", "review_authorization_ref"], "POLICY")
    need(policy["schema"] == SCHEMA, "POLICY:SCHEMA_MISMATCH")
    sha(policy["approved_test_source_sha"], "POLICY:SOURCE")
    positive(policy["min_disk_free_bytes"], "POLICY:MIN_DISK")
    positive(policy["min_ram_available_bytes"], "POLICY:MIN_RAM")
    ref(policy["review_authorization_ref"], "POLICY:REVIEW")
    fields = ["schema", "role", "test_source_sha", *PIN.keys(), "host_id_sha256",
              "agent_pubkey_sha256", "state_root_sha256", "failure_domain_ref",
              "observation_ref", "free_disk_bytes", "available_ram_bytes",
              "production_runtime_touched", "real_persistence_enabled"]
    for role, h in (("A", a), ("B", b)):
        w = "HOST_" + role
        keys(h, fields, w)
        need(h["schema"] == SCHEMA and h["role"] == role, w + ":SCHEMA_OR_ROLE")
        need(h["test_source_sha"] == policy["approved_test_source_sha"], w + ":SOURCE_MISMATCH")
        for k, expected in PIN.items():
            need(h[k] == expected, w + ":" + k + ":PIN_MISMATCH")
        for k in ("host_id_sha256", "agent_pubkey_sha256", "state_root_sha256"):
            sha(h[k], w + ":" + k)
        ref(h["failure_domain_ref"], w + ":DOMAIN")
        ref(h["observation_ref"], w + ":OBSERVATION")
        positive(h["free_disk_bytes"], w + ":DISK")
        positive(h["available_ram_bytes"], w + ":RAM")
        need(h["free_disk_bytes"] >= policy["min_disk_free_bytes"], w + ":INSUFFICIENT_DISK")
        need(h["available_ram_bytes"] >= policy["min_ram_available_bytes"], w + ":INSUFFICIENT_RAM")
        need(h["production_runtime_touched"] is False, w + ":PRODUCTION_BOUNDARY")
        need(h["real_persistence_enabled"] is False, w + ":REAL_GATE_OPEN")
    for k in ("host_id_sha256", "agent_pubkey_sha256", "state_root_sha256",
              "failure_domain_ref", "observation_ref"):
        need(a[k] != b[k], "PAIR:" + k + ":NOT_DISTINCT")
    return ("G53_MANIFEST_PAIR_SCHEMA=PASS\n"
            "G53_PHYSICAL_INDEPENDENCE=NOT_PROVEN\n"
            "G53_DHT_REPLICATION=NOT_TESTED\n"
            "G53_GO_NO_GO=HOLD_PENDING_INDEPENDENT_REVIEW")
def reject_duplicate_json_keys(pairs):
    """Reject ambiguously repeated names at ANY nesting depth."""
    obj = {}
    for name, value in pairs:
        if name in obj:
            raise PreflightRejected("INPUT:DUPLICATE_JSON_KEY:" + name)
        obj[name] = value
    return obj

def reject_non_finite(value):
    raise PreflightRejected("INPUT:NONFINITE_NUMBER:" + value)
def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--policy", type=Path, required=True)
    p.add_argument("--host-a", type=Path, required=True)
    p.add_argument("--host-b", type=Path, required=True)
    args = p.parse_args()
    try:
        def read(path):
            need(path.is_file() and path.stat().st_size <= 32768, "INPUT:MISSING_OR_TOO_LARGE")
            return json.loads(path.read_text(encoding="utf-8"),
                              object_pairs_hook=reject_duplicate_json_keys,
                              parse_constant=reject_non_finite)
        print(validate_pair(read(args.policy), read(args.host_a), read(args.host_b)))
    except (PreflightRejected, OSError, ValueError, UnicodeError) as exc:
        print("G53_MANIFEST_PAIR_SCHEMA=FAIL:" + str(exc), file=sys.stderr)
        sys.exit(2)
if __name__ == "__main__":
    main()
