"""Only fabricated TEST checkpoint witness signatures; no real enrollment."""
import shutil
import sqlite3
import subprocess
import time
import unittest
from pathlib import Path

from g53_attestation import canonical, public_der, sha256
from g53_checkpoint_witness import (
    GENESIS, SCHEMA, witness_head_digest, witness_message, verify_signed_checkpoint,
)
from g53_preflight import PreflightRejected
from g53_nonce_store import DDL
from test_g53_ledger_checkpoint import LedgerCheckpointTest, SOURCE


class SignedCheckpointWitnessTest(LedgerCheckpointTest):
    def setUp(self):
        super().setUp()
        self.statement = None
        self.head = None
        self.witness = "REVIEWER"  # exclusively fabricated ephemeral fixture
        self.witness_pin = sha256(public_der(self.public[self.witness]))
        self.external_sequence = 1
        self.external_previous = GENESIS

    def create_statement(self, seq=1, prev=GENESIS):
        self.statement = dict(
            schema=SCHEMA,
            sequence=seq,
            previous_head_sha256=prev,
            checkpoint=self.checkpoint(),
            approved_test_source_sha256=SOURCE,
            issued_at_epoch=self.now,
            witness_pubkey_spki_sha256=self.witness_pin,
            authorization_ref="TEST-fictional-approved-witness",
        )
        self.sign_statement()
        return self.statement

    def sign_statement(self):
        self.sign_one(self.witness, witness_message(self.statement))
        self.head = witness_head_digest(self.statement)

    def verify(self, **overrides):
        values = {
            "witness_pub": self.public[self.witness],
            "witness_sig": self.sig[self.witness],
            "externally_pinned_witness_pubkey_sha256": self.witness_pin,
            "externally_pinned_latest_head_sha256": self.head,
            "externally_pinned_latest_sequence": self.external_sequence,
            "externally_pinned_previous_head_sha256": self.external_previous,
            "approved_test_source_sha256": SOURCE,
            "now_epoch": self.now,
        }
        values.update(overrides)
        return verify_signed_checkpoint(self.db_path, self.statement, **values)

    def rejects(self, code, **kwargs):
        with self.assertRaisesRegex(PreflightRejected, code):
            self.verify(**kwargs)

    def test_witness_signed_genesis_checkpoint_keeps_hold(self):
        self.consume()
        self.create_statement()
        output = self.verify()
        self.assertIn("G53_TEST_SIGNED_CHECKPOINT_WITNESS=PASS", output)
        self.assertIn("G53_AUTHENTIC_OUT_OF_BAND_LATEST_HEAD=NOT_PROVEN", output)
        self.assertIn("G53_GO_NO_GO=HOLD", output)

    def test_witness_sequential_head_chain_with_second_claim(self):
        self.consume()
        self.create_statement()
        earlier_head = self.head
        self.second_claim()
        self.create_statement(seq=2, prev=earlier_head)
        self.external_sequence = 2
        self.external_previous = earlier_head
        self.assertIn("G53_TEST_PINNED_SEQUENCE_AND_PREDECESSOR=PASS", self.verify())

    def test_witness_replay_old_signed_attestation_rejected_by_later_sequence(self):
        self.consume()
        self.create_statement()
        older_statement = dict(self.statement)
        old_head = self.head
        self.second_claim()
        self.create_statement(seq=2, prev=old_head)
        next_head = self.head
        self.external_sequence = 2
        self.external_previous = old_head
        self.statement = older_statement
        self.head = next_head
        self.rejects("STALE_OR_FUTURE_SEQUENCE")

    def test_witness_previous_head_mismatch_rejected(self):
        self.consume()
        self.create_statement()
        self.external_previous = "a" * 64
        self.rejects("PREDECESSOR_NOT_PINNED")

    def test_witness_forged_latest_digest_rejected(self):
        self.consume()
        self.create_statement()
        self.rejects("LATEST_EXTERNAL_HEAD_MISMATCH",
                     externally_pinned_latest_head_sha256="b" * 64)

    def test_witness_modified_signed_checkpoint_rejected_even_if_rehashed(self):
        self.consume()
        self.create_statement()
        self.statement["checkpoint"] = dict(self.statement["checkpoint"], row_count=0)
        new_digest = witness_head_digest(self.statement)
        self.rejects("SIGNATURE_OR_KEY_INVALID",
                     externally_pinned_latest_head_sha256=new_digest)

    def test_witness_rollback_of_test_ledger_rejected(self):
        self.consume()
        backup = self.root / "old-db.sqlite"
        shutil.copy2(self.db_path, backup)
        self.second_claim()
        self.create_statement()
        self.assertIn("G53_TEST_SIGNED_CHECKPOINT_WITNESS=PASS", self.verify())
        shutil.copy2(backup, self.db_path)
        self.rejects("ROLLBACK_OR_COUNT_CHANGE")

    def test_witness_db_row_mutation_same_count_rejected(self):
        self.consume()
        self.create_statement()
        with sqlite3.connect(str(self.db_path)) as conn:
            conn.execute("DROP TRIGGER test_nonce_claims_no_update")
            conn.execute("UPDATE test_nonce_claims SET reviewer_pubkey_spki_sha256=?",
                         ("f" * 64,))
            conn.executescript(DDL)
        self.rejects("CONTENT_DRIFT")

    def test_witness_source_mismatch_rejected(self):
        self.create_statement()
        self.rejects("SOURCE_MISMATCH", approved_test_source_sha256="b" * 64)

    def test_witness_key_fingerprint_not_trusted(self):
        self.create_statement()
        self.rejects("WITNESS_KEY_NOT_EXTRINSICALLY_PINNED",
                     externally_pinned_witness_pubkey_sha256="b" * 64)

    def test_witness_key_substitution_rejected(self):
        self.create_statement()
        self.rejects("WITNESS_KEY_NOT_EXTRINSICALLY_PINNED",
                     witness_pub=self.public["A"])

    def test_witness_corrupt_signature_rejected(self):
        self.create_statement()
        self.sig[self.witness].write_bytes(b"x" * 64)
        self.rejects("SIGNATURE_OR_KEY_INVALID")

    def test_witness_missing_signature_rejected(self):
        self.create_statement()
        self.sig[self.witness].unlink()
        self.rejects("MISSING_OR_TOO_LARGE")

    def test_witness_future_issued_at_rejected(self):
        self.create_statement()
        self.statement["issued_at_epoch"] = self.now + 1000
        self.rejects("INVALID_OR_FUTURE_ISSUED_AT")

    def test_witness_bool_sequence_fails_closed(self):
        self.create_statement()
        self.statement["sequence"] = True
        self.rejects("INVALID_SEQUENCE")

    def test_witness_disconnected_non_genesis_sequence_rejected(self):
        self.create_statement(seq=2, prev=GENESIS)
        self.external_sequence = 2
        self.rejects("INVALID_GENESIS_LINK")

    def test_witness_missing_external_anchor_fails_closed(self):
        self.create_statement()
        self.rejects("EXTERNAL_HEAD", externally_pinned_latest_head_sha256="")

    def test_witness_absent_db_fails_closed(self):
        self.create_statement()
        self.db_path.unlink()
        self.rejects("STORE_MISSING")


# Avoid executing all inherited nonce/checkpoint cases again.
def load_tests(loader, standard_tests, pattern):
    result = unittest.TestSuite()
    for method in sorted(name for name in SignedCheckpointWitnessTest.__dict__
                         if name.startswith("test_witness_")):
        result.addTest(SignedCheckpointWitnessTest(method))
    return result

if __name__ == "__main__":
    unittest.main()
