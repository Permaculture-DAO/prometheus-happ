#!/usr/bin/env python3
"""G5.3 external-evidence register inspection. TEST-only; never issues GO.

This is a purely OFFLINE schema/consistency/freshness check. A supplied
"independent review" is a CLAIM and does not authenticate a custodian,
human reviewer, source, separate failure domain or latest state.
"""
import argparse
import datetime as dt
import json
import re
import sys
from pathlib import Path
from urllib.parse import urlsplit

from g53_attestation import canonical, json_file, sha256, strict_file
from g53_preflight import PreflightRejected, keys, need, sha

SCHEMA = "PRM-G53-PHYSICAL-EVIDENCE-REGISTER-v1"
EXPECTED_GATES = tuple("G53-REAL-%03d" % n for n in range(1, 19))
STATUSES = {
    "NOT_OBSERVED", "EVIDENCE_SUBMITTED_UNVERIFIED",
    "INDEPENDENT_REVIEW_CLAIMED", "REJECTED", "EXPIRED",
}
REGISTER_FIELDS = (
    "schema", "date", "status", "release", "project_reference",
    "authority_lattice_reference", "authority_invariants",
    "draft_pull_request", "source_commit_at_creation", "last_verified_ci",
    "ci_status_at_creation", "host_status_at_creation", "custodian_A",
    "custodian_B", "independent_reviewer", "unknown_is_not_proven",
    "automatic_go_permitted", "gates", "final_decision",
)
REQUIRED_ROW_FIELDS = (
    "id", "requirement", "evidence_type", "status",
    "independently_observed_source_uri", "raw_evidence_sha256",
    "independent_observer_ref", "reviewed_at_utc", "expiry_utc",
    "reviewer_decision_ref", "notes",
)
SHA256 = re.compile(r"^[0-9a-f]{64}$")
COMMIT = re.compile(r"^[0-9a-f]{40}$")
DATE = re.compile(r"^\d{4}-\d{2}-\d{2}$")
UTC_TIME = re.compile(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z$")


def _reference(value, marker):
    need(type(value) is str and 4 <= len(value) <= 512 and
         value == value.strip() and
         not any(c in value for c in "\r\n\x00"),
         marker + ":INVALID_REFERENCE")


def _https_uri(value, marker):
    _reference(value, marker)
    parts = urlsplit(value)
    need(parts.scheme == "https" and bool(parts.hostname) and
         parts.username is None and parts.password is None and
         not parts.query and not parts.fragment,
         marker + ":URI_MUST_BE_PLAIN_HTTPS_NO_CREDENTIALS")


def _time(value, marker):
    need(type(value) is str and UTC_TIME.fullmatch(value) is not None,
         marker + ":INVALID_UTC")
    try:
        return dt.datetime.strptime(value, "%Y-%m-%dT%H:%M:%SZ").replace(
            tzinfo=dt.timezone.utc)
    except ValueError as exc:
        raise PreflightRejected(marker + ":INVALID_UTC") from exc


def inspect_register(register, *, externally_supplied_digest, now_epoch=None):
    """Never returns physical-GO, including with 18 apparently reviewed gates."""
    sha(externally_supplied_digest, "REAL_EVIDENCE:EXTERNAL_DIGEST_FORMAT")
    need(type(register) is dict, "REAL_EVIDENCE:NOT_OBJECT")
    keys(register, REGISTER_FIELDS, "REAL_EVIDENCE:ROOT")
    need(register["schema"] == SCHEMA, "REAL_EVIDENCE:SCHEMA")
    need(sha256(canonical(register)) == externally_supplied_digest,
         "REAL_EVIDENCE:EXTERNAL_DIGEST_MISMATCH")
    need(register["status"] == "HOLD_EXTERNAL_EVIDENCE_ABSENT" and
         register["release"] == "TEST_ONLY_NO_PRODUCTION_AUTHORITY",
         "REAL_EVIDENCE:FORBIDDEN_RELEASE_PROMOTION")
    need(register["unknown_is_not_proven"] is True and
         register["automatic_go_permitted"] is False,
         "REAL_EVIDENCE:UNKNOWN_OR_AUTOGO_BOUNDARY_BREACH")
    need(register["authority_invariants"] ==
         ["UNKNOWN != ZERO", "AI_IS_NON_AUTHORITATIVE",
          "TRANSPORT_DOES_NOT_CREATE_AUTHORITY"],
         "REAL_EVIDENCE:AUTHORITY_INVARIANTS")
    need(register["authority_lattice_reference"] ==
         "PROMETHEUS_AUTHORITY_LATTICE_v1_0.yaml",
         "REAL_EVIDENCE:AUTHORITY_REFERENCE")
    need(register["project_reference"] == "PRM-ARCH-DET-002",
         "REAL_EVIDENCE:PROJECT_REFERENCE")
    need(type(register["date"]) is str and
         DATE.fullmatch(register["date"]) is not None,
         "REAL_EVIDENCE:DATE_FORMAT")
    try:
        dt.date.fromisoformat(register["date"])
    except ValueError as exc:
        raise PreflightRejected("REAL_EVIDENCE:INVALID_DATE") from exc
    need(type(register["source_commit_at_creation"]) is str and
         COMMIT.fullmatch(register["source_commit_at_creation"]) is not None,
         "REAL_EVIDENCE:SOURCE_SHA_INVALID")
    for field in ("draft_pull_request", "last_verified_ci"):
        _https_uri(register[field], "REAL_EVIDENCE:" + field)
    need(type(register["ci_status_at_creation"]) is dict and
         type(register["host_status_at_creation"]) is dict,
         "REAL_EVIDENCE:SNAPSHOT_FIELDS_INVALID")
    for field in ("custodian_A", "custodian_B", "independent_reviewer"):
        claim = register[field]
        need(claim is None or
             (type(claim) is str and 4 <= len(claim) <= 256),
             "REAL_EVIDENCE:IDENTITY_CLAIM_MALFORMED")
    need(type(register["gates"]) is list and
         len(register["gates"]) == len(EXPECTED_GATES),
         "REAL_EVIDENCE:REQUIRED_GATES_COUNT")
    decision = register["final_decision"]
    keys(decision, ["physical_G53", "partition_G54", "canonical_mutation",
                    "production_deployment", "real_data_persistence",
                    "reason"], "REAL_EVIDENCE:FINAL_DECISION")
    need(decision["physical_G53"] == "HOLD_PENDING_INDEPENDENT_REVIEW" and
         decision["partition_G54"] == "HOLD_NOT_AUTHORIZED" and
         decision["canonical_mutation"] == "PROHIBITED" and
         decision["production_deployment"] == "PROHIBITED" and
         decision["real_data_persistence"] == "CLOSED",
         "REAL_EVIDENCE:PHYSICAL_OR_REAL_PROMOTION_FORBIDDEN")
    _reference(decision["reason"], "REAL_EVIDENCE:REASON")
    now = dt.datetime.fromtimestamp(now_epoch, tz=dt.timezone.utc) if (
        now_epoch is not None) else dt.datetime.now(dt.timezone.utc)
    counts = {key: 0 for key in STATUSES}
    seen = []
    expired_claims = 0
    for item in register["gates"]:
        keys(item, REQUIRED_ROW_FIELDS, "REAL_EVIDENCE:ROW")
        gate_id = item["id"]
        need(type(gate_id) is str and gate_id in EXPECTED_GATES,
             "REAL_EVIDENCE:UNKNOWN_GATE_ID")
        seen.append(gate_id)
        _reference(item["requirement"], gate_id + ":REQUIREMENT")
        _reference(item["evidence_type"], gate_id + ":EVIDENCE_TYPE")
        _reference(item["notes"], gate_id + ":NOTES")
        status = item["status"]
        need(type(status) is str and status in STATUSES,
             gate_id + ":UNKNOWN_STATUS")
        counts[status] += 1
        evidence = (
            item["independently_observed_source_uri"],
            item["raw_evidence_sha256"],
            item["independent_observer_ref"],
            item["reviewed_at_utc"], item["expiry_utc"],
            item["reviewer_decision_ref"],
        )
        if status == "NOT_OBSERVED":
            need(all(value is None for value in evidence),
                 gate_id + ":UNOBSERVED_HAS_EVIDENCE_CLAIMS")
        elif status == "EVIDENCE_SUBMITTED_UNVERIFIED":
            _https_uri(evidence[0], gate_id + ":SUBMITTED_URI")
            sha(evidence[1], gate_id + ":SUBMITTED_SHA")
            need(all(x is None for x in evidence[2:]),
                 gate_id + ":UNVERIFIED_HAS_APPROVAL_CLAIMS")
        elif status == "INDEPENDENT_REVIEW_CLAIMED":
            _https_uri(evidence[0], gate_id + ":REVIEWED_URI")
            sha(evidence[1], gate_id + ":REVIEWED_SHA")
            _reference(evidence[2], gate_id + ":OBSERVER")
            _reference(evidence[5], gate_id + ":DECISION")
            reviewed = _time(evidence[3], gate_id + ":REVIEWED_TIME")
            expires = _time(evidence[4], gate_id + ":EXPIRY_TIME")
            need(expires > reviewed and reviewed <= now,
                 gate_id + ":REVIEW_TIME_ORDER")
            if expires <= now:
                expired_claims += 1
        else:
            # REJECTED/EXPIRED may retain references for later review, but
            # cannot be represented as an approved current observation.
            for idx, value in enumerate(evidence):
                need(value is None or type(value) is str,
                     gate_id + ":INVALID_REJECTED_EVIDENCE")
    need(seen == list(EXPECTED_GATES),
         "REAL_EVIDENCE:ORDER_DUPLICATE_OR_MISSING_GATE")
    complete_but_untrusted = (
        counts["INDEPENDENT_REVIEW_CLAIMED"] == len(EXPECTED_GATES)
        and expired_claims == 0)
    return {
        "schema": "PRM-G53-PHYSICAL-EVIDENCE-REVIEW-v1",
        "record_digest_matched_caller_supplied_pin": True,
        "external_pin_provenance": "NOT_AUTHENTICATED",
        "documentary_status": ("CLAIMED_COMPLETE_UNAUTHENTICATED" if
                               complete_but_untrusted else "INCOMPLETE"),
        "counts": {k: counts[k] for k in sorted(counts)},
        "expired_review_claims": expired_claims,
        "independent_custodian_identity": "NOT_VERIFIED_BY_THIS_TOOL",
        "independent_publication_retrieval": "NOT_PERFORMED",
        "physical_host_independence": "NOT_VERIFIED_BY_THIS_TOOL",
        "physical_G53": "HOLD_PENDING_INDEPENDENT_REVIEW",
        "partition_G54": "HOLD_NOT_AUTHORIZED",
        "real_data_persistence": "CLOSED",
        "automatic_go": False,
    }


def inspect_register_file(path, *, externally_supplied_digest, now_epoch=None):
    path = Path(path)
    strict_file(path, 1024 * 1024)
    obj = json_file(path)
    return inspect_register(obj,
                            externally_supplied_digest=externally_supplied_digest,
                            now_epoch=now_epoch)


def main(argv=None):
    parser = argparse.ArgumentParser(
        description="G5.3 offline evidence inventory (ALWAYS physical HOLD)")
    parser.add_argument("register")
    parser.add_argument("--expected-sha256", required=True,
                        help="Previously approved, externally supplied canonical JSON digest; "
                             "this program does not authenticate its origin")
    args = parser.parse_args(argv)
    try:
        report = inspect_register_file(
            args.register, externally_supplied_digest=args.expected_sha256)
    except (PreflightRejected, OSError, ValueError, TypeError) as exc:
        print("G53_REAL_EVIDENCE_INSPECTION=REJECTED:" + str(exc),
              file=sys.stderr)
        return 2
    print(json.dumps(report, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
