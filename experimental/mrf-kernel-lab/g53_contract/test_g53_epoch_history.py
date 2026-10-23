"""G5.3 TEST multiepoch key rotation with original historical signatures intact."""
import copy
import shutil
import unittest

from g53_attestation import canonical, sha256
from g53_epoch_history import verify_historical_epoch_chain
from g53_checkpoint_quorum import entry_head
from g53_preflight import PreflightRejected
from g53_root_enrollment import enrollment_head
from test_g53_root_enrollment import RootEnrollmentTest, SOURCE


class EpochHistoryTest(RootEnrollmentTest):
    def setUp(self):
        super().setUp()
        self.epochs = []
        self.latest_external_head = None
        self.latest_external_epoch = 0
        self.origin_now = self.now

    def save_epoch(self, records):
        self.epochs.append(copy.deepcopy({
            "anchor": self.anchor,
            "root_endorsements": self.anchor_endorsements,
            "registry": self.quorum_registry,
            "records": records,
        }))
        self.latest_external_head = enrollment_head(self.anchor)
        self.latest_external_epoch = len(self.epochs)

    def two_epochs(self):
        self.first()
        self.authorize()
        self.old_signatures = copy.deepcopy(self.chain[0]["endorsements"])
        self.save_epoch(self.chain[:1])
        old_root_head = self.latest_external_head
        self.old_revoked = self.reviewers[0]
        active = self.reviewers[1:]
        self.now = self.origin_now + 40
        self.second_claim()
        self.append(signers=active)
        self.quorum_registry["reviewer_pubkey_spki_sha256"] = active
        self.authorize(epoch=2, predecessor=old_root_head, revoked=[self.old_revoked])
        self.save_epoch(self.chain[1:])

    def verify_epochs(self, **changes):
        args = dict(
            root_public_pems=self.root_pems,
            reviewer_public_pems=self.review_pubs,
            externally_pinned_root_key_sha256=self.roots,
            externally_pinned_latest_root_head_sha256=self.latest_external_head,
            externally_pinned_latest_epoch=self.latest_external_epoch,
            independently_approved_source_sha256=SOURCE,
            independently_approved_authorization_ref=self.policy["review_authorization_ref"],
            now_epoch=self.now)
        args.update(changes)
        return verify_historical_epoch_chain(self.db_path, self.epochs, **args)

    def rejected(self, message, **changes):
        with self.assertRaisesRegex(PreflightRejected, message):
            self.verify_epochs(**changes)

    def test_epoch_rotation_preserves_original_signatures(self):
        self.two_epochs()
        result = self.verify_epochs()
        self.assertEqual(self.epochs[0]["records"][0]["endorsements"],
                         self.old_signatures)
        self.assertIn("G53_TEST_MULTIEPOCH_HISTORICAL_SIGNATURES=PASS", result)
        self.assertIn("G53_GO_NO_GO=HOLD", result)
        self.assertIn("G53_ACTUAL_REVIEWER_INDEPENDENCE=NOT_PROVEN", result)

    def test_epoch_historical_anchor_can_expire_without_invalidating_old_signatures(self):
        self.two_epochs()
        self.now = self.origin_now + 140
        self.assertIn("G53_TEST_MULTIEPOCH_HISTORICAL_SIGNATURES=PASS",
                      self.verify_epochs())

    def test_epoch_history_missing_genesis(self):
        self.two_epochs()
        self.epochs = self.epochs[1:]
        self.rejected("HISTORY_MISSING_OR_STALE")

    def test_epoch_reordered_policies(self):
        self.two_epochs()
        self.epochs.reverse()
        self.rejected("ROOT_EPOCH_GAP")

    def test_epoch_old_policy_replayed_as_latest(self):
        self.two_epochs()
        self.epochs = self.epochs[:1]
        self.rejected("HISTORY_MISSING_OR_STALE")

    def test_epoch_tamper_earlier_historical_signature(self):
        self.two_epochs()
        self.epochs[0]["records"][0]["endorsements"][0]["signature_hex"] = "0" * 128
        self.rejected("SIGNATURE_OR_KEY_INVALID")

    def test_epoch_insufficient_historical_signatures(self):
        self.two_epochs()
        old = self.epochs[0]["records"][0]["endorsements"]
        self.epochs[0]["records"][0]["endorsements"] = old[:1]
        self.rejected("INSUFFICIENT_HISTORICAL_QUORUM")

    def test_epoch_retroactive_new_roster_cannot_replace_old(self):
        self.two_epochs()
        self.epochs[0]["registry"] = copy.deepcopy(self.epochs[1]["registry"])
        self.rejected("ROSTER_HASH_MISMATCH")

    def test_epoch_revoked_reviewer_cannot_approve_new_checkpoint(self):
        self.two_epochs()
        item = self.epochs[1]["records"][0]
        item["endorsements"] = self.sign_statement(
            item["statement"], signers=self.reviewers[:2])
        self.rejected("REVIEWER_WRONG_EPOCH")

    def test_epoch_broken_root_policy_chain(self):
        self.two_epochs()
        self.epochs[1]["anchor"]["previous_anchor_sha256"] = "f" * 64
        self.rejected("ROOT_FORK_OR_MISSING_PREDECESSOR")

    def test_epoch_changed_second_checkpoint_predecessor(self):
        self.two_epochs()
        self.epochs[1]["records"][0]["statement"]["previous_head_sha256"] = "f" * 64
        self.rejected("RECORD_FORK")

    def test_epoch_newer_signature_with_old_record_sequence(self):
        self.two_epochs()
        self.epochs[1]["records"][0]["statement"]["sequence"] = 1
        self.rejected("GLOBAL_SEQUENCE_GAP")

    def test_epoch_rollback_of_local_database(self):
        self.two_epochs()
        old_db = self.root / "rolled-back-db"
        # Exercise an earlier valid SQLite database snapshot by removing the
        # last row through a privileged TEST fixture only.
        import sqlite3
        from g53_nonce_store import DDL
        with sqlite3.connect(str(self.db_path)) as c:
            c.execute("DROP TRIGGER test_nonce_claims_no_delete")
            c.execute("DELETE FROM test_nonce_claims WHERE nonce_hex = ?", ("d4" * 32,))
            c.executescript(DDL)
        self.rejected("ROLLBACK_OR_COUNT_CHANGE")

    def test_epoch_external_final_policy_head_mismatch(self):
        self.two_epochs()
        self.rejected("EXTERNAL_LATEST_ROOT_HEAD_MISMATCH",
                      externally_pinned_latest_root_head_sha256="f" * 64)

    def test_epoch_wrong_root_identity_pin(self):
        self.two_epochs()
        pins = sorted(["f" * 64] + self.roots[:2])
        self.rejected("ROOT_SET_MISMATCH",
                      externally_pinned_root_key_sha256=pins)

    def test_epoch_missing_root_quorum(self):
        self.two_epochs()
        self.epochs[1]["root_endorsements"] = self.epochs[1]["root_endorsements"][:1]
        self.rejected("ROOT_QUORUM_TOO_SMALL")

    def test_epoch_forged_root_signature(self):
        self.two_epochs()
        self.epochs[1]["root_endorsements"][0]["signature_hex"] = "0" * 128
        self.rejected("SIGNATURE_OR_KEY_INVALID")

    def test_epoch_revocation_list_overlaps_active_roster(self):
        self.two_epochs()
        self.epochs[1]["anchor"]["revoked_reviewer_pubkey_spki_sha256"] = sorted(
            [self.old_revoked, self.reviewers[1]])
        self.rejected("REVOKED_REENROLLED")

    def test_epoch_signed_root_authorizes_different_roster_rejected(self):
        self.two_epochs()
        self.epochs[1]["anchor"]["active_reviewer_registry_sha256"] = "a" * 64
        self.rejected("ROSTER_HASH_MISMATCH")

    def test_epoch_historical_signer_newly_revoked_retains_old_signature(self):
        self.two_epochs()
        hist = self.epochs[0]["records"][0]["endorsements"]
        self.assertIn(self.old_revoked, [s["reviewer_pubkey_spki_sha256"] for s in hist])
        self.assertIn("G53_TEST_TEMPORAL_REVOCATION_ENFORCEMENT=PASS",
                      self.verify_epochs())

    def test_epoch_second_record_before_new_policy_issued_rejected(self):
        self.two_epochs()
        self.epochs[1]["records"][0]["statement"]["issued_at_epoch"] = (
            self.epochs[1]["anchor"]["issued_at_epoch"] - 1)
        self.rejected("RECORD_TIME_OUTSIDE_AUTHORIZED_EPOCH")

    def test_epoch_new_policy_checkpoint_regresses_in_count(self):
        self.two_epochs()
        self.epochs[1]["records"][0]["statement"]["checkpoint"]["row_count"] = 0
        self.rejected("CHECKPOINT_ROW_REGRESSION")

    def test_epoch_latest_root_expired(self):
        self.two_epochs()
        self.now = self.origin_now + 200
        self.rejected("LATEST_ROOT_EXPIRED")

    def test_epoch_unauthorized_source_switch(self):
        self.two_epochs()
        self.epochs[1]["anchor"]["approved_source_sha256"] = "f" * 64
        self.rejected("ROOT_SOURCE_MISMATCH")


def load_tests(loader, tests, pattern):
    suite = unittest.TestSuite()
    for name in sorted(n for n in EpochHistoryTest.__dict__
                       if n.startswith("test_epoch_")):
        suite.addTest(EpochHistoryTest(name))
    return suite


if __name__ == "__main__":
    unittest.main()
