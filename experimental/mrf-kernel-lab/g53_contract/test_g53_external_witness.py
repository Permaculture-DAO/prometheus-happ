"""G5.3 synthetic witness commitments; never attest actual external custody."""
import copy
import shutil
import sqlite3
import subprocess
import unittest
from pathlib import Path

from g53_attestation import canonical, public_der, sha256
from g53_external_witness import (
    GENESIS, SCHEMA, history_hash, witness_head, witness_message,
    verify_witnessed_history,
)
from g53_preflight import PreflightRejected
from g53_root_enrollment import enrollment_head
from test_g53_root_rotation import RootHandoverTest, SOURCE


class ExternalWitnessTest(RootHandoverTest):
    def setUp(self):
        super().setUp()
        self.witness_keys, self.witness_pubs = {}, {}
        for role in ("INDEPENDENT_CUSTODIAN_TEST_1", "INDEPENDENT_CUSTODIAN_TEST_2"):
            priv = self.root / (role + ".private.pem")
            pub = self.root / (role + ".public.pem")
            subprocess.run(["openssl", "genpkey", "-algorithm", "ED25519",
                            "-out", str(priv)], capture_output=True, check=True)
            subprocess.run(["openssl", "pkey", "-in", str(priv), "-pubout",
                            "-out", str(pub)], capture_output=True, check=True)
            fingerprint = sha256(public_der(pub))
            self.witness_keys[fingerprint] = priv
            self.witness_pubs[fingerprint] = pub
        self.witness_pins = sorted(self.witness_keys)
        self.statement = {
            "schema": SCHEMA,
            "witness_round": 1,
            "previous_witness_head_sha256": GENESIS,
            "latest_root_anchor_sha256": enrollment_head(self.epochs[-1]["anchor"]),
            "latest_root_epoch": len(self.epochs),
            "genesis_root_fingerprints_sha256": sha256(canonical(self.old_root_set)),
            "latest_root_fingerprints_sha256": sha256(canonical(self.new_root_set)),
            "history_bundle_sha256": history_hash(self.epochs, self.rotations),
            "source_sha256": SOURCE,
            "authorization_ref": self.policy["review_authorization_ref"],
            "session_nonce_hex": "a3" * 32,
            "issued_at_epoch": self.now - 10,
            "expires_at_epoch": self.now + 120,
        }
        self.sign_receipt()

    def sign_receipt(self):
        endorsements = []
        for fingerprint in self.witness_pins:
            msg = self.root / "external-witness-msg.test"
            sig = self.root / "external-witness-sig.test"
            msg.write_bytes(witness_message(self.statement))
            subprocess.run(["openssl", "pkeyutl", "-sign", "-inkey",
                            str(self.witness_keys[fingerprint]), "-rawin",
                            "-in", str(msg), "-out", str(sig)],
                           capture_output=True, check=True)
            endorsements.append({"witness_pubkey_spki_sha256": fingerprint,
                                 "signature_hex": sig.read_bytes().hex()})
        self.receipt = {"statement": self.statement, "endorsements": endorsements}
        self.external_witness_head = witness_head(self.receipt)

    def verify(self, **overrides):
        args = {
            "root_public_pems": self.root_pems,
            "reviewer_public_pems": self.review_pubs,
            "witness_public_pems": self.witness_pubs,
            "externally_pinned_genesis_root_fingerprints": self.old_root_set,
            "externally_pinned_latest_root_fingerprints": self.new_root_set,
            "externally_pinned_witness_fingerprints": self.witness_pins,
            "externally_pinned_latest_witness_head_sha256": self.external_witness_head,
            "externally_pinned_previous_witness_head_sha256": GENESIS,
            "externally_pinned_latest_witness_round": 1,
            "independently_approved_source_sha256": SOURCE,
            "independently_approved_authorization_ref":
                self.policy["review_authorization_ref"],
            "now_epoch": self.now,
        }
        args.update(overrides)
        return verify_witnessed_history(self.db_path, self.epochs,
                                        self.rotations, self.receipt, **args)

    def rejects(self, marker, **overrides):
        with self.assertRaisesRegex(PreflightRejected, marker):
            self.verify(**overrides)

    def test_witness_two_synthetic_signatures_still_hold(self):
        out = self.verify()
        self.assertIn("G53_TEST_TWO_WITNESS_HISTORY_COMMITMENTS=PASS", out)
        self.assertIn("G53_WITNESS_OPERATOR_INDEPENDENCE=NOT_PROVEN", out)
        self.assertIn("G53_GO_NO_GO=HOLD", out)

    def test_witness_second_round_with_non_genesis_predecessor(self):
        self.statement["witness_round"] = 2
        self.statement["previous_witness_head_sha256"] = "e" * 64
        self.sign_receipt()
        msg = self.verify(
            externally_pinned_latest_witness_round=2,
            externally_pinned_previous_witness_head_sha256="e" * 64)
        self.assertIn("G53_TEST_WITNESS_FRESHNESS_AND_PIN_MATCH=PASS", msg)

    def test_witness_old_receipt_against_newer_round_rejected(self):
        self.rejects("STALE_OR_FUTURE_ROUND",
                     externally_pinned_latest_witness_round=2)

    def test_witness_old_receipt_against_new_latest_head_rejected(self):
        self.rejects("EXTERNAL_HEAD_MISMATCH",
                     externally_pinned_latest_witness_head_sha256="f" * 64)

    def test_witness_previous_head_mismatch(self):
        self.rejects("PREDECESSOR_NOT_PINNED",
                     externally_pinned_previous_witness_head_sha256="b" * 64)

    def test_witness_round_two_claims_genesis_predecessor(self):
        self.statement["witness_round"] = 2
        self.sign_receipt()
        self.rejects("GENESIS_LINK",
                     externally_pinned_latest_witness_round=2)

    def test_witness_signature_corrupted_with_recalculated_head(self):
        self.receipt["endorsements"][0]["signature_hex"] = "0" * 128
        self.external_witness_head = witness_head(self.receipt)
        self.rejects("SIGNATURE_OR_KEY_INVALID")

    def test_witness_missing_second_signature(self):
        self.receipt["endorsements"] = self.receipt["endorsements"][:1]
        self.external_witness_head = witness_head(self.receipt)
        self.rejects("MISSING_WITNESS_ENDORSEMENT")

    def test_witness_duplicate_signature(self):
        self.receipt["endorsements"][1] = copy.deepcopy(self.receipt["endorsements"][0])
        self.external_witness_head = witness_head(self.receipt)
        self.rejects("MISSING_DUPLICATE_OR_UNSORTED_SIGNERS")

    def test_witness_second_public_key_substituted(self):
        pubs = dict(self.witness_pubs)
        pubs[self.witness_pins[1]] = self.witness_pubs[self.witness_pins[0]]
        self.rejects("KEY_SUBSTITUTION", witness_public_pems=pubs)

    def test_witness_root_key_as_external_witness_rejected(self):
        self.rejects("WITNESS_ROLE_COLLISION",
                     externally_pinned_witness_fingerprints=sorted(
                         [self.old_root_set[0], self.witness_pins[0]]),
                     witness_public_pems={
                         self.old_root_set[0]: self.root_pems[self.old_root_set[0]],
                         self.witness_pins[0]: self.witness_pubs[self.witness_pins[0]],
                     })

    def test_witness_unapproved_source_claim(self):
        self.statement["source_sha256"] = "f" * 64
        self.sign_receipt()
        self.rejects("SOURCE_MISMATCH")

    def test_witness_unapproved_authorization_claim(self):
        self.statement["authorization_ref"] = "TEST-mismatched-ref"
        self.sign_receipt()
        self.rejects("AUTHORIZATION_MISMATCH")

    def test_witness_bundle_tampered_without_resigning_receipt(self):
        self.epochs[0]["records"][0]["endorsements"][0]["signature_hex"] = "0" * 128
        self.rejects("HISTORY_BUNDLE_MISMATCH")

    def test_witness_rewritten_bundle_and_resigned_cannot_use_old_pin(self):
        old_pin = self.external_witness_head
        self.epochs[0]["records"][0]["endorsements"][0]["signature_hex"] = "0" * 128
        self.statement["history_bundle_sha256"] = history_hash(
            self.epochs, self.rotations)
        self.sign_receipt()
        self.rejects("EXTERNAL_HEAD_MISMATCH",
                     externally_pinned_latest_witness_head_sha256=old_pin)

    def test_witness_missing_history_epoch(self):
        self.epochs.pop()
        self.rejects("EPOCH_COUNT_MISMATCH")

    def test_witness_changed_root_history_head(self):
        self.statement["latest_root_anchor_sha256"] = "e" * 64
        self.sign_receipt()
        self.rejects("ROOT_HEAD_MISMATCH")

    def test_witness_substituted_latest_root_key_claim(self):
        self.statement["latest_root_fingerprints_sha256"] = "e" * 64
        self.sign_receipt()
        self.rejects("LATEST_ROOT_SET_MISMATCH")

    def test_witness_invalid_nonce(self):
        self.statement["session_nonce_hex"] = "0" * 64
        self.sign_receipt()
        self.rejects("NONCE_INVALID")

    def test_witness_stale_session(self):
        self.rejects("EXPIRED_OR_FUTURE", now_epoch=self.now + 1000)

    def test_witness_future_session(self):
        self.rejects("EXPIRED_OR_FUTURE", now_epoch=self.now - 1000)

    def test_witness_too_long_session_window(self):
        self.statement["expires_at_epoch"] = self.now + 4000
        self.sign_receipt()
        self.rejects("TIME_WINDOW")

    def test_witness_signed_unknown_field_is_rejected(self):
        self.statement["allow_real_persistence"] = True
        self.sign_receipt()
        self.rejects("KEY_SET_MISMATCH")

    def test_witness_missing_sqlite_database(self):
        self.db_path.unlink()
        self.rejects("STORE_MISSING")

    def test_witness_final_database_content_rollback(self):
        with sqlite3.connect(str(self.db_path)) as conn:
            conn.execute("DROP TRIGGER test_nonce_claims_no_delete")
            conn.execute("DELETE FROM test_nonce_claims WHERE nonce_hex = ?", ("d4"*32,))
        self.rejects("SCHEMA_GUARDS_MISSING")

    def test_witness_noninteger_round(self):
        self.statement["witness_round"] = True
        self.sign_receipt()
        self.rejects("STALE_OR_FUTURE_ROUND")

    def test_witness_mismatched_external_key_set(self):
        self.rejects("KEY_SET_MISMATCH",
                     externally_pinned_witness_fingerprints=[
                         self.witness_pins[0], "f"*64])

    def test_witness_forged_witness_identity_claim(self):
        self.receipt["endorsements"][0]["witness_pubkey_spki_sha256"] = "f" * 64
        self.external_witness_head = witness_head(self.receipt)
        self.rejects("UNAUTHORIZED_SIGNER")


def load_tests(loader, original_tests, pattern):
    suite = unittest.TestSuite()
    for name in sorted(k for k in ExternalWitnessTest.__dict__
                       if k.startswith("test_witness_")):
        suite.addTest(ExternalWitnessTest(name))
    return suite


if __name__ == "__main__":
    unittest.main()
