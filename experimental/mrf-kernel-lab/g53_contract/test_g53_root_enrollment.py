"""Fabricated TEST root-of-trust only: no live enrollment or independent custodian."""
import copy
import subprocess
import unittest
from pathlib import Path

from g53_attestation import canonical, public_der, sha256
from g53_checkpoint_quorum import entry_head
from g53_root_enrollment import (
    GENESIS, SCHEMA, enrollment_head, enrollment_message,
    verify_enrolled_checkpoint_chain,
)
from g53_preflight import PreflightRejected
from test_g53_checkpoint_quorum import QuorumChainTest, SOURCE


class RootEnrollmentTest(QuorumChainTest):
    def setUp(self):
        super().setUp()
        self.root_keys, self.root_pems = {}, {}
        for role in ("ROOT_1", "ROOT_2", "ROOT_3"):
            private = self.root / (role + ".private.test.pem")
            public = self.root / (role + ".public.test.pem")
            subprocess.run(["openssl", "genpkey", "-algorithm", "ED25519",
                            "-out", str(private)], check=True, capture_output=True)
            subprocess.run(["openssl", "pkey", "-in", str(private), "-pubout",
                            "-out", str(public)], check=True, capture_output=True)
            digest = sha256(public_der(public))
            self.root_keys[digest] = private
            self.root_pems[digest] = public
        self.roots = sorted(self.root_keys)
        self.anchor = None
        self.anchor_endorsements = []
        self.anchor_head = None
        self.anchor_epoch = 1
        self.anchor_predecessor = GENESIS

    def authorize(self, *, epoch=1, predecessor=GENESIS, revoked=None,
                  signers=None):
        if revoked is None:
            revoked = []
        self.anchor = {
            "schema": SCHEMA,
            "epoch": epoch,
            "previous_anchor_sha256": predecessor,
            "approved_source_sha256": SOURCE,
            "authorization_ref": self.policy["review_authorization_ref"],
            "active_reviewer_registry_sha256": sha256(canonical(self.quorum_registry)),
            "revoked_reviewer_pubkey_spki_sha256": sorted(revoked),
            "latest_checkpoint_head_sha256": self.latest_head,
            "latest_checkpoint_sequence": self.latest_seq,
            "issued_at_epoch": self.now - 20,
            "expires_at_epoch": self.now + 120,
        }
        self.anchor_epoch = epoch
        self.anchor_predecessor = predecessor
        self.sign_anchor(signers=signers)
        return self.anchor

    def sign_anchor(self, signers=None):
        if signers is None:
            signers = self.roots[:2]
        endorsements = []
        for signer in sorted(signers):
            msg = self.root / "root.msg"
            sig = self.root / "root.sig"
            msg.write_bytes(enrollment_message(self.anchor))
            subprocess.run(["openssl", "pkeyutl", "-sign",
                            "-inkey", str(self.root_keys[signer]),
                            "-rawin", "-in", str(msg), "-out", str(sig)],
                           check=True, capture_output=True)
            endorsements.append({
                "root_pubkey_spki_sha256": signer,
                "signature_hex": sig.read_bytes().hex(),
            })
        self.anchor_endorsements = endorsements
        self.anchor_head = enrollment_head(self.anchor)

    def check(self, **changes):
        params = dict(
            reviewer_public_pems=self.review_pubs,
            root_public_pems=self.root_pems,
            externally_pinned_root_key_sha256=self.roots,
            externally_pinned_latest_anchor_sha256=self.anchor_head,
            externally_pinned_latest_anchor_epoch=self.anchor_epoch,
            externally_pinned_previous_anchor_sha256=self.anchor_predecessor,
            externally_approved_source_sha256=SOURCE,
            externally_approved_authorization_ref=self.policy["review_authorization_ref"],
            now_epoch=self.now,
        )
        params.update(changes)
        return verify_enrolled_checkpoint_chain(
            self.db_path, self.chain, self.quorum_registry,
            self.anchor, self.anchor_endorsements, **params)

    def rejects(self, marker, **changes):
        with self.assertRaisesRegex(PreflightRejected, marker):
            self.check(**changes)

    def ready(self):
        self.first()
        self.authorize()

    def test_root_two_of_three_fixture_pass_remains_hold(self):
        self.ready()
        out = self.check()
        self.assertIn("G53_TEST_ROOT_QUORUM_ENROLLMENT=PASS", out)
        self.assertIn("G53_TEST_ROOT_SIGNED_LATEST_HEAD=PASS", out)
        self.assertIn("G53_ROOT_SIGNER_HUMAN_INDEPENDENCE=NOT_PROVEN", out)
        self.assertIn("G53_GO_NO_GO=HOLD", out)

    def test_root_three_of_three_signers_accepted(self):
        self.first()
        self.authorize(signers=self.roots)
        self.assertIn("G53_TEST_ROOT_QUORUM_ENROLLMENT=PASS", self.check())

    def test_root_epoch_two_with_pinned_predecessor(self):
        self.ready()
        old_head = self.anchor_head
        self.second()
        self.authorize(epoch=2, predecessor=old_head)
        self.assertIn("G53_TEST_ROOT_SIGNED_LATEST_HEAD=PASS", self.check())

    def test_root_old_signed_anchor_replayed_vs_epoch_two(self):
        self.ready()
        self.rejects("STALE_OR_FUTURE_EPOCH",
                     externally_pinned_latest_anchor_epoch=2)

    def test_root_old_anchor_replayed_vs_new_head(self):
        self.ready()
        self.rejects("EXTERNAL_HEAD_MISMATCH",
                     externally_pinned_latest_anchor_sha256="f" * 64)

    def test_root_epoch_two_with_wrong_predecessor_rejected(self):
        self.ready()
        self.anchor["epoch"] = 2
        self.anchor_epoch = 2
        self.anchor_predecessor = "f" * 64
        self.rejects("PREDECESSOR_NOT_PINNED")

    def test_root_invalid_genesis_link(self):
        self.ready()
        self.anchor["previous_anchor_sha256"] = "f" * 64
        self.anchor_predecessor = "f" * 64
        self.rejects("INVALID_GENESIS_LINK")

    def test_root_tamper_registry_without_root_signature(self):
        self.ready()
        self.anchor["active_reviewer_registry_sha256"] = "f" * 64
        self.rejects("EXTERNAL_HEAD_MISMATCH")

    def test_root_validly_resigned_forged_current_head_is_not_latest(self):
        self.ready()
        self.anchor["latest_checkpoint_head_sha256"] = "f" * 64
        self.sign_anchor()
        self.rejects("EXTERNAL_HEAD_MISMATCH",
                     externally_pinned_latest_anchor_sha256="a" * 64)

    def test_root_wrong_expected_root_fingerprint(self):
        self.ready()
        pins = ["1" * 64, self.roots[1], self.roots[2]]
        self.rejects("MISSING_OR_EXTRA_ROOT_KEY",
                     externally_pinned_root_key_sha256=sorted(set(pins)))

    def test_root_pubkey_substitution(self):
        self.ready()
        altered = dict(self.root_pems)
        altered[self.roots[0]] = self.root_pems[self.roots[1]]
        self.rejects("ROOT_KEY_MISMATCH", root_public_pems=altered)

    def test_root_insufficient_signatures(self):
        self.ready()
        self.anchor_endorsements = self.anchor_endorsements[:1]
        self.rejects("INSUFFICIENT_OR_EXCESS_ENDORSEMENTS")

    def test_root_duplicate_signer(self):
        self.ready()
        self.anchor_endorsements[1] = copy.deepcopy(self.anchor_endorsements[0])
        self.rejects("DUPLICATE_OR_UNSORTED_SIGNERS")

    def test_root_signature_corruption(self):
        self.ready()
        self.anchor_endorsements[0]["signature_hex"] = "0" * 128
        self.rejects("SIGNATURE_OR_KEY_INVALID")

    def test_root_time_expired(self):
        self.ready()
        self.rejects("STALE_OR_FUTURE_TIME", now_epoch=self.now + 500)

    def test_root_time_before_issue(self):
        self.ready()
        self.rejects("STALE_OR_FUTURE_TIME", now_epoch=self.now - 500)

    def test_root_window_exceeds_limit(self):
        self.ready()
        self.anchor["expires_at_epoch"] = self.now + 5000
        self.rejects("INVALID_TIME_WINDOW")

    def test_root_duplicate_revoked_fingerprint(self):
        self.ready()
        v = self.reviewers[0]
        self.anchor["revoked_reviewer_pubkey_spki_sha256"] = [v, v]
        self.rejects("DUPLICATE_OR_UNSORTED_REVOCATIONS")

    def test_root_active_revoked_overlap_fails(self):
        self.ready()
        self.anchor["revoked_reviewer_pubkey_spki_sha256"] = [self.reviewers[0]]
        self.rejects("ACTIVE_REVOKED_OVERLAP")

    def test_root_revocation_and_rotation_to_two_active_reviewers(self):
        self.first()
        removed = self.reviewers[0]
        active = self.reviewers[1:]
        self.chain[0]["endorsements"] = self.sign_statement(
            self.chain[0]["statement"], signers=active)
        self.latest_head = entry_head(self.chain[0])
        self.quorum_registry["reviewer_pubkey_spki_sha256"] = active
        # The policy approves a separate two-member active roster in TEST.
        self.authorize(revoked=[removed])
        pub = {k: self.review_pubs[k] for k in active}
        result = self.check(reviewer_public_pems=pub)
        self.assertIn("G53_TEST_REVOKED_SIGNER_EXCLUSION=PASS", result)
        self.assertIn("G53_GO_NO_GO=HOLD", result)

    def test_root_revocation_forged_old_registry_rejected(self):
        self.ready()
        # A root-signed new roster digest cannot be bypassed using old registry.
        self.anchor["active_reviewer_registry_sha256"] = "a" * 64
        self.sign_anchor()
        self.rejects("UNAPPROVED_REVIEWER_ROSTER")

    def test_root_new_checkpoint_beyond_signed_latest_head_rejected(self):
        self.ready()
        self.second()
        self.rejects("EXTERNAL_SEQUENCE_MISMATCH")

    def test_root_source_mismatch(self):
        self.ready()
        self.rejects("SOURCE_MISMATCH",
                     externally_approved_source_sha256="f" * 64)

    def test_root_invalid_epoch_bool(self):
        self.ready()
        self.anchor["epoch"] = True
        self.rejects("EPOCH_TYPE")

    def test_root_unexpected_anchor_field_rejected(self):
        self.ready()
        self.anchor["promote_to_real"] = True
        self.rejects("KEY_SET_MISMATCH")

    def test_root_missing_test_database(self):
        self.ready()
        self.db_path.unlink()
        self.rejects("STORE_MISSING")


def load_tests(loader, tests, pattern):
    result = unittest.TestSuite()
    for method in sorted(name for name in RootEnrollmentTest.__dict__
                         if name.startswith("test_root_")):
        result.addTest(RootEnrollmentTest(method))
    return result


if __name__ == "__main__":
    unittest.main()
