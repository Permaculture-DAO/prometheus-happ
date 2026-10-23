"""Synthetic Ed25519 proof-of-possession only, NEVER real host credentials."""
import copy
import hashlib
import json
import subprocess
import sys
import tempfile
import time
import unittest
from pathlib import Path
from g53_attestation import SCHEMA, canonical, public_der, sha256, signed_message, validate_attestations
from g53_preflight import PreflightRejected
from test_g53_preflight import POLICY, host

class G53SignedAttestationTest(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.policy = copy.deepcopy(POLICY)
        self.a, self.b = host("A"), host("B")
        self.keys = {}
        self.pub = {}
        self.sig = {}
        for role in ("A", "B"):
            key = self.root / ("key-" + role + ".pem")
            pub = self.root / ("pub-" + role + ".pem")
            sig = self.root / ("sig-" + role)
            subprocess.run(["openssl", "genpkey", "-algorithm", "ED25519", "-out", str(key)],
                           capture_output=True, check=True)
            subprocess.run(["openssl", "pkey", "-in", str(key), "-pubout", "-out", str(pub)],
                           capture_output=True, check=True)
            self.keys[role], self.pub[role], self.sig[role] = key, pub, sig
        now = int(time.time())
        self.now = now
        self.challenge = {
            "schema": SCHEMA, "nonce_hex": "a7" * 32,
            "issued_at_epoch": now - 10,
            "expires_at_epoch": now + 120,
            "approved_test_source_sha": self.policy["approved_test_source_sha"],
            "host_a_pubkey_sha256": sha256(public_der(self.pub["A"])),
            "host_b_pubkey_sha256": sha256(public_der(self.pub["B"])),
        }
        self.sign()

    def sign(self):
        for role, h in (("A", self.a), ("B", self.b)):
            msg = self.root / ("message-" + role)
            msg.write_bytes(signed_message(self.challenge, role, h))
            subprocess.run(["openssl", "pkeyutl", "-sign", "-inkey",
                            str(self.keys[role]), "-rawin", "-in", str(msg),
                            "-out", str(self.sig[role])],
                           capture_output=True, check=True)

    def verify(self, *, now=None, digest=None):
        return validate_attestations(
            self.policy, self.a, self.b, self.challenge,
            pub_a=self.pub["A"], pub_b=self.pub["B"],
            sig_a=self.sig["A"], sig_b=self.sig["B"],
            approved_challenge_sha256=digest or sha256(canonical(self.challenge)),
            now_epoch=self.now if now is None else now)

    def rejects(self, reason, *, now=None, digest=None):
        with self.assertRaisesRegex(PreflightRejected, reason):
            self.verify(now=now, digest=digest)

    def test_valid_ephemeral_signatures_still_hold(self):
        v = self.verify()
        self.assertIn("G53_TEST_ED25519_MANIFEST_SIGNATURES=PASS", v)
        self.assertIn("G53_TRUSTED_KEY_ENROLLMENT=NOT_PROVEN", v)
        self.assertIn("G53_GO_NO_GO=HOLD", v)

    def test_mutated_host_after_signing_is_rejected(self):
        self.a["free_disk_bytes"] += 100
        self.rejects("SIGNATURE_OR_KEY_INVALID")

    def test_swapped_signatures_rejected(self):
        self.sig["A"], self.sig["B"] = self.sig["B"], self.sig["A"]
        self.rejects("SIGNATURE_OR_KEY_INVALID")

    def test_challenge_nonce_change_breaks_signature(self):
        self.challenge["nonce_hex"] = "b7" * 32
        self.rejects("SIGNATURE_OR_KEY_INVALID")

    def test_external_challenge_digest_pinning(self):
        self.rejects("CHALLENGE_DIGEST_MISMATCH", digest="0" * 64)

    def test_stale_challenge_rejected(self):
        self.rejects("CHALLENGE_EXPIRED_OR_FUTURE", now=self.now + 500)

    def test_future_challenge_rejected(self):
        self.rejects("CHALLENGE_EXPIRED_OR_FUTURE", now=self.now - 1000)

    def test_excessive_time_window_rejected(self):
        self.challenge["expires_at_epoch"] = self.now + 5000
        self.rejects("TIME_WINDOW")

    def test_swapped_public_keys_rejected(self):
        self.pub["A"], self.pub["B"] = self.pub["B"], self.pub["A"]
        self.rejects("UNTRUSTED_PUBLIC_KEY_A")

    def test_signature_corruption_rejected(self):
        self.sig["A"].write_bytes(b"x" * 64)
        self.rejects("SIGNATURE_OR_KEY_INVALID")

    def test_signature_truncation_rejected(self):
        self.sig["A"].write_bytes(b"x")
        self.rejects("INVALID_SIGNATURE_LENGTH_A")

    def test_missing_signature_rejected(self):
        self.sig["B"].unlink()
        self.rejects("MISSING_OR_TOO_LARGE")

    def test_extra_challenge_field_rejected(self):
        self.challenge["go_live"] = True
        self.rejects("KEY_SET_MISMATCH")

    def test_manifest_source_mismatch_rejected_before_signature(self):
        self.b["test_source_sha"] = "1" * 64
        self.rejects("SOURCE_MISMATCH")

    def test_duplicate_pubkey_pins_rejected(self):
        self.challenge["host_b_pubkey_sha256"] = self.challenge["host_a_pubkey_sha256"]
        self.rejects("DUPLICATE_SIGNER_KEY_PINS")

    def test_non_ed25519_key_rejected(self):
        rsa = self.root / "rsa-key.pem"
        pub = self.root / "rsa-pub.pem"
        subprocess.run(["openssl", "genpkey", "-algorithm", "RSA", "-pkeyopt",
                        "rsa_keygen_bits:2048", "-out", str(rsa)],
                       capture_output=True, check=True)
        subprocess.run(["openssl", "pkey", "-in", str(rsa), "-pubout", "-out", str(pub)],
                       capture_output=True, check=True)
        self.pub["A"] = pub
        self.rejects("ED25519_PUBLIC_KEY_REQUIRED")

    def test_real_boundary_cannot_be_signed_into_go(self):
        self.a["real_persistence_enabled"] = True
        self.sign()
        self.rejects("REAL_GATE_OPEN")

if __name__ == "__main__":
    unittest.main()
