"""Ephemeral TEST-only concurrency experiments; never production nonce authority."""
import copy
import sqlite3
import subprocess
import sys
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
import tempfile
import unittest

from g53_attestation import canonical, sha256
from g53_nonce_store import (NonceClaimRejected, consume_verified_test_session,
                             provision_test_store, test_store_claim_count)
from g53_preflight import PreflightRejected
from test_g53_witness_audit import WitnessAuditTest

class TestAtomicNonceStore(unittest.TestCase):
    # Share fabricated fixture setup without rerunning the witness test suite.
    sign_one = WitnessAuditTest.sign_one
    sign = WitnessAuditTest.sign
    def setUp(self):
        WitnessAuditTest.setUp(self)
        self.db_path = self.root / "synthetic-nonce-ledger.sqlite3"
        provision_test_store(self.db_path)

    def consume(self):
        return consume_verified_test_session(
            self.db_path, self.policy, self.a, self.b, self.challenge,
            pub_a=self.public["A"], pub_b=self.public["B"],
            sig_a=self.sig["A"], sig_b=self.sig["B"],
            approved_challenge_sha256=self.approved_digest,
            registry=self.registry, reviewer_pub=self.public["REVIEWER"],
            reviewer_sig=self.sig["REVIEWER"],
            reviewer_expected_pubkey_sha256=self.review_pin,
            nonce_snapshot=self.snapshot, now_epoch=self.now)

    def test_first_valid_claim_succeeds_but_physical_go_stays_hold(self):
        result = self.consume()
        self.assertIn("G53_TEST_LOCAL_ATOMIC_NONCE_CONSUMPTION=PASS", result)
        self.assertIn("G53_DISTRIBUTED_NONCE_CONSENSUS=NOT_PROVEN", result)
        self.assertIn("G53_GO_NO_GO=HOLD", result)
        self.assertEqual(test_store_claim_count(self.db_path), 1)

    def test_reuse_after_restart_is_rejected(self):
        self.consume()
        with self.assertRaisesRegex(NonceClaimRejected, "ALREADY_CONSUMED"):
            self.consume()
        self.assertEqual(test_store_claim_count(self.db_path), 1)

    def test_six_concurrent_identical_claims_yield_exactly_one_commit(self):
        def attempt(_):
            try:
                self.consume()
                return "ACCEPT"
            except NonceClaimRejected as exc:
                if "ALREADY_CONSUMED" in str(exc):
                    return "REPLAY_REJECTED"
                raise
        with ThreadPoolExecutor(max_workers=6) as pool:
            outcomes = list(pool.map(attempt, range(6)))
        self.assertEqual(outcomes.count("ACCEPT"), 1, outcomes)
        self.assertEqual(outcomes.count("REPLAY_REJECTED"), 5, outcomes)
        self.assertEqual(test_store_claim_count(self.db_path), 1)

    def test_same_nonce_new_signed_challenge_rejected(self):
        self.consume()
        self.challenge["expires_at_epoch"] += 1
        self.approved_digest = sha256(canonical(self.challenge))
        self.registry["challenge_sha256"] = self.approved_digest
        self.sign()
        with self.assertRaisesRegex(NonceClaimRejected, "ALREADY_CONSUMED"):
            self.consume()

    def test_new_nonce_distinct_signed_challenge_accepted(self):
        self.consume()
        self.challenge["nonce_hex"] = "d4" * 32
        self.approved_digest = sha256(canonical(self.challenge))
        self.registry["challenge_sha256"] = self.approved_digest
        self.sign()
        self.assertIn("G53_TEST_LOCAL_ATOMIC_NONCE_CONSUMPTION=PASS", self.consume())
        self.assertEqual(test_store_claim_count(self.db_path), 2)

    def test_bad_signature_does_not_consume_nonce(self):
        self.sig["A"].write_bytes(b"x" * 64)
        with self.assertRaises(PreflightRejected):
            self.consume()
        self.assertEqual(test_store_claim_count(self.db_path), 0)
        self.sign()
        self.consume()
        self.assertEqual(test_store_claim_count(self.db_path), 1)

    def test_real_gate_violation_does_not_consume(self):
        self.a["real_persistence_enabled"] = True
        with self.assertRaisesRegex(PreflightRejected, "REAL_GATE_OPEN"):
            self.consume()
        self.assertEqual(test_store_claim_count(self.db_path), 0)

    def test_unprovisioned_store_fails_closed(self):
        self.db_path.unlink()
        with self.assertRaisesRegex(NonceClaimRejected, "NOT_PROVISIONED"):
            self.consume()

    def test_double_provisioning_rejected(self):
        with self.assertRaisesRegex(NonceClaimRejected, "ALREADY_EXISTS"):
            provision_test_store(self.db_path)

    def test_database_update_and_delete_are_blocked(self):
        self.consume()
        with sqlite3.connect(str(self.db_path)) as conn:
            with self.assertRaisesRegex(sqlite3.DatabaseError, "TEST_APPEND_ONLY_UPDATE_REJECTED"):
                conn.execute("UPDATE test_nonce_claims SET nonce_hex=?",
                             ("a1" * 32,))
            with self.assertRaisesRegex(sqlite3.DatabaseError, "TEST_APPEND_ONLY_DELETE_REJECTED"):
                conn.execute("DELETE FROM test_nonce_claims")
        self.assertEqual(test_store_claim_count(self.db_path), 1)

    def test_duplicate_sql_insert_from_independent_process_fails(self):
        self.consume()
        script = (
            "import sqlite3,sys; c=sqlite3.connect(sys.argv[1]); "
            "c.execute('INSERT INTO test_nonce_claims "
            "(challenge_sha256,nonce_hex,approved_test_source_sha,"
            "reviewer_pubkey_spki_sha256,expires_at_epoch,claimed_at_epoch) "
            "VALUES (?,?,?,?,?,?)', "
            "(sys.argv[2],sys.argv[3],sys.argv[4],sys.argv[5],2000000000,100)); c.commit()"
        )
        run = subprocess.run([sys.executable, '-c', script,
                              str(self.db_path), self.approved_digest,
                              self.challenge['nonce_hex'],
                              self.policy['approved_test_source_sha'], self.review_pin],
                             capture_output=True, text=True, timeout=5)
        self.assertNotEqual(run.returncode, 0)
        self.assertIn('UNIQUE constraint failed', run.stderr)
        self.assertEqual(test_store_claim_count(self.db_path), 1)

    def test_missing_append_only_trigger_fails_closed(self):
        with sqlite3.connect(str(self.db_path)) as conn:
            conn.execute('DROP TRIGGER test_nonce_claims_no_delete')
        with self.assertRaisesRegex(NonceClaimRejected, 'SCHEMA_UNTRUSTED'):
            self.consume()
        self.assertEqual(0, sqlite3.connect(str(self.db_path)).execute(
            'SELECT COUNT(*) FROM test_nonce_claims').fetchone()[0])

    def test_missing_nonce_unique_constraint_fails_closed(self):
        self.db_path.unlink()
        with sqlite3.connect(str(self.db_path)) as conn:
            conn.executescript('''CREATE TABLE test_nonce_claims(
                challenge_sha256 TEXT PRIMARY KEY NOT NULL,
                nonce_hex TEXT NOT NULL,
                approved_test_source_sha TEXT NOT NULL,
                reviewer_pubkey_spki_sha256 TEXT NOT NULL,
                expires_at_epoch INTEGER NOT NULL, claimed_at_epoch INTEGER NOT NULL);
                CREATE TRIGGER test_nonce_claims_no_update
                BEFORE UPDATE ON test_nonce_claims
                BEGIN SELECT RAISE(ABORT, 'TEST_APPEND_ONLY_UPDATE_REJECTED'); END;
                CREATE TRIGGER test_nonce_claims_no_delete
                BEFORE DELETE ON test_nonce_claims
                BEGIN SELECT RAISE(ABORT, 'TEST_APPEND_ONLY_DELETE_REJECTED'); END;''')
        with self.assertRaisesRegex(NonceClaimRejected, 'SCHEMA_UNTRUSTED'):
            self.consume()

    def test_snapshot_incomplete_cannot_bypass_local_ledger(self):
        self.consume()
        self.snapshot["prior_consumptions"] = []  # deliberate stale supplied view
        with self.assertRaisesRegex(NonceClaimRejected, "ALREADY_CONSUMED"):
            self.consume()
        self.assertEqual(test_store_claim_count(self.db_path), 1)

    def test_expired_challenge_rejected_before_commit(self):
        # With a deterministic CI clock, the same fixture is valid at t=self.now,
        # but the transaction-level check must still fail if time advances.
        # Here call an expired fabricated epoch; witness rejects without claiming.
        self.now = self.challenge["expires_at_epoch"] + 1
        with self.assertRaisesRegex(PreflightRejected, "CHALLENGE_EXPIRED_OR_FUTURE"):
            self.consume()
        self.assertEqual(test_store_claim_count(self.db_path), 0)

if __name__ == "__main__":
    unittest.main()
