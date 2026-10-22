#!/usr/bin/env python3
"""G5.3 TEST-only signed manifest challenge. Signatures are NOT physical host proof."""
import argparse
import hashlib
import json
import re
import subprocess
import sys
import tempfile
import time
from pathlib import Path
from g53_preflight import (
    PreflightRejected, keys, need, sha, validate_pair,
    reject_duplicate_json_keys, reject_non_finite,
)

SCHEMA = "PRM-G53-ATTESTATION-CHALLENGE-v1"
ED25519_SPKI_PREFIX = bytes.fromhex("302a300506032b6570032100")
MAX_VALIDITY_SECONDS = 1800

def canonical(value):
    return json.dumps(value, sort_keys=True, ensure_ascii=False,
                      separators=(",", ":"), allow_nan=False).encode("utf-8")

def sha256(data):
    return hashlib.sha256(data).hexdigest()

def signed_message(challenge, role, host):
    """Domain-separated, deterministic message. Binds the entire challenge and host."""
    need(role in ("A", "B"), "ATTEST:INVALID_ROLE")
    return (b"PRM-G53-HOST-ATTEST-v1\x00"
            + hashlib.sha256(canonical(challenge)).digest()
            + role.encode("ascii") + b"\x00"
            + hashlib.sha256(canonical(host)).digest())

def strict_file(path, max_bytes=32768):
    need(path.is_file() and path.stat().st_size <= max_bytes, "ATTEST:MISSING_OR_TOO_LARGE")
    return path.read_bytes()

def json_file(path):
    return json.loads(strict_file(path).decode("utf-8"),
                      object_pairs_hook=reject_duplicate_json_keys,
                      parse_constant=reject_non_finite)

def openssl(args, input_data=None):
    try:
        p = subprocess.run(["openssl", *args], input=input_data,
                           capture_output=True, check=False, timeout=6)
    except (OSError, subprocess.TimeoutExpired) as exc:
        raise PreflightRejected("ATTEST:OPENSSL_UNAVAILABLE") from exc
    need(p.returncode == 0, "ATTEST:SIGNATURE_OR_KEY_INVALID")
    return p.stdout

def public_der(path):
    pem = strict_file(path, 4096)
    need(b"-----BEGIN PUBLIC KEY-----" in pem and b"PRIVATE KEY" not in pem,
         "ATTEST:PUBLIC_KEY_REQUIRED")
    der = openssl(["pkey", "-pubin", "-in", str(path), "-outform", "DER"])
    need(len(der) == 44 and der.startswith(ED25519_SPKI_PREFIX),
         "ATTEST:ED25519_PUBLIC_KEY_REQUIRED")
    return der

def validate_attestations(policy, a, b, challenge, *,
                          pub_a, pub_b, sig_a, sig_b, approved_challenge_sha256,
                          now_epoch=None):
    """Check claimed, signed TEST statements; never infer physical independence."""
    validate_pair(policy, a, b)
    keys(challenge, ["schema", "nonce_hex", "issued_at_epoch", "expires_at_epoch",
                     "approved_test_source_sha", "host_a_pubkey_sha256",
                     "host_b_pubkey_sha256"], "CHALLENGE")
    need(challenge["schema"] == SCHEMA, "ATTEST:CHALLENGE_SCHEMA")
    nonce = challenge["nonce_hex"]
    need(type(nonce) is str and re.fullmatch(r"[0-9a-f]{64}", nonce) is not None
         and nonce != "0" * 64, "ATTEST:CHALLENGE_NONCE")
    sha(approved_challenge_sha256, "ATTEST:EXTERNALLY_APPROVED_CHALLENGE")
    need(sha256(canonical(challenge)) == approved_challenge_sha256,
         "ATTEST:CHALLENGE_DIGEST_MISMATCH")
    need(challenge["approved_test_source_sha"] == policy["approved_test_source_sha"],
         "ATTEST:SOURCE_MISMATCH")
    issued = challenge["issued_at_epoch"]
    expires = challenge["expires_at_epoch"]
    need(type(issued) is int and type(expires) is int and issued > 0,
         "ATTEST:TIME_TYPE")
    need(0 < expires - issued <= MAX_VALIDITY_SECONDS, "ATTEST:TIME_WINDOW")
    now = int(time.time()) if now_epoch is None else now_epoch
    need(type(now) is int and issued <= now <= expires, "ATTEST:CHALLENGE_EXPIRED_OR_FUTURE")
    digests = []
    for role, host, pub, sig in (("A", a, pub_a, sig_a), ("B", b, pub_b, sig_b)):
        der = public_der(pub)
        expected = challenge["host_" + role.lower() + "_pubkey_sha256"]
        sha(expected, "ATTEST:PUBLIC_KEY_DIGEST")
        digest = sha256(der)
        need(digest == expected, "ATTEST:UNTRUSTED_PUBLIC_KEY_" + role)
        digests.append(digest)
        signature = strict_file(sig, 128)
        need(len(signature) == 64, "ATTEST:INVALID_SIGNATURE_LENGTH_" + role)
        with tempfile.TemporaryDirectory(prefix="g53-attest-") as tmp:
            msg = Path(tmp) / "signed-message"
            msg.write_bytes(signed_message(challenge, role, host))
            openssl(["pkeyutl", "-verify", "-pubin", "-inkey", str(pub),
                     "-sigfile", str(sig), "-rawin", "-in", str(msg)])
    need(digests[0] != digests[1], "ATTEST:NON_INDEPENDENT_SIGNING_KEYS")
    return ("\n".join([
        "G53_TEST_ED25519_MANIFEST_SIGNATURES=PASS",
        "G53_CHALLENGE_DIGEST_MATCH=PASS",
        "G53_CHALLENGE_FRESH_AT_VERIFICATION=PASS",
        "G53_TRUSTED_KEY_ENROLLMENT=NOT_PROVEN",
        "G53_NONCE_NONREUSE=NOT_PROVEN",
        "G53_PHYSICAL_INDEPENDENCE=NOT_PROVEN",
        "G53_DHT_REPLICATION=NOT_TESTED",
        "G53_GO_NO_GO=HOLD_PENDING_INDEPENDENT_REVIEW",
    ]))

def main():
    p = argparse.ArgumentParser(description=__doc__)
    for name in ("policy", "host-a", "host-b", "challenge", "pub-a", "pub-b", "sig-a", "sig-b"):
        p.add_argument("--" + name, type=Path, required=True)
    p.add_argument("--approved-challenge-sha256", required=True)
    args = p.parse_args()
    try:
        print(validate_attestations(
            json_file(args.policy), json_file(args.host_a), json_file(args.host_b),
            json_file(args.challenge),
            pub_a=args.pub_a, pub_b=args.pub_b,
            sig_a=args.sig_a, sig_b=args.sig_b,
            approved_challenge_sha256=args.approved_challenge_sha256))
    except (PreflightRejected, OSError, ValueError, UnicodeError, TypeError) as exc:
        print("G53_TEST_ED25519_MANIFEST_SIGNATURES=FAIL:" + str(exc), file=sys.stderr)
        return 2
    return 0

if __name__ == "__main__":
    sys.exit(main())
