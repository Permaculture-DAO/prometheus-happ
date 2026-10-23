"""G5.3 documentary admission gate: external facts MUST remain unproven."""
import copy
import json
import subprocess
import sys
import unittest
from pathlib import Path

from g53_attestation import canonical, sha256
from g53_preflight import PreflightRejected
from g53_real_evidence_gate import (
    EXPECTED_GATES, inspect_register, inspect_register_file,
)

FIXTURE = Path(__file__).with_name("G53_EXTERNAL_EVIDENCE_REGISTER_v1.json")


class RealEvidenceGateTest(unittest.TestCase):
    def setUp(self):
        self.data = json.loads(FIXTURE.read_text(encoding="utf-8"))
        self.now = 1810000000

    def check(self, *, record=None, digest=None):
        if record is None:
            record = self.data
        if digest is None:
            digest = sha256(canonical(record))
        return inspect_register(record, externally_supplied_digest=digest,
                                now_epoch=self.now)

    def rejects(self, marker, *, record=None, digest=None):
        with self.assertRaisesRegex(PreflightRejected, marker):
            self.check(record=record, digest=digest)

    def one_claim(self, gate_index=0, status="INDEPENDENT_REVIEW_CLAIMED"):
        row = self.data["gates"][gate_index]
        row["status"] = status
        row["independently_observed_source_uri"] = (
            "https://independently-claimed.example/receipts/" + row["id"])
        row["raw_evidence_sha256"] = "a" * 64
        if status == "INDEPENDENT_REVIEW_CLAIMED":
            row["independent_observer_ref"] = "UNVERIFIED-reviewer-123"
            row["reviewed_at_utc"] = "2026-10-23T00:00:00Z"
            row["expiry_utc"] = "2028-01-01T00:00:00Z"
            row["reviewer_decision_ref"] = "UNVERIFIED-decision-123"

    def all_claimed(self):
        for n in range(len(self.data["gates"])):
            self.one_claim(n)

    def test_real_baseline_18_missing_and_physical_hold(self):
        report = self.check()
        self.assertEqual(len(EXPECTED_GATES), 18)
        self.assertEqual(report["counts"]["NOT_OBSERVED"], 18)
        self.assertEqual(report["documentary_status"], "INCOMPLETE")
        self.assertEqual(report["physical_G53"], "HOLD_PENDING_INDEPENDENT_REVIEW")
        self.assertEqual(report["real_data_persistence"], "CLOSED")
        self.assertFalse(report["automatic_go"])

    def test_real_all_18_fake_review_claims_never_upgrade_to_go(self):
        self.all_claimed()
        report = self.check()
        self.assertEqual(report["counts"]["INDEPENDENT_REVIEW_CLAIMED"], 18)
        self.assertEqual(report["documentary_status"],
                         "CLAIMED_COMPLETE_UNAUTHENTICATED")
        self.assertEqual(report["external_pin_provenance"], "NOT_AUTHENTICATED")
        self.assertEqual(report["independent_publication_retrieval"], "NOT_PERFORMED")
        self.assertEqual(report["physical_G53"], "HOLD_PENDING_INDEPENDENT_REVIEW")
        self.assertFalse(report["automatic_go"])

    def test_real_mutated_record_against_external_sha_fails(self):
        genuine = sha256(canonical(self.data))
        self.data["gates"][0]["notes"] = "Altered after approval"
        self.rejects("EXTERNAL_DIGEST_MISMATCH", digest=genuine)

    def test_real_wrong_external_digest_fails(self):
        self.rejects("EXTERNAL_DIGEST_MISMATCH", digest="f" * 64)

    def test_real_extra_root_go_flag_rejected(self):
        self.data["physical_go"] = True
        self.rejects("KEY_SET_MISMATCH")

    def test_real_auto_go_switch_rejected(self):
        self.data["automatic_go_permitted"] = True
        self.rejects("UNKNOWN_OR_AUTOGO_BOUNDARY_BREACH")

    def test_real_unknown_zero_switch_rejected(self):
        self.data["unknown_is_not_proven"] = False
        self.rejects("UNKNOWN_OR_AUTOGO_BOUNDARY_BREACH")

    def test_real_release_prod_rejected(self):
        self.data["release"] = "PRODUCTION_RELEASED"
        self.rejects("FORBIDDEN_RELEASE_PROMOTION")

    def test_real_physical_go_decision_rejected(self):
        self.data["final_decision"]["physical_G53"] = "GO"
        self.rejects("PHYSICAL_OR_REAL_PROMOTION_FORBIDDEN")

    def test_real_g54_promotion_rejected(self):
        self.data["final_decision"]["partition_G54"] = "PASS"
        self.rejects("PHYSICAL_OR_REAL_PROMOTION_FORBIDDEN")

    def test_real_persistence_open_rejected(self):
        self.data["final_decision"]["real_data_persistence"] = "OPEN"
        self.rejects("PHYSICAL_OR_REAL_PROMOTION_FORBIDDEN")

    def test_real_authority_lattice_replace_rejected(self):
        self.data["authority_lattice_reference"] = "UNAPPROVED.yaml"
        self.rejects("AUTHORITY_REFERENCE")

    def test_real_empty_gate_register_rejected(self):
        self.data["gates"] = []
        self.rejects("REQUIRED_GATES_COUNT")

    def test_real_duplicate_gate_rejected(self):
        self.data["gates"][1]["id"] = self.data["gates"][0]["id"]
        self.rejects("ORDER_DUPLICATE_OR_MISSING_GATE")

    def test_real_reordered_evidence_rows_rejected(self):
        self.data["gates"].reverse()
        self.rejects("ORDER_DUPLICATE_OR_MISSING_GATE")

    def test_real_invalid_claim_status_rejected(self):
        self.data["gates"][0]["status"] = "APPROVED_GO"
        self.rejects("UNKNOWN_STATUS")

    def test_real_unknown_unobserved_claim_conflict_rejected(self):
        self.data["gates"][0]["raw_evidence_sha256"] = "a" * 64
        self.rejects("UNOBSERVED_HAS_EVIDENCE_CLAIMS")

    def test_real_unverified_submission_remains_hold(self):
        self.one_claim(status="EVIDENCE_SUBMITTED_UNVERIFIED")
        report = self.check()
        self.assertEqual(report["counts"]["EVIDENCE_SUBMITTED_UNVERIFIED"], 1)
        self.assertEqual(report["physical_G53"], "HOLD_PENDING_INDEPENDENT_REVIEW")

    def test_real_unverified_submission_cannot_add_fake_approval(self):
        self.one_claim(status="EVIDENCE_SUBMITTED_UNVERIFIED")
        self.data["gates"][0]["reviewer_decision_ref"] = "FAKE-APPROVAL"
        self.rejects("UNVERIFIED_HAS_APPROVAL_CLAIMS")

    def test_real_review_claim_missing_original_hash(self):
        self.one_claim()
        self.data["gates"][0]["raw_evidence_sha256"] = None
        self.rejects("REVIEWED_SHA")

    def test_real_review_claim_requires_https(self):
        self.one_claim()
        self.data["gates"][0]["independently_observed_source_uri"] = (
            "http://example.org/claim")
        self.rejects("URI_MUST_BE_PLAIN_HTTPS_NO_CREDENTIALS")

    def test_real_review_claim_bans_tokenized_url(self):
        self.one_claim()
        self.data["gates"][0]["independently_observed_source_uri"] = (
            "https://example.org/claim?secret=bad")
        self.rejects("URI_MUST_BE_PLAIN_HTTPS_NO_CREDENTIALS")

    def test_real_review_claim_future_observation_rejected(self):
        self.one_claim()
        self.data["gates"][0]["reviewed_at_utc"] = "2032-01-01T00:00:00Z"
        self.data["gates"][0]["expiry_utc"] = "2033-01-01T00:00:00Z"
        self.rejects("REVIEW_TIME_ORDER")

    def test_real_expired_claim_not_documentary_complete(self):
        self.all_claimed()
        self.data["gates"][0]["expiry_utc"] = "2026-10-24T00:00:00Z"
        report = self.check()
        self.assertEqual(report["expired_review_claims"], 1)
        self.assertEqual(report["documentary_status"], "INCOMPLETE")
        self.assertEqual(report["physical_G53"], "HOLD_PENDING_INDEPENDENT_REVIEW")

    def test_real_expiry_before_review_rejected(self):
        self.one_claim()
        self.data["gates"][0]["expiry_utc"] = "2026-10-22T00:00:00Z"
        self.rejects("REVIEW_TIME_ORDER")

    def test_real_review_claim_boolean_evidence_not_accepted(self):
        self.one_claim()
        self.data["gates"][0]["reviewer_decision_ref"] = True
        self.rejects("DECISION:INVALID_REFERENCE")

    def test_real_file_fixture_consistent_and_hold(self):
        report = inspect_register_file(
            FIXTURE, externally_supplied_digest=sha256(canonical(self.data)),
            now_epoch=self.now)
        self.assertEqual(report["counts"]["NOT_OBSERVED"], 18)
        self.assertEqual(report["physical_G53"], "HOLD_PENDING_INDEPENDENT_REVIEW")

    def test_real_cli_outputs_machine_hold_and_exit_zero(self):
        p = subprocess.run([
            sys.executable, str(Path(__file__).with_name("g53_real_evidence_gate.py")),
            str(FIXTURE), "--expected-sha256",
            sha256(canonical(self.data)),
        ], capture_output=True, text=True, check=True)
        out = json.loads(p.stdout)
        self.assertFalse(out["automatic_go"])
        self.assertEqual(out["physical_G53"], "HOLD_PENDING_INDEPENDENT_REVIEW")

    def test_real_cli_wrong_digest_fails_closed(self):
        p = subprocess.run([
            sys.executable, str(Path(__file__).with_name("g53_real_evidence_gate.py")),
            str(FIXTURE), "--expected-sha256", "0" * 64,
        ], capture_output=True, text=True)
        self.assertEqual(p.returncode, 2)
        self.assertIn("EXTERNAL_DIGEST_MISMATCH", p.stderr)


if __name__ == "__main__":
    unittest.main()
