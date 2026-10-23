"""G5.3 TEST-only checkpoint/rollback negative controls; ephemeral local SQLite."""
import copy
import shutil
import sqlite3
import unittest
from pathlib import Path
from g53_attestation import canonical, sha256
from g53_ledger_checkpoint import (
    CheckpointRejected, observe_checkpoint, verify_checkpoint,
)
from g53_nonce_store import DDL
from g53_preflight import PreflightRejected
from test_g53_nonce_store import TestAtomicNonceStore

SOURCE = "c" * 64

class LedgerCheckpointTest(TestAtomicNonceStore):
    def checkpoint(self):
        return observe_checkpoint(self.db_path, SOURCE)

    def audit(self, checkpoint=None, digest=None, source=SOURCE):
        if checkpoint is None:
            checkpoint = self.checkpoint()
        if digest is None:
            digest = sha256(canonical(checkpoint))
        return verify_checkpoint(
            self.db_path, checkpoint, externally_approved_digest=digest,
            reviewed_source_sha256=source)

    def second_claim(self):
        self.challenge["nonce_hex"] = "d4" * 32
        self.approved_digest = sha256(canonical(self.challenge))
        self.registry["challenge_sha256"] = self.approved_digest
        self.sign()
        self.consume()

    def test_checkpoint_empty_ledger_explicit_hold(self):
        data = self.checkpoint()
        self.assertEqual(data["row_count"], 0)
        msg = self.audit(data)
        self.assertIn("G53_TEST_LEDGER_CHECKPOINT_MATCH=PASS", msg)
        self.assertIn("G53_LATEST_EXTERNAL_CHECKPOINT_PROVENANCE=NOT_PROVEN", msg)
        self.assertIn("G53_GO_NO_GO=HOLD_PENDING_INDEPENDENT_REVIEW", msg)

    def test_checkpoint_valid_consumption_and_stable_reopen(self):
        self.consume()
        snapshot = self.checkpoint()
        self.assertEqual(snapshot["row_count"], 1)
        self.assertEqual(snapshot, self.checkpoint())
        self.assertIn("G53_TEST_LEDGER_CHECKPOINT_MATCH=PASS", self.audit(snapshot))

    def test_checkpoint_new_authorized_consumption_fails_old_pin(self):
        self.consume()
        checkpoint = self.checkpoint()
        self.second_claim()
        with self.assertRaisesRegex(PreflightRejected, "ROLLBACK_OR_COUNT_CHANGE"):
            self.audit(checkpoint)

    def test_checkpoint_rolled_back_file_detected_from_later_pin(self):
        self.consume()
        backup = self.root / "OLD-test-database-copy"
        shutil.copy2(self.db_path, backup)
        self.second_claim()
        later_checkpoint = self.checkpoint()
        self.assertEqual(later_checkpoint["row_count"], 2)
        shutil.copy2(backup, self.db_path)
        with self.assertRaisesRegex(PreflightRejected, "ROLLBACK_OR_COUNT_CHANGE"):
            self.audit(later_checkpoint)

    def test_checkpoint_same_count_row_mutation_detected(self):
        self.consume()
        checkpoint = self.checkpoint()
        with sqlite3.connect(str(self.db_path)) as conn:
            conn.execute("DROP TRIGGER test_nonce_claims_no_update")
            conn.execute("UPDATE test_nonce_claims SET reviewer_pubkey_spki_sha256=?",
                         ("f" * 64,))
            conn.executescript(DDL)
        self.assertEqual(self.checkpoint()["row_count"], 1)
        with self.assertRaisesRegex(PreflightRejected, "CONTENT_DRIFT"):
            self.audit(checkpoint)

    def test_checkpoint_schema_guard_removed_detected(self):
        self.consume()
        checkpoint = self.checkpoint()
        with sqlite3.connect(str(self.db_path)) as conn:
            conn.execute("DROP TRIGGER test_nonce_claims_no_delete")
        with self.assertRaisesRegex(PreflightRejected, "SCHEMA_GUARDS_MISSING"):
            self.audit(checkpoint)

    def test_checkpoint_digest_wrong_external_pin_rejected(self):
        self.consume()
        with self.assertRaisesRegex(PreflightRejected, "EXTERNAL_DIGEST_MISMATCH"):
            self.audit(digest="0" * 64)

    def test_checkpoint_forged_count_rejected(self):
        self.consume()
        genuine = self.checkpoint()
        # Model a genuine, previously approved independent checkpoint pin:
        # do NOT re-pin a forged candidate to itself.
        pinned_digest = sha256(canonical(genuine))
        forged = dict(genuine, row_count=0)
        with self.assertRaisesRegex(PreflightRejected, "EXTERNAL_DIGEST_MISMATCH"):
            self.audit(checkpoint=forged, digest=pinned_digest)

    def test_checkpoint_wrong_reviewed_source_rejected(self):
        self.consume()
        with self.assertRaisesRegex(PreflightRejected, "SOURCE_MISMATCH"):
            self.audit(source="b" * 64)

    def test_checkpoint_missing_test_store_fails_closed(self):
        self.db_path.unlink()
        with self.assertRaisesRegex(PreflightRejected, "STORE_MISSING"):
            self.checkpoint()

    def test_checkpoint_extra_raw_row_detected(self):
        self.consume()
        previous = self.checkpoint()
        with sqlite3.connect(str(self.db_path)) as conn:
            conn.execute("INSERT INTO test_nonce_claims VALUES (?,?,?,?,?,?)", (
                "f" * 64, "e" * 64, SOURCE, "c" * 64, 2000000000, 100))
        with self.assertRaisesRegex(PreflightRejected, "ROLLBACK_OR_COUNT_CHANGE"):
            self.audit(previous)

    def test_checkpoint_rejects_noninteger_count_even_with_matching_digest(self):
        forged = dict(self.checkpoint(), row_count=True)
        with self.assertRaisesRegex(PreflightRejected, "INVALID_COUNT"):
            self.audit(checkpoint=forged, digest=sha256(canonical(forged)))

# Do NOT rerun 15 imported nonce tests (or 17 witness fixtures) in this suite:
# unittest imports the prior TestCase for fixture reuse. Limit to our own tests.
def load_tests(loader, tests, pattern):
    suite = unittest.TestSuite()
    for name in sorted(n for n in LedgerCheckpointTest.__dict__
                       if n.startswith("test_checkpoint_")):
        suite.addTest(LedgerCheckpointTest(name))
    return suite

if __name__ == "__main__":
    unittest.main()
