"""G5.3 TEST-only, fabricated multi-reviewer full-chain and alternative-history tests."""
import copy
import shutil
import subprocess
import unittest
from pathlib import Path

from g53_attestation import canonical, public_der, sha256
from g53_checkpoint_quorum import (
    GENESIS, REGISTRY_SCHEMA, SCHEMA, entry_head, signing_message,
    verify_quorum_checkpoint_chain,
)
from g53_preflight import PreflightRejected
from test_g53_ledger_checkpoint import LedgerCheckpointTest, SOURCE


class QuorumChainTest(LedgerCheckpointTest):
    def setUp(self):
        super().setUp()
        self.review_keys, self.review_pubs = {}, {}
        for role in ("REVIEWER_1", "REVIEWER_2", "REVIEWER_3"):
            private = self.root / (role + ".key.pem")
            public = self.root / (role + ".pub.pem")
            subprocess.run(["openssl", "genpkey", "-algorithm", "ED25519",
                            "-out", str(private)], check=True, capture_output=True)
            subprocess.run(["openssl", "pkey", "-in", str(private), "-pubout",
                            "-out", str(public)], check=True, capture_output=True)
            digest = sha256(public_der(public))
            self.review_keys[digest] = private
            self.review_pubs[digest] = public
        self.reviewers = sorted(self.review_keys)
        self.registry = {
            "schema": REGISTRY_SCHEMA,
            "test_source_sha256": SOURCE,
            "authorization_ref": self.policy["review_authorization_ref"],
            "reviewer_pubkey_spki_sha256": self.reviewers,
            "required_quorum": 2,
        }
        self.registry_pin = sha256(canonical(self.registry))
        self.chain, self.latest_head, self.latest_seq = [], GENESIS, 0

    def sign_statement(self, statement, signers=None):
        if signers is None:
            signers = self.reviewers[:2]
        endorsements = []
        for digest in sorted(signers):
            msg = self.root / "statement.msg"
            sig = self.root / "statement.sig"
            msg.write_bytes(signing_message(statement))
            subprocess.run(["openssl", "pkeyutl", "-sign", "-inkey",
                            str(self.review_keys[digest]), "-rawin", "-in",
                            str(msg), "-out", str(sig)],
                           check=True, capture_output=True)
            endorsements.append({"reviewer_pubkey_spki_sha256": digest,
                                 "signature_hex": sig.read_bytes().hex()})
        return endorsements

    def append(self, checkpoint=None, signers=None):
        statement = {
            "schema": SCHEMA, "sequence": len(self.chain) + 1,
            "previous_head_sha256": self.latest_head,
            "checkpoint": self.checkpoint() if checkpoint is None else checkpoint,
            "approved_test_source_sha256": SOURCE,
            "issued_at_epoch": self.now,
            "authorization_ref": self.policy["review_authorization_ref"],
        }
        item = {"statement": statement,
                "endorsements": self.sign_statement(statement, signers)}
        self.chain.append(item)
        self.latest_seq = len(self.chain)
        self.latest_head = entry_head(item)
        return item

    def first(self):
        self.consume()
        return self.append()

    def second(self):
        self.second_claim()
        return self.append()

    def verify(self, **overrides):
        args = dict(
            reviewer_public_pems=self.review_pubs,
            externally_approved_registry_sha256=self.registry_pin,
            externally_pinned_latest_head_sha256=self.latest_head,
            externally_pinned_latest_sequence=self.latest_seq,
            independently_approved_test_source_sha256=SOURCE,
            externally_approved_authorization_ref=self.policy["review_authorization_ref"],
            now_epoch=self.now)
        args.update(overrides)
        return verify_quorum_checkpoint_chain(self.db_path, self.chain,
                                              self.registry, **args)

    def rejected(self, message, **overrides):
        with self.assertRaisesRegex(PreflightRejected, message):
            self.verify(**overrides)

    def test_quorum_genesis_two_of_three_signed_hold(self):
        self.first()
        msg = self.verify()
        self.assertIn("G53_TEST_FULL_QUORUM_CHECKPOINT_CHAIN=PASS", msg)
        self.assertIn("G53_EXTERNAL_LATEST_HEAD_AUTHENTICITY=NOT_PROVEN", msg)
        self.assertIn("G53_GO_NO_GO=HOLD", msg)

    def test_quorum_two_entries_and_three_signatures(self):
        self.first()
        self.second_claim()
        self.append(signers=self.reviewers)
        self.assertIn("G53_TEST_QUORUM_SIGNATURES_AND_LINKS=PASS", self.verify())

    def test_quorum_chain_two_entries_and_two_signatures(self):
        self.first()
        self.second()
        self.assertIn("G53_TEST_FULL_QUORUM_CHECKPOINT_CHAIN=PASS", self.verify())

    def test_quorum_insufficient_signatures(self):
        e = self.first()
        e["endorsements"] = e["endorsements"][:1]
        self.rejected("INSUFFICIENT_OR_EXCESS_SIGNERS")

    def test_quorum_duplicate_signers(self):
        e = self.first()
        e["endorsements"][1] = dict(e["endorsements"][0])
        self.rejected("DUPLICATE_OR_UNSORTED_SIGNERS")

    def test_quorum_invalid_signature(self):
        e = self.first()
        e["endorsements"][0]["signature_hex"] = "0" * 128
        self.rejected("SIGNATURE_OR_KEY_INVALID")

    def test_quorum_key_substitution(self):
        self.first()
        keys = dict(self.review_pubs)
        keys[self.reviewers[0]] = self.review_pubs[self.reviewers[1]]
        self.rejected("REVIEWER_KEY_NOT_PINNED", reviewer_public_pems=keys)

    def test_quorum_forged_registry(self):
        self.first()
        self.registry["required_quorum"] = 3
        self.rejected("UNAPPROVED_REGISTRY_DIGEST")

    def test_quorum_noninteger_quorum(self):
        self.first()
        self.registry["required_quorum"] = True
        self.registry_pin = sha256(canonical(self.registry))
        self.rejected("INVALID_QUORUM")

    def test_quorum_missing_reviewer(self):
        self.first()
        keys = dict(self.review_pubs)
        del keys[self.reviewers[0]]
        self.rejected("MISSING_OR_EXTRA_REVIEWER_KEY", reviewer_public_pems=keys)

    def test_quorum_old_valid_chain_against_newer_pinned_sequence(self):
        self.first()
        old = copy.deepcopy(self.chain)
        self.second()
        self.chain = old
        self.rejected("EXTERNAL_SEQUENCE_MISMATCH")

    def test_quorum_reordered_history(self):
        self.first()
        self.second()
        self.chain.reverse()
        self.rejected("SEQUENCE_GAP_OR_REORDER")

    def test_quorum_deleted_first_history_item(self):
        self.first()
        self.second()
        self.chain = self.chain[1:]
        self.latest_seq = 1
        self.rejected("SEQUENCE_GAP_OR_REORDER")

    def test_quorum_broken_predecessor(self):
        self.first()
        e = self.second()
        e["statement"]["previous_head_sha256"] = "f" * 64
        self.rejected("BROKEN_PREDECESSOR_OR_FORK")

    def test_quorum_validly_resigned_fork_does_not_match_external_head(self):
        self.first()
        e = self.second()
        e["statement"]["issued_at_epoch"] += 1
        e["endorsements"] = self.sign_statement(e["statement"])
        self.rejected("EXTERNAL_LATEST_HEAD_MISMATCH")

    def test_quorum_modified_checkpoint_resigned(self):
        e = self.first()
        e["statement"]["checkpoint"]["row_count"] = 0
        e["endorsements"] = self.sign_statement(e["statement"])
        self.latest_head = entry_head(e)
        self.rejected("ROLLBACK_OR_COUNT_CHANGE")

    def test_quorum_older_sqlite_file_rejected(self):
        self.first()
        old = self.root / "older.test.sqlite"
        shutil.copy2(self.db_path, old)
        self.second()
        self.assertIn("G53_TEST_FULL_QUORUM_CHECKPOINT_CHAIN=PASS", self.verify())
        shutil.copy2(old, self.db_path)
        self.rejected("ROLLBACK_OR_COUNT_CHANGE")

    def test_quorum_sequence_gap(self):
        e = self.first()
        e["statement"]["sequence"] = 2
        self.rejected("SEQUENCE_GAP_OR_REORDER")

    def test_quorum_row_count_regression(self):
        self.first()
        self.second_claim()
        self.append(checkpoint=dict(self.checkpoint(), row_count=0))
        self.rejected("ROW_COUNT_REGRESSION")

    def test_quorum_future_timestamp(self):
        e = self.first()
        e["statement"]["issued_at_epoch"] = self.now + 500
        self.rejected("INVALID_OR_REVERSED_TIME")

    def test_quorum_invalid_source(self):
        e = self.first()
        e["statement"]["approved_test_source_sha256"] = "f" * 64
        self.rejected("SOURCE_MISMATCH")

    def test_quorum_external_head_forged(self):
        self.first()
        self.rejected("EXTERNAL_LATEST_HEAD_MISMATCH",
                      externally_pinned_latest_head_sha256="a" * 64)

    def test_quorum_missing_external_head(self):
        self.first()
        self.rejected("LATEST_HEAD", externally_pinned_latest_head_sha256="")

    def test_quorum_boolean_sequence(self):
        e = self.first()
        e["statement"]["sequence"] = True
        self.rejected("SEQUENCE_GAP_OR_REORDER")

    def test_quorum_empty_chain(self):
        self.rejected("EMPTY_OR_TOO_LONG", externally_pinned_latest_sequence=0)

    def test_quorum_unexpected_statement_field(self):
        e = self.first()
        e["statement"]["promote_to_real"] = True
        self.rejected("KEY_SET_MISMATCH")

    def test_quorum_missing_sqlite_file(self):
        self.first()
        self.db_path.unlink()
        self.rejected("STORE_MISSING")


def load_tests(loader, tests, pattern):
    suite = unittest.TestSuite()
    for method in sorted(x for x in QuorumChainTest.__dict__
                         if x.startswith("test_quorum_")):
        suite.addTest(QuorumChainTest(method))
    return suite


if __name__ == "__main__":
    unittest.main()
