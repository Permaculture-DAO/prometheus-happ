"""Fictional, ephemeral CI-only signing keys and nonce ledger snapshots."""
import copy
import json
import subprocess
import tempfile
import time
import unittest
from pathlib import Path

from g53_attestation import SCHEMA, canonical, public_der, sha256, signed_message
from g53_preflight import PreflightRejected
from g53_witness_audit import (REGISTRY_SCHEMA, SNAPSHOT_SCHEMA,
                               review_message, validate_witness_session)
from test_g53_preflight import POLICY, host

class WitnessAuditTest(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.policy = copy.deepcopy(POLICY)
        self.a, self.b = host("A"), host("B")
        self.private, self.public, self.sig = {}, {}, {}
        for role in ("A", "B", "REVIEWER"):
            key = self.root / (role + ".key.pem")
            pub = self.root / (role + ".pub.pem")
            sig = self.root / (role + ".sig")
            subprocess.run(["openssl", "genpkey", "-algorithm", "ED25519",
                            "-out", str(key)], check=True, capture_output=True)
            subprocess.run(["openssl", "pkey", "-in", str(key), "-pubout",
                            "-out", str(pub)], check=True, capture_output=True)
            self.private[role], self.public[role], self.sig[role] = key, pub, sig
        self.now = int(time.time())
        self.challenge = dict(schema=SCHEMA, nonce_hex="e3" * 32,
                              issued_at_epoch=self.now - 20,
                              expires_at_epoch=self.now + 120,
                              approved_test_source_sha=self.policy["approved_test_source_sha"],
                              host_a_pubkey_sha256=sha256(public_der(self.public["A"])),
                              host_b_pubkey_sha256=sha256(public_der(self.public["B"])))
        self.approved_digest = sha256(canonical(self.challenge))
        self.review_pin = sha256(public_der(self.public["REVIEWER"]))
        self.registry = dict(
            schema=REGISTRY_SCHEMA, challenge_sha256=self.approved_digest,
            test_source_sha=self.policy["approved_test_source_sha"],
            review_authorization_ref=self.policy["review_authorization_ref"],
            host_a_pubkey_sha256=self.challenge["host_a_pubkey_sha256"],
            host_b_pubkey_sha256=self.challenge["host_b_pubkey_sha256"],
            reviewer_pubkey_spki_sha256=self.review_pin)
        self.snapshot = dict(schema=SNAPSHOT_SCHEMA, prior_consumptions=[])
        self.sign()

    def sign_one(self, role, content):
        msg = self.root / (role + ".input")
        msg.write_bytes(content)
        subprocess.run(["openssl", "pkeyutl", "-sign", "-inkey",
                        str(self.private[role]), "-rawin", "-in", str(msg),
                        "-out", str(self.sig[role])], check=True, capture_output=True)
    def sign(self):
        self.sign_one("A", signed_message(self.challenge, "A", self.a))
        self.sign_one("B", signed_message(self.challenge, "B", self.b))
        self.sign_one("REVIEWER", review_message(self.registry))
    def run_check(self):
        return validate_witness_session(
            self.policy, self.a, self.b, self.challenge,
            pub_a=self.public["A"], pub_b=self.public["B"],
            sig_a=self.sig["A"], sig_b=self.sig["B"],
            approved_challenge_sha256=self.approved_digest,
            registry=self.registry, reviewer_pub=self.public["REVIEWER"],
            reviewer_sig=self.sig["REVIEWER"],
            reviewer_expected_pubkey_sha256=self.review_pin,
            nonce_snapshot=self.snapshot, now_epoch=self.now)
    def rejects(self, reason):
        with self.assertRaisesRegex(PreflightRejected, reason):
            self.run_check()
    def test_fabricated_signature_and_empty_snapshot_still_hold(self):
        result = self.run_check()
        self.assertIn("G53_TEST_REVIEWER_ED25519_SIGNATURE=PASS", result)
        self.assertIn("G53_ATOMIC_NONCE_CONSUMPTION=NOT_PROVEN", result)
        self.assertIn("G53_GO_NO_GO=HOLD", result)
    def test_known_nonce_replay_rejected(self):
        self.snapshot["prior_consumptions"].append(dict(
            nonce_hex=self.challenge["nonce_hex"], challenge_sha256="1" * 64))
        self.rejects("NONCE_ALREADY_RECORDED")
    def test_known_challenge_replay_rejected(self):
        self.snapshot["prior_consumptions"].append(dict(
            nonce_hex="a1" * 32, challenge_sha256=self.approved_digest))
        self.rejects("CHALLENGE_ALREADY_RECORDED")
    def test_duplicate_prior_nonce_rejected(self):
        self.snapshot["prior_consumptions"] = [
            dict(nonce_hex="b1" * 32, challenge_sha256="1" * 64),
            dict(nonce_hex="b1" * 32, challenge_sha256="2" * 64)]
        self.rejects("DUPLICATE_NONCE")
    def test_duplicate_prior_challenge_rejected(self):
        self.snapshot["prior_consumptions"] = [
            dict(nonce_hex="b1" * 32, challenge_sha256="1" * 64),
            dict(nonce_hex="b2" * 32, challenge_sha256="1" * 64)]
        self.rejects("DUPLICATE_CHALLENGE")
    def test_invalid_entry_rejected(self):
        self.snapshot["prior_consumptions"] = [dict(nonce_hex="x", challenge_sha256="1" * 64)]
        self.rejects("INVALID_NONCE")
    def test_large_snapshot_rejected(self):
        self.snapshot["prior_consumptions"] = [None] * 129
        self.rejects("ENTRY_LIMIT")
    def test_registry_challenge_mismatch_rejected(self):
        self.registry["challenge_sha256"] = "1" * 64
        self.rejects("CHALLENGE_MISMATCH")
    def test_registry_source_mismatch_rejected(self):
        self.registry["test_source_sha"] = "1" * 64
        self.rejects("SOURCE_MISMATCH")
    def test_registry_ref_mismatch_rejected(self):
        self.registry["review_authorization_ref"] = "TEST-wrong-authorization"
        self.rejects("AUTHORIZATION_REF_MISMATCH")
    def test_registry_host_key_substitution_rejected(self):
        self.registry["host_a_pubkey_sha256"] = "1" * 64
        self.rejects("HOST_KEY_MISMATCH_A")
    def test_reviewer_registry_tampering_rejected(self):
        self.registry["review_authorization_ref"] += "-different"
        self.registry["review_authorization_ref"] = self.policy["review_authorization_ref"]
        self.registry["extra"] = True
        self.rejects("KEY_SET_MISMATCH")
    def test_review_signature_corruption_rejected(self):
        self.sig["REVIEWER"].write_bytes(b"x" * 64)
        self.rejects("SIGNATURE_OR_KEY_INVALID")
    def test_reviewer_external_pin_mismatch_rejected(self):
        self.review_pin = "0" * 64
        self.rejects("REVIEWER_PIN_MISMATCH")
    def test_reviewer_same_as_host_signer_rejected(self):
        self.public["REVIEWER"] = self.public["A"]
        self.review_pin = sha256(public_der(self.public["A"]))
        self.registry["reviewer_pubkey_spki_sha256"] = self.review_pin
        self.rejects("SAME_AS_HOST_SIGNER")
    def test_registry_change_requires_new_review_signature(self):
        self.registry["review_authorization_ref"] = "TEST-updated-authorization"
        self.policy["review_authorization_ref"] = "TEST-updated-authorization"
        self.rejects("SIGNATURE_OR_KEY_INVALID")
    def test_open_real_gate_stays_rejected(self):
        self.a["real_persistence_enabled"] = True
        self.rejects("REAL_GATE_OPEN")

if __name__ == "__main__":
    unittest.main()
