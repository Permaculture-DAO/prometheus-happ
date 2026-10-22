#!/usr/bin/env python3
"""TEST-only signed reviewer assertion + supplied nonce-ledger audit. No real replay prevention."""
import argparse
import json
import re
import sys
import tempfile
from pathlib import Path

from g53_attestation import (
    canonical, json_file, openssl, public_der, sha256, signed_message,
    strict_file, validate_attestations,
)
from g53_preflight import PreflightRejected, keys, need, ref, sha

REGISTRY_SCHEMA = "PRM-G53-WITNESS-REGISTRY-v1"
SNAPSHOT_SCHEMA = "PRM-G53-NONCE-SNAPSHOT-v1"
NONCE = re.compile(r"^[0-9a-f]{64}$")

def review_message(registry):
    return b"PRM-G53-REVIEW-v1\x00" + bytes.fromhex(sha256(canonical(registry)))

def validate_witness_session(policy, a, b, challenge, *, pub_a, pub_b, sig_a, sig_b,
                             approved_challenge_sha256, registry, reviewer_pub,
                             reviewer_sig, reviewer_expected_pubkey_sha256,
                             nonce_snapshot, now_epoch=None):
    # This checks only signatures under provided public keys, never trust enrollment.
    host_result = validate_attestations(
        policy, a, b, challenge, pub_a=pub_a, pub_b=pub_b,
        sig_a=sig_a, sig_b=sig_b,
        approved_challenge_sha256=approved_challenge_sha256,
        now_epoch=now_epoch)
    keys(registry, ["schema", "challenge_sha256", "test_source_sha",
                    "review_authorization_ref", "host_a_pubkey_sha256",
                    "host_b_pubkey_sha256", "reviewer_pubkey_spki_sha256"], "REVIEW")
    need(registry["schema"] == REGISTRY_SCHEMA, "REVIEW:SCHEMA")
    need(registry["challenge_sha256"] == approved_challenge_sha256,
         "REVIEW:CHALLENGE_MISMATCH")
    need(registry["test_source_sha"] == policy["approved_test_source_sha"],
         "REVIEW:SOURCE_MISMATCH")
    need(registry["review_authorization_ref"] == policy["review_authorization_ref"],
         "REVIEW:AUTHORIZATION_REF_MISMATCH")
    for role in ("a", "b"):
        name = "host_" + role + "_pubkey_sha256"
        sha(registry[name], "REVIEW:" + name)
        need(registry[name] == challenge[name], "REVIEW:HOST_KEY_MISMATCH_" + role.upper())
    sha(registry["reviewer_pubkey_spki_sha256"], "REVIEW:REVIEWER_KEY")
    sha(reviewer_expected_pubkey_sha256, "REVIEW:EXTERNAL_PIN")
    reviewer_der = public_der(reviewer_pub)
    digest = sha256(reviewer_der)
    need(digest == registry["reviewer_pubkey_spki_sha256"]
         and digest == reviewer_expected_pubkey_sha256, "REVIEW:REVIEWER_PIN_MISMATCH")
    need(digest not in (registry["host_a_pubkey_sha256"],
                        registry["host_b_pubkey_sha256"]), "REVIEW:SAME_AS_HOST_SIGNER")
    signature = strict_file(reviewer_sig, 128)
    need(len(signature) == 64, "REVIEW:INVALID_SIGNATURE_LENGTH")
    with tempfile.TemporaryDirectory(prefix="g53-review-") as tmp:
        message = Path(tmp) / "review-message"
        message.write_bytes(review_message(registry))
        openssl(["pkeyutl", "-verify", "-pubin", "-inkey", str(reviewer_pub),
                 "-sigfile", str(reviewer_sig), "-rawin", "-in", str(message)])
    keys(nonce_snapshot, ["schema", "prior_consumptions"], "NONCE_SNAPSHOT")
    need(nonce_snapshot["schema"] == SNAPSHOT_SCHEMA, "NONCE_SNAPSHOT:SCHEMA")
    entries = nonce_snapshot["prior_consumptions"]
    need(type(entries) is list and len(entries) <= 128, "NONCE_SNAPSHOT:ENTRY_LIMIT")
    seen_nonce, seen_challenge = set(), set()
    for i, entry in enumerate(entries):
        where = "NONCE_SNAPSHOT:" + str(i)
        keys(entry, ["nonce_hex", "challenge_sha256"], where)
        nonce = entry["nonce_hex"]
        need(type(nonce) is str and NONCE.fullmatch(nonce) and nonce != "0" * 64,
             where + ":INVALID_NONCE")
        sha(entry["challenge_sha256"], where + ":CHALLENGE")
        need(nonce not in seen_nonce, where + ":DUPLICATE_NONCE")
        need(entry["challenge_sha256"] not in seen_challenge,
             where + ":DUPLICATE_CHALLENGE")
        seen_nonce.add(nonce)
        seen_challenge.add(entry["challenge_sha256"])
    need(challenge["nonce_hex"] not in seen_nonce, "NONCE_SNAPSHOT:NONCE_ALREADY_RECORDED")
    need(approved_challenge_sha256 not in seen_challenge,
         "NONCE_SNAPSHOT:CHALLENGE_ALREADY_RECORDED")
    return ("\n".join([
        host_result,
        "G53_TEST_REVIEWER_ED25519_SIGNATURE=PASS",
        "G53_NONCE_UNUSED_IN_SUPPLIED_SNAPSHOT=PASS",
        "G53_REVIEWER_KEY_ENROLLMENT=NOT_PROVEN",
        "G53_NONCE_LOG_COMPLETENESS=NOT_PROVEN",
        "G53_ATOMIC_NONCE_CONSUMPTION=NOT_PROVEN",
        "G53_PHYSICAL_INDEPENDENCE=NOT_PROVEN",
        "G53_GO_NO_GO=HOLD_PENDING_INDEPENDENT_REVIEW",
    ]))

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for arg in ("policy", "host-a", "host-b", "challenge",
                "pub-a", "pub-b", "sig-a", "sig-b",
                "registry", "reviewer-pub", "reviewer-sig", "nonce-snapshot"):
        parser.add_argument("--" + arg, type=Path, required=True)
    parser.add_argument("--approved-challenge-sha256", required=True)
    parser.add_argument("--reviewer-expected-pubkey-sha256", required=True)
    opts = parser.parse_args()
    try:
        result = validate_witness_session(
            json_file(opts.policy), json_file(opts.host_a), json_file(opts.host_b),
            json_file(opts.challenge),
            pub_a=opts.pub_a, pub_b=opts.pub_b, sig_a=opts.sig_a, sig_b=opts.sig_b,
            approved_challenge_sha256=opts.approved_challenge_sha256,
            registry=json_file(opts.registry),
            reviewer_pub=opts.reviewer_pub, reviewer_sig=opts.reviewer_sig,
            reviewer_expected_pubkey_sha256=opts.reviewer_expected_pubkey_sha256,
            nonce_snapshot=json_file(opts.nonce_snapshot))
        print(result)
        return 0
    except (PreflightRejected, OSError, ValueError, UnicodeError, TypeError) as exc:
        print("G53_TEST_REVIEWER_ED25519_SIGNATURE=FAIL:" + str(exc), file=sys.stderr)
        return 2

if __name__ == "__main__":
    sys.exit(main())
