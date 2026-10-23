"""G5.3 TEST-only old+new root quorum handovers with historical signatures."""
import copy
import subprocess
import unittest
from pathlib import Path

from g53_attestation import canonical, public_der, sha256
from g53_epoch_history import verify_historical_epoch_chain
from g53_root_enrollment import enrollment_head, enrollment_message
from g53_root_rotation import SCHEMA, handover_message
from g53_preflight import PreflightRejected
from test_g53_epoch_history import EpochHistoryTest, SOURCE


class RootHandoverTest(EpochHistoryTest):
    def setUp(self):
        super().setUp()
        self.two_epochs()
        self.old_root_set = list(self.roots)
        self.old_epoch_snapshot = copy.deepcopy(self.epochs[0])
        self.old_review_signatures = copy.deepcopy(
            self.epochs[0]["records"][0]["endorsements"])
        self.new_root_set = []
        for role in ("ROTATED_ROOT_1", "ROTATED_ROOT_2", "ROTATED_ROOT_3"):
            private = self.root / (role + ".key.pem")
            public = self.root / (role + ".pub.pem")
            subprocess.run(["openssl", "genpkey", "-algorithm", "ED25519",
                            "-out", str(private)], capture_output=True, check=True)
            subprocess.run(["openssl", "pkey", "-in", str(private), "-pubout",
                            "-out", str(public)], capture_output=True, check=True)
            digest = sha256(public_der(public))
            self.root_keys[digest] = private
            self.root_pems[digest] = public
            self.new_root_set.append(digest)
        self.new_root_set.sort()
        self.epochs[1]["root_endorsements"] = self.sign_root_anchor(
            self.epochs[1]["anchor"], self.new_root_set[:2])
        self.handover = {
            "statement": {
                "schema": SCHEMA,
                "epoch": 2,
                "previous_anchor_head_sha256": enrollment_head(
                    self.epochs[0]["anchor"]),
                "new_anchor_head_sha256": enrollment_head(
                    self.epochs[1]["anchor"]),
                "old_root_fingerprints": self.old_root_set,
                "new_root_fingerprints": self.new_root_set,
                "source_sha256": SOURCE,
                "authorization_ref": self.policy["review_authorization_ref"],
            },
        }
        self.sign_transfer()
        self.rotations = [self.handover]

    def _sign_group(self, statement_bytes, signers):
        signatures = []
        for signer in sorted(signers):
            msg = self.root / "root-handover-msg.test"
            sig = self.root / "root-handover-sig.test"
            msg.write_bytes(statement_bytes)
            subprocess.run(["openssl", "pkeyutl", "-sign", "-inkey",
                            str(self.root_keys[signer]), "-rawin", "-in",
                            str(msg), "-out", str(sig)],
                           capture_output=True, check=True)
            signatures.append({
                "root_pubkey_spki_sha256": signer,
                "signature_hex": sig.read_bytes().hex(),
            })
        return signatures

    def sign_root_anchor(self, anchor, signers):
        return self._sign_group(enrollment_message(anchor), signers)

    def sign_transfer(self):
        payload = handover_message(self.handover["statement"])
        self.handover["old_endorsements"] = self._sign_group(
            payload, self.old_root_set[:2])
        self.handover["new_endorsements"] = self._sign_group(
            payload, self.new_root_set[:2])

    def verify_rotation(self, **changes):
        arguments = dict(
            root_public_pems=self.root_pems,
            reviewer_public_pems=self.review_pubs,
            externally_pinned_root_key_sha256=self.old_root_set,
            externally_pinned_latest_root_key_sha256=self.new_root_set,
            root_rotations=self.rotations,
            externally_pinned_latest_root_head_sha256=self.latest_external_head,
            externally_pinned_latest_epoch=self.latest_external_epoch,
            independently_approved_source_sha256=SOURCE,
            independently_approved_authorization_ref=self.policy["review_authorization_ref"],
            now_epoch=self.now,
        )
        arguments.update(changes)
        return verify_historical_epoch_chain(self.db_path, self.epochs, **arguments)

    def rejected(self, marker, **changes):
        with self.assertRaisesRegex(PreflightRejected, marker):
            self.verify_rotation(**changes)

    def test_rotation_dual_quorum_pass_and_history_unchanged(self):
        result = self.verify_rotation()
        self.assertIn("G53_TEST_ROOT_KEY_DUAL_QUORUM_HANDOVER=PASS", result)
        self.assertIn("G53_GO_NO_GO=HOLD", result)
        self.assertEqual(self.epochs[0], self.old_epoch_snapshot)
        self.assertEqual(self.epochs[0]["records"][0]["endorsements"],
                         self.old_review_signatures)

    def test_rotation_both_quorums_three_of_three(self):
        self.handover["old_endorsements"] = self._sign_group(
            handover_message(self.handover["statement"]), self.old_root_set)
        self.handover["new_endorsements"] = self._sign_group(
            handover_message(self.handover["statement"]), self.new_root_set)
        self.epochs[1]["root_endorsements"] = self.sign_root_anchor(
            self.epochs[1]["anchor"], self.new_root_set)
        self.assertIn("G53_TEST_ROOT_KEY_DUAL_QUORUM_HANDOVER=PASS",
                      self.verify_rotation())

    def test_rotation_outgoing_single_signature_refused(self):
        self.handover["old_endorsements"] = self.handover["old_endorsements"][:1]
        self.rejected("OUTGOING:INSUFFICIENT_QUORUM")

    def test_rotation_incoming_single_signature_refused(self):
        self.handover["new_endorsements"] = self.handover["new_endorsements"][:1]
        self.rejected("INCOMING:INSUFFICIENT_QUORUM")

    def test_rotation_outgoing_corrupt_signature(self):
        self.handover["old_endorsements"][0]["signature_hex"] = "0" * 128
        self.rejected("SIGNATURE_OR_KEY_INVALID")

    def test_rotation_incoming_corrupt_signature(self):
        self.handover["new_endorsements"][0]["signature_hex"] = "0" * 128
        self.rejected("SIGNATURE_OR_KEY_INVALID")

    def test_rotation_old_signer_duplicate(self):
        self.handover["old_endorsements"][1] = copy.deepcopy(
            self.handover["old_endorsements"][0])
        self.rejected("OUTGOING:DUPLICATE_OR_UNSORTED_SIGNER")

    def test_rotation_new_signer_duplicate(self):
        self.handover["new_endorsements"][1] = copy.deepcopy(
            self.handover["new_endorsements"][0])
        self.rejected("INCOMING:DUPLICATE_OR_UNSORTED_SIGNER")

    def test_rotation_old_quorum_signed_only_by_new_roots(self):
        self.handover["old_endorsements"] = self._sign_group(
            handover_message(self.handover["statement"]), self.new_root_set[:2])
        self.rejected("OUTGOING:UNAUTHORIZED_SIGNER")

    def test_rotation_new_quorum_signed_only_by_old_roots(self):
        self.handover["new_endorsements"] = self._sign_group(
            handover_message(self.handover["statement"]), self.old_root_set[:2])
        self.rejected("INCOMING:UNAUTHORIZED_SIGNER")

    def test_rotation_missing_transition_refused(self):
        self.rejected("TRANSITION_GAP", root_rotations=[])

    def test_rotation_unauthorized_epoch_two_root_signers(self):
        self.epochs[1]["root_endorsements"] = self.sign_root_anchor(
            self.epochs[1]["anchor"], self.old_root_set[:2])
        self.rejected("ROOT_NOT_ENROLLED")

    def test_rotation_new_keys_cannot_resign_epoch_one_anchor(self):
        self.epochs[0]["root_endorsements"] = self.sign_root_anchor(
            self.epochs[0]["anchor"], self.new_root_set[:2])
        self.rejected("ROOT_NOT_ENROLLED")

    def test_rotation_latest_root_set_must_match_external_pin(self):
        self.rejected("EXTERNAL_LATEST_ROOT_SET_MISMATCH",
                      externally_pinned_latest_root_key_sha256=self.old_root_set)

    def test_rotation_changed_previous_anchor_hash(self):
        self.handover["statement"]["previous_anchor_head_sha256"] = "f" * 64
        self.rejected("PREVIOUS_ANCHOR_MISMATCH")

    def test_rotation_changed_new_anchor_hash(self):
        self.handover["statement"]["new_anchor_head_sha256"] = "f" * 64
        self.rejected("NEW_ANCHOR_MISMATCH")

    def test_rotation_unapproved_source_in_handover(self):
        self.handover["statement"]["source_sha256"] = "f" * 64
        self.rejected("SOURCE_OR_AUTHORIZATION_MISMATCH")

    def test_rotation_unapproved_authorization(self):
        self.handover["statement"]["authorization_ref"] = "TEST-altered"
        self.rejected("SOURCE_OR_AUTHORIZATION_MISMATCH")

    def test_rotation_bad_epoch(self):
        self.handover["statement"]["epoch"] = 3
        self.rejected("INVALID_EPOCH")

    def test_rotation_duplicate_new_root_fingerprints(self):
        self.handover["statement"]["new_root_fingerprints"] = [
            self.new_root_set[0], self.new_root_set[0]]
        self.rejected("NEW_ROOTS:NOT_SORTED_UNIQUE")

    def test_rotation_old_new_root_set_overlap(self):
        self.handover["statement"]["new_root_fingerprints"] = (
            self.old_root_set[:2])
        self.rejected("OLD_NEW_ROOT_SET_OVERLAP")

    def test_rotation_missing_new_public_key(self):
        keys = dict(self.root_pems)
        del keys[self.new_root_set[0]]
        self.rejected("PUBLIC_KEY_MAP_MISMATCH", root_public_pems=keys)

    def test_rotation_public_key_substitution(self):
        keys = dict(self.root_pems)
        keys[self.new_root_set[0]] = self.root_pems[self.new_root_set[1]]
        self.rejected("PUBLIC_KEY_NOT_PINNED", root_public_pems=keys)

    def test_rotation_old_historical_signature_corruption_refused(self):
        self.epochs[0]["records"][0]["endorsements"][0]["signature_hex"] = "0" * 128
        self.rejected("SIGNATURE_OR_KEY_INVALID")

    def test_rotation_old_policy_replay_with_new_head_pin(self):
        self.epochs = self.epochs[:1]
        self.rejected("HISTORY_MISSING_OR_STALE")

    def test_rotation_old_database_snapshot_rejected(self):
        self.db_path.unlink()
        self.rejected("STORE_MISSING")

    def test_rotation_unsigned_transition_binding_cannot_update_anchor(self):
        # A legitimate-looking epoch-2 policy altered after old+new signoffs
        # fails even when its original root keys could sign the altered anchor.
        self.epochs[1]["anchor"]["latest_checkpoint_sequence"] += 1
        self.rejected("NEW_ANCHOR_MISMATCH")


def load_tests(loader, tests, pattern):
    suite = unittest.TestSuite()
    for name in sorted(n for n in RootHandoverTest.__dict__
                       if n.startswith("test_rotation_")):
        suite.addTest(RootHandoverTest(name))
    return suite


if __name__ == "__main__":
    unittest.main()
