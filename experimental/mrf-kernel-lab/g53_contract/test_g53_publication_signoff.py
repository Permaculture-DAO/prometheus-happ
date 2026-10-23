"""Synthetic signed-publication preflight; no real external retrieval/custodians."""
import copy
import json
import subprocess
import unittest
from pathlib import Path

from g53_attestation import canonical, sha256
from g53_preflight import PreflightRejected
from g53_publication_signoff import (
    GENESIS, SCHEMA, publication_head, publication_message,
    verify_custodian_publications,
)
from test_g53_custody_preclearance import CustodyPreclearanceTest


class PublicationSignoffTest(CustodyPreclearanceTest):
    def setUp(self):
        super().setUp()
        self.publication_files = {}
        self.pub_heads, self.pub_sequences, self.pub_previous = {}, {}, {}
        for item in self.dossier["custodians"]:
            custodian = item["custodian_ref"]
            statement = {
                "schema": SCHEMA,
                "custodian_ref": custodian,
                "witness_pubkey_spki_sha256": item["witness_pubkey_spki_sha256"],
                "publication_channel_ref": item["publication_channel_ref"],
                "publication_evidence_sha256": item["publication_evidence_sha256"],
                "witness_receipt_head_sha256": self.external_witness_head,
                "witness_round": self.statement["witness_round"],
                "session_nonce_hex": self.statement["session_nonce_hex"],
                "publication_sequence": 1,
                "previous_publication_head_sha256": GENESIS,
                "issued_at_epoch": self.now,
            }
            path = self.root / (custodian + "-signed-publication.json")
            self.sign_publication(statement, path)
            self.publication_files[custodian] = path
            self.pub_heads[custodian] = publication_head(json.loads(path.read_text()))
            self.pub_sequences[custodian] = 1
            self.pub_previous[custodian] = GENESIS

    def sign_publication(self, statement, path, signing_key=None):
        if signing_key is None:
            signing_key = statement["witness_pubkey_spki_sha256"]
        msg = self.root / "publication-message.test"
        sig = self.root / "publication-signature.test"
        msg.write_bytes(publication_message(statement))
        subprocess.run([
            "openssl", "pkeyutl", "-sign", "-inkey",
            str(self.witness_keys[signing_key]), "-rawin",
            "-in", str(msg), "-out", str(sig),
        ], capture_output=True, check=True)
        signed = {"statement": statement, "signature_hex": sig.read_bytes().hex()}
        path.write_bytes(canonical(signed))
        return signed

    def mutate_and_sign(self, index, field, value, signer=None):
        custodian = self.dossier["custodians"][index]["custodian_ref"]
        path = self.publication_files[custodian]
        signed = json.loads(path.read_bytes())
        signed["statement"][field] = value
        signed = self.sign_publication(signed["statement"], path, signer)
        self.pub_heads[custodian] = publication_head(signed)

    def verify_publication(self, **changes):
        params = {
            "custodian_evidence_files": self.evidence_files,
            "signed_publication_files": self.publication_files,
            "externally_pinned_publication_heads": self.pub_heads,
            "externally_pinned_publication_sequences": self.pub_sequences,
            "externally_pinned_previous_publication_heads": self.pub_previous,
            "externally_approved_dossier_sha256": self.dossier_pin,
            "root_public_pems": self.root_pems,
            "reviewer_public_pems": self.review_pubs,
            "witness_public_pems": self.witness_pubs,
            "externally_pinned_genesis_root_fingerprints": self.old_root_set,
            "externally_pinned_latest_root_fingerprints": self.new_root_set,
            "externally_pinned_witness_fingerprints": self.witness_pins,
            "externally_pinned_latest_witness_head_sha256": self.external_witness_head,
            "externally_pinned_previous_witness_head_sha256": GENESIS,
            "externally_pinned_latest_witness_round": 1,
            "independently_approved_source_sha256": "c"*64,
            "independently_approved_authorization_ref":
                self.policy["review_authorization_ref"],
            "now_epoch": self.now,
        }
        params.update(changes)
        return verify_custodian_publications(
            self.db_path, self.epochs, self.rotations, self.receipt,
            self.dossier, **params)

    def rejected(self, reason, **changes):
        with self.assertRaisesRegex(PreflightRejected, reason):
            self.verify_publication(**changes)

    def test_signoff_two_synthetic_publications_still_hold(self):
        output = self.verify_publication()
        self.assertIn("G53_TEST_TWO_SIGNED_PUBLICATIONS=PASS", output)
        self.assertIn("G53_EXTERNAL_PUBLICATION_RETRIEVAL=NOT_PERFORMED", output)
        self.assertIn("G53_GO_NO_GO=HOLD_PENDING_INDEPENDENT_REVIEW", output)

    def test_signoff_changed_file_without_changing_outside_pin(self):
        path = next(iter(self.publication_files.values()))
        path.write_bytes(path.read_bytes()+b" ")
        self.rejected("NON_CANONICAL_BYTES")

    def test_signoff_signature_corrupted_and_rehashed(self):
        custodian = self.dossier["custodians"][0]["custodian_ref"]
        path = self.publication_files[custodian]
        signed = json.loads(path.read_text())
        signed["signature_hex"] = "0"*128
        path.write_bytes(canonical(signed))
        self.pub_heads[custodian] = publication_head(signed)
        self.rejected("SIGNATURE_OR_KEY_INVALID")

    def test_signoff_signature_swapped_and_rehashed(self):
        ids = list(self.publication_files)
        file_a = self.publication_files[ids[0]]
        file_b = self.publication_files[ids[1]]
        a, b = json.loads(file_a.read_text()), json.loads(file_b.read_text())
        a["signature_hex"] = b["signature_hex"]
        file_a.write_bytes(canonical(a))
        self.pub_heads[ids[0]] = publication_head(a)
        self.rejected("SIGNATURE_OR_KEY_INVALID")

    def test_signoff_missing_second_publication_file(self):
        custodian = list(self.publication_files)[1]
        self.publication_files[custodian].unlink()
        self.rejected("MISSING_OR_TOO_LARGE")

    def test_signoff_missing_second_external_pin(self):
        fewer = dict(list(self.pub_heads.items())[:1])
        self.rejected("EXTERNAL_HEADS_INCOMPLETE",
                      externally_pinned_publication_heads=fewer)

    def test_signoff_identical_external_heads_refused(self):
        refs = list(self.pub_heads)
        changed = dict(self.pub_heads)
        changed[refs[1]] = changed[refs[0]]
        self.rejected("EXTERNAL_HEAD_COLLISION",
                      externally_pinned_publication_heads=changed)

    def test_signoff_wrong_current_external_head(self):
        custodian = list(self.pub_heads)[0]
        h = dict(self.pub_heads)
        h[custodian] = "f"*64
        self.rejected("EXTERNAL_HEAD_MISMATCH",
                      externally_pinned_publication_heads=h)

    def test_signoff_wrong_publication_evidence_digest(self):
        self.mutate_and_sign(0, "publication_evidence_sha256", "f"*64)
        self.rejected("SIGNED_CLAIM_MISMATCH:publication_evidence_sha256")

    def test_signoff_wrong_signed_channel(self):
        self.mutate_and_sign(0, "publication_channel_ref", "fictional-channel-rogue")
        self.rejected("SIGNED_CLAIM_MISMATCH:publication_channel_ref")

    def test_signoff_wrong_signed_nonce(self):
        self.mutate_and_sign(0, "session_nonce_hex", "a1"*32)
        self.rejected("SIGNED_CLAIM_MISMATCH:session_nonce_hex")

    def test_signoff_wrong_witness_head(self):
        self.mutate_and_sign(0, "witness_receipt_head_sha256", "f"*64)
        self.rejected("SIGNED_CLAIM_MISMATCH:witness_receipt_head_sha256")

    def test_signoff_wrong_witness_round(self):
        self.mutate_and_sign(0, "witness_round", 2)
        self.rejected("SIGNED_CLAIM_MISMATCH:witness_round")

    def test_signoff_wrong_custodian_claim(self):
        self.mutate_and_sign(0, "custodian_ref", "fictional-unrelated-custodian")
        self.rejected("SIGNED_CLAIM_MISMATCH:custodian_ref")

    def test_signoff_wrong_signing_witness_claim(self):
        self.mutate_and_sign(0, "witness_pubkey_spki_sha256", "f"*64,
                             signer=self.witness_pins[0])
        self.rejected("SIGNED_CLAIM_MISMATCH:witness_pubkey_spki_sha256")

    def test_signoff_boolean_publication_sequence(self):
        self.mutate_and_sign(0, "publication_sequence", True)
        self.rejected("SEQUENCE_NOT_PINNED")

    def test_signoff_explicit_wrong_pinned_sequence(self):
        key = list(self.pub_sequences)[0]
        seqs = dict(self.pub_sequences)
        seqs[key] = 2
        self.rejected("SEQUENCE_NOT_PINNED",
                      externally_pinned_publication_sequences=seqs)

    def test_signoff_second_sequence_requires_real_predecessor(self):
        self.mutate_and_sign(0, "publication_sequence", 2)
        key = self.dossier["custodians"][0]["custodian_ref"]
        self.pub_sequences[key] = 2
        self.rejected("INVALID_GENESIS_LINK")

    def test_signoff_wrong_previous_publication_head(self):
        self.mutate_and_sign(0, "previous_publication_head_sha256", "e"*64)
        self.rejected("PREDECESSOR_NOT_PINNED")

    def test_signoff_valid_second_sequence_under_external_previous_pin(self):
        key = self.dossier["custodians"][0]["custodian_ref"]
        prior = self.pub_heads[key]
        self.mutate_and_sign(0, "publication_sequence", 2)
        self.mutate_and_sign(0, "previous_publication_head_sha256", prior)
        self.pub_sequences[key] = 2
        self.pub_previous[key] = prior
        output = self.verify_publication()
        self.assertIn("G53_TEST_PER_CUSTODIAN_PUBLICATION_PINS=PASS", output)
        self.assertIn("G53_EXTERNAL_PUBLICATION_RETRIEVAL=NOT_PERFORMED", output)

    def test_signoff_timestamp_before_session(self):
        self.mutate_and_sign(0, "issued_at_epoch",
                             self.statement["issued_at_epoch"] - 1)
        self.rejected("TIMESTAMP_OUTSIDE_WITNESS_SESSION")

    def test_signoff_timestamp_after_session(self):
        self.mutate_and_sign(0, "issued_at_epoch",
                             self.statement["expires_at_epoch"] + 1)
        self.rejected("TIMESTAMP_OUTSIDE_WITNESS_SESSION")

    def test_signoff_unexpected_go_flag_denied(self):
        custodian = list(self.publication_files)[0]
        path = self.publication_files[custodian]
        signed = json.loads(path.read_text())
        signed["statement"]["physical_go"] = True
        self.sign_publication(signed["statement"], path)
        self.pub_heads[custodian] = publication_head(json.loads(path.read_text()))
        self.rejected("KEY_SET_MISMATCH")

    def test_signoff_noncanonical_external_pin_does_not_validate(self):
        custodian = list(self.pub_heads)[0]
        heads = dict(self.pub_heads)
        heads[custodian] = "INVALID"
        self.rejected("EXTERNAL_HEAD_FORMAT",
                      externally_pinned_publication_heads=heads)

    def test_signoff_missing_test_database_refused(self):
        self.db_path.unlink()
        self.rejected("STORE_MISSING")

    def test_signoff_dossier_wrong_pub_binding_refused(self):
        self.dossier["custodians"][0]["publication_evidence_sha256"] = "f"*64
        self.repin()
        self.rejected("PUBLICATION:BYTE_DIGEST_MISMATCH")

    def test_signoff_malformed_signed_publication_json(self):
        path = next(iter(self.publication_files.values()))
        path.write_bytes(b'{"statement":')
        with self.assertRaises((ValueError, PreflightRejected)):
            self.verify_publication()


def load_tests(loader, discovered, pattern):
    suite = unittest.TestSuite()
    for name in sorted(k for k in PublicationSignoffTest.__dict__
                       if k.startswith("test_signoff_")):
        suite.addTest(PublicationSignoffTest(name))
    return suite


if __name__ == "__main__":
    unittest.main()
