"""Synthetic custody dossier packet tests: no physical or human trust assertions."""
import copy
import json
import unittest
from pathlib import Path

from g53_attestation import canonical, sha256
from g53_custody_preclearance import (
    SCHEMA, ENROLLMENT_SCHEMA, PUBLICATION_SCHEMA, verify_custody_preclearance,
)
from g53_preflight import PreflightRejected
from test_g53_external_witness import ExternalWitnessTest, SOURCE


class CustodyPreclearanceTest(ExternalWitnessTest):
    def setUp(self):
        super().setUp()
        self.dossier = {
            "schema": SCHEMA,
            "scope": "G5.3_TEST_ONLY",
            "source_sha256": SOURCE,
            "authorization_ref": self.policy["review_authorization_ref"],
            "witness_receipt_head_sha256": self.external_witness_head,
            "witness_round": self.statement["witness_round"],
            "session_nonce_hex": self.statement["session_nonce_hex"],
            "custodians": [],
        }
        self.evidence_files = {}
        for index, fingerprint in enumerate(self.witness_pins):
            name = "synthetic-custodian-" + str(index+1)
            fields = {
                "custodian_ref": name,
                "organization_ref": "fictional-org-" + str(index+1),
                "administrator_domain_ref": "fictional-admin-" + str(index+1),
                "failure_domain_ref": "fictional-failure-" + str(index+1),
                "custody_domain_ref": "fictional-custody-" + str(index+1),
                "publication_channel_ref": "fictional-publication-" + str(index+1),
                "enrollment_authority_ref": "fictional-authority-" + str(index+1),
                "reviewer_approval_ref": "fictional-approval-" + str(index+1),
                "witness_pubkey_spki_sha256": fingerprint,
            }
            enrollment = {
                "schema": ENROLLMENT_SCHEMA,
                **{key:fields[key] for key in (
                    "custodian_ref", "organization_ref",
                    "administrator_domain_ref", "failure_domain_ref",
                    "custody_domain_ref", "enrollment_authority_ref",
                    "reviewer_approval_ref", "witness_pubkey_spki_sha256")},
                "session_nonce_hex": self.statement["session_nonce_hex"],
                "source_sha256": SOURCE,
            }
            publication = {
                "schema": PUBLICATION_SCHEMA,
                "custodian_ref": name,
                "witness_pubkey_spki_sha256": fingerprint,
                "witness_receipt_head_sha256": self.external_witness_head,
                "witness_round": self.statement["witness_round"],
                "session_nonce_hex": self.statement["session_nonce_hex"],
                "source_sha256": SOURCE,
                "publication_channel_ref": fields["publication_channel_ref"],
            }
            a = self.root / (name + "-enrollment.json")
            b = self.root / (name + "-publication.json")
            self.write_bytes(a, enrollment)
            self.write_bytes(b, publication)
            fields["enrollment_evidence_sha256"] = sha256(a.read_bytes())
            fields["publication_evidence_sha256"] = sha256(b.read_bytes())
            self.dossier["custodians"].append(fields)
            self.evidence_files[name] = {"enrollment":a, "publication":b}
        self.dossier_pin = sha256(canonical(self.dossier))

    def write_bytes(self, path, value):
        path.write_bytes(canonical(value))

    def repin(self):
        self.dossier_pin = sha256(canonical(self.dossier))

    def check_packet(self, **args):
        params = {
            "custodian_evidence_files": self.evidence_files,
            "externally_approved_dossier_sha256": self.dossier_pin,
            "root_public_pems": self.root_pems,
            "reviewer_public_pems": self.review_pubs,
            "witness_public_pems": self.witness_pubs,
            "externally_pinned_genesis_root_fingerprints": self.old_root_set,
            "externally_pinned_latest_root_fingerprints": self.new_root_set,
            "externally_pinned_witness_fingerprints": self.witness_pins,
            "externally_pinned_latest_witness_head_sha256": self.external_witness_head,
            "externally_pinned_previous_witness_head_sha256": "0"*64,
            "externally_pinned_latest_witness_round": 1,
            "independently_approved_source_sha256": SOURCE,
            "independently_approved_authorization_ref":
                self.policy["review_authorization_ref"],
            "now_epoch": self.now,
        }
        params.update(args)
        return verify_custody_preclearance(
            self.db_path, self.epochs, self.rotations, self.receipt,
            self.dossier, **params)

    def rejects_packet(self, code, **args):
        with self.assertRaisesRegex(PreflightRejected, code):
            self.check_packet(**args)

    def modify_evidence(self, index, kind, field, value):
        entry = self.dossier["custodians"][index]
        path = self.evidence_files[entry["custodian_ref"]][kind]
        data = json.loads(path.read_text())
        data[field] = value
        self.write_bytes(path, data)
        entry[kind + "_evidence_sha256"] = sha256(path.read_bytes())
        self.repin()

    def test_custody_two_fixture_records_pass_but_physical_hold(self):
        output = self.check_packet()
        self.assertIn("G53_TEST_CUSTODY_PACKET_BYTE_BINDINGS=PASS", output)
        self.assertIn("G53_REAL_CUSTODIAN_IDENTITY_AUTHENTICATION=NOT_PROVEN", output)
        self.assertIn("G53_INDEPENDENT_ADMIN_AND_FAILURE_DOMAINS=NOT_PROVEN", output)
        self.assertIn("G53_GO_NO_GO=HOLD_PENDING_INDEPENDENT_REVIEW", output)

    def test_custody_changed_evidence_bytes_without_updated_pin(self):
        p = self.evidence_files[self.dossier["custodians"][0]["custodian_ref"]]["enrollment"]
        p.write_bytes(p.read_bytes()+b" ")
        self.rejects_packet("ENROLLMENT:BYTE_DIGEST_MISMATCH")

    def test_custody_missing_publication_file(self):
        p = self.evidence_files[self.dossier["custodians"][1]["custodian_ref"]]["publication"]
        p.unlink()
        self.rejects_packet("MISSING_OR_TOO_LARGE")

    def test_custody_duplicate_admin_control_label(self):
        self.dossier["custodians"][1]["administrator_domain_ref"] = (
            self.dossier["custodians"][0]["administrator_domain_ref"])
        self.repin()
        self.rejects_packet("CLAIMED_DOMAIN_OR_KEY_COLLISION:administrator_domain_ref")

    def test_custody_duplicate_failure_domain_label(self):
        self.dossier["custodians"][1]["failure_domain_ref"] = (
            self.dossier["custodians"][0]["failure_domain_ref"])
        self.repin()
        self.rejects_packet("CLAIMED_DOMAIN_OR_KEY_COLLISION:failure_domain_ref")

    def test_custody_same_publication_channel_ref(self):
        self.dossier["custodians"][1]["publication_channel_ref"] = (
            self.dossier["custodians"][0]["publication_channel_ref"])
        self.repin()
        self.rejects_packet("CLAIMED_DOMAIN_OR_KEY_COLLISION:publication_channel_ref")

    def test_custody_same_organization_ref(self):
        self.dossier["custodians"][1]["organization_ref"] = (
            self.dossier["custodians"][0]["organization_ref"])
        self.repin()
        self.rejects_packet("CLAIMED_DOMAIN_OR_KEY_COLLISION:organization_ref")

    def test_custody_same_enrollment_authority_ref(self):
        self.dossier["custodians"][1]["enrollment_authority_ref"] = (
            self.dossier["custodians"][0]["enrollment_authority_ref"])
        self.repin()
        self.rejects_packet("CLAIMED_DOMAIN_OR_KEY_COLLISION:enrollment_authority_ref")

    def test_custody_wrong_witness_key_in_claim(self):
        self.dossier["custodians"][0]["witness_pubkey_spki_sha256"] = "f"*64
        self.repin()
        self.rejects_packet("SIGNER_TO_CUSTODIAN_KEY_MISMATCH")

    def test_custody_dossier_extra_go_flag_denied(self):
        self.dossier["physical_go"] = True
        self.repin()
        self.rejects_packet("KEY_SET_MISMATCH")

    def test_custody_dossier_scope_cannot_promote_to_real(self):
        self.dossier["scope"] = "REAL"
        self.repin()
        self.rejects_packet("PRODUCTION_SCOPE_FORBIDDEN")

    def test_custody_wrong_out_of_band_dossier_digest(self):
        self.rejects_packet("EXTERNAL_DOSSIER_MISMATCH",
                            externally_approved_dossier_sha256="f"*64)

    def test_custody_custodian_record_claim_not_matching_dossier(self):
        self.modify_evidence(0, "enrollment", "organization_ref", "fictional-unrelated")
        self.rejects_packet("ENROLLMENT_CLAIM_MISMATCH:organization_ref")

    def test_custody_publication_record_points_to_different_head(self):
        self.modify_evidence(0, "publication",
                             "witness_receipt_head_sha256", "f"*64)
        self.rejects_packet("PUBLICATION_COMMITMENT_MISMATCH")

    def test_custody_publication_session_mismatch(self):
        self.modify_evidence(1, "publication", "session_nonce_hex", "b3"*32)
        self.rejects_packet("PUBLICATION_COMMITMENT_MISMATCH")

    def test_custody_enrollment_session_mismatch(self):
        self.modify_evidence(0, "enrollment", "session_nonce_hex", "b3"*32)
        self.rejects_packet("ENROLLMENT_SESSION_OR_SOURCE_MISMATCH")

    def test_custody_publication_wrong_channel(self):
        self.modify_evidence(0, "publication", "publication_channel_ref", "fictional-other")
        self.rejects_packet("PUBLICATION_COMMITMENT_MISMATCH")

    def test_custody_missing_evidence_key(self):
        changed = {k: v for k, v in list(self.evidence_files.items())[:1]}
        self.rejects_packet("MISSING_OR_EXTRA_FILES",
                            custodian_evidence_files=changed)

    def test_custody_order_swapped(self):
        self.dossier["custodians"].reverse()
        self.repin()
        self.rejects_packet("CUSTODIANS_NOT_SORTED")

    def test_custody_wrong_receipt_round(self):
        self.dossier["witness_round"] = 2
        self.repin()
        self.rejects_packet("WITNESS_ROUND_MISMATCH")

    def test_custody_wrong_original_receipt_head(self):
        self.dossier["witness_receipt_head_sha256"] = "f"*64
        self.repin()
        self.rejects_packet("WITNESS_HEAD_MISMATCH")

    def test_custody_malformed_enrollment_json(self):
        path = self.evidence_files[self.dossier["custodians"][0]["custodian_ref"]]["enrollment"]
        path.write_bytes(b'{"schema":')
        self.dossier["custodians"][0]["enrollment_evidence_sha256"] = sha256(path.read_bytes())
        self.repin()
        with self.assertRaises((ValueError, PreflightRejected, json.JSONDecodeError)):
            self.check_packet()

    def test_custody_json_duplicate_keys_fails_closed(self):
        path = self.evidence_files[self.dossier["custodians"][0]["custodian_ref"]]["publication"]
        path.write_bytes(b'{"schema":"x","schema":"y"}')
        self.dossier["custodians"][0]["publication_evidence_sha256"] = sha256(path.read_bytes())
        self.repin()
        self.rejects_packet("DUPLICATE_JSON_KEY")

    def test_custody_missing_first_custodian_denied(self):
        self.dossier["custodians"] = self.dossier["custodians"][1:]
        self.repin()
        self.rejects_packet("EXACTLY_TWO_CUSTODIANS_REQUIRED")

    def test_custody_original_receipt_invalid_signature_denied(self):
        self.receipt["endorsements"][0]["signature_hex"] = "0"*128
        self.external_witness_head = sha256(canonical(self.receipt))
        self.rejects_packet("WITNESS_HEAD_MISMATCH")

    def test_custody_status_is_not_a_go_vote(self):
        out = self.check_packet()
        self.assertNotIn("G53_GO_NO_GO=GO", out)
        self.assertIn("G53_PHYSICAL_CROSS_HOST_REPLICATION=NOT_TESTED", out)


def load_tests(loader, original_tests, pattern):
    suite = unittest.TestSuite()
    for test in sorted(name for name in CustodyPreclearanceTest.__dict__
                       if name.startswith("test_custody_")):
        suite.addTest(CustodyPreclearanceTest(test))
    return suite


if __name__ == "__main__":
    unittest.main()
