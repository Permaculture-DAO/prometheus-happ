#!/usr/bin/env python3
"""G5.3 TEST-only custodian evidence packet preclearance (NEVER a real custody GO).

Checks file bytes, internal claims and prior witness signatures. Distinct labels
or uploaded files cannot establish independent custodians or external trust.
"""
from pathlib import Path

from g53_attestation import canonical, json_file, sha256, strict_file
from g53_external_witness import verify_witnessed_history, witness_head
from g53_preflight import keys, need, sha

SCHEMA = "PRM-G53-CUSTODY-PRECLEARANCE-v1"
ENROLLMENT_SCHEMA = "PRM-G53-CUSTODIAN-ENROLLMENT-EVIDENCE-v1"
PUBLICATION_SCHEMA = "PRM-G53-CUSTODIAN-PUBLICATION-EVIDENCE-v1"
MAX_EVIDENCE_BYTES = 32768


def _nonblank(value, marker):
    need(type(value) is str and 4 <= len(value) <= 256 and
         value == value.strip() and
         not any(x in value for x in "\r\n\x00"),
         marker + ":INVALID_REF")


def _parse_pinned_evidence(path, digest, tag):
    sha(digest, tag + ":PIN_FORMAT")
    raw = strict_file(Path(path), MAX_EVIDENCE_BYTES)
    need(sha256(raw) == digest, tag + ":BYTE_DIGEST_MISMATCH")
    # json_file enforces strict duplicate-key and non-finite JSON rejection.
    data = json_file(Path(path))
    need(type(data) is dict, tag + ":NOT_OBJECT")
    return data


def verify_custody_preclearance(
        db_path, epochs, transitions, receipt, dossier, *,
        custodian_evidence_files,
        externally_approved_dossier_sha256,
        root_public_pems, reviewer_public_pems, witness_public_pems,
        externally_pinned_genesis_root_fingerprints,
        externally_pinned_latest_root_fingerprints,
        externally_pinned_witness_fingerprints,
        externally_pinned_latest_witness_head_sha256,
        externally_pinned_previous_witness_head_sha256,
        externally_pinned_latest_witness_round,
        independently_approved_source_sha256,
        independently_approved_authorization_ref,
        now_epoch=None):
    """Offline mechanical preclearance; a passing dossier ALWAYS means HOLD."""
    sha(externally_approved_dossier_sha256, "CUSTODY:EXTERNAL_DOSSIER_DIGEST")
    need(type(dossier) is dict, "CUSTODY:DOSSIER_NOT_OBJECT")
    keys(dossier, [
        "schema", "scope", "source_sha256", "authorization_ref",
        "witness_receipt_head_sha256", "witness_round",
        "session_nonce_hex", "custodians"
    ], "CUSTODY:DOSSIER")
    need(dossier["schema"] == SCHEMA, "CUSTODY:SCHEMA")
    need(dossier["scope"] == "G5.3_TEST_ONLY", "CUSTODY:PRODUCTION_SCOPE_FORBIDDEN")
    need(sha256(canonical(dossier)) == externally_approved_dossier_sha256,
         "CUSTODY:EXTERNAL_DOSSIER_MISMATCH")
    need(dossier["source_sha256"] == independently_approved_source_sha256,
         "CUSTODY:SOURCE_MISMATCH")
    need(dossier["authorization_ref"] == independently_approved_authorization_ref,
         "CUSTODY:AUTHORIZATION_MISMATCH")
    actual_head = witness_head(receipt)
    need(dossier["witness_receipt_head_sha256"] == actual_head and
         actual_head == externally_pinned_latest_witness_head_sha256,
         "CUSTODY:WITNESS_HEAD_MISMATCH")
    statement = receipt["statement"]
    need(type(dossier["witness_round"]) is int and
         dossier["witness_round"] == statement["witness_round"] and
         dossier["witness_round"] == externally_pinned_latest_witness_round,
         "CUSTODY:WITNESS_ROUND_MISMATCH")
    need(dossier["session_nonce_hex"] == statement["session_nonce_hex"],
         "CUSTODY:SESSION_NONCE_MISMATCH")
    custodians = dossier["custodians"]
    need(type(custodians) is list and len(custodians) == 2,
         "CUSTODY:EXACTLY_TWO_CUSTODIANS_REQUIRED")
    fields = [
        "custodian_ref", "organization_ref", "administrator_domain_ref",
        "failure_domain_ref", "custody_domain_ref", "publication_channel_ref",
        "enrollment_authority_ref", "reviewer_approval_ref",
        "witness_pubkey_spki_sha256", "enrollment_evidence_sha256",
        "publication_evidence_sha256"
    ]
    for entry in custodians:
        keys(entry, fields, "CUSTODY:CUSTODIAN")
        for field in fields[:8]:
            _nonblank(entry[field], "CUSTODY:" + field)
        for field in fields[8:]:
            sha(entry[field], "CUSTODY:" + field)
    # Syntactically non-colliding references are a necessary, NOT sufficient
    # condition for actually independent organizations and failure domains.
    for field in fields[:8] + ["witness_pubkey_spki_sha256",
                               "enrollment_evidence_sha256",
                               "publication_evidence_sha256"]:
        need(custodians[0][field] != custodians[1][field],
             "CUSTODY:CLAIMED_DOMAIN_OR_KEY_COLLISION:" + field)
    custodian_ids = [x["custodian_ref"] for x in custodians]
    need(custodian_ids == sorted(custodian_ids),
         "CUSTODY:CUSTODIANS_NOT_SORTED")
    signed_witness_pins = sorted(externally_pinned_witness_fingerprints)
    need(sorted(x["witness_pubkey_spki_sha256"] for x in custodians)
         == signed_witness_pins, "CUSTODY:SIGNER_TO_CUSTODIAN_KEY_MISMATCH")
    need(type(custodian_evidence_files) is dict and
         set(custodian_evidence_files) == set(custodian_ids),
         "CUSTODY:MISSING_OR_EXTRA_FILES")

    # Cryptographic root-rotation, reviewer, receipt and final DB checks
    # predate and are mandatory for this custody evidence packet.
    checked = verify_witnessed_history(
        db_path, epochs, transitions, receipt,
        root_public_pems=root_public_pems,
        reviewer_public_pems=reviewer_public_pems,
        witness_public_pems=witness_public_pems,
        externally_pinned_genesis_root_fingerprints=
            externally_pinned_genesis_root_fingerprints,
        externally_pinned_latest_root_fingerprints=
            externally_pinned_latest_root_fingerprints,
        externally_pinned_witness_fingerprints=
            externally_pinned_witness_fingerprints,
        externally_pinned_latest_witness_head_sha256=
            externally_pinned_latest_witness_head_sha256,
        externally_pinned_previous_witness_head_sha256=
            externally_pinned_previous_witness_head_sha256,
        externally_pinned_latest_witness_round=
            externally_pinned_latest_witness_round,
        independently_approved_source_sha256=
            independently_approved_source_sha256,
        independently_approved_authorization_ref=
            independently_approved_authorization_ref,
        now_epoch=now_epoch,
    )
    # Relate each independently supplied pair of evidence files to the exact
    # cryptographically checked witness receipt. No file/record is trusted for
    # proving its own publisher's identity or administrative control.
    for entry in custodians:
        custodian_id = entry["custodian_ref"]
        evidence = custodian_evidence_files[custodian_id]
        need(type(evidence) is dict and set(evidence) ==
             {"enrollment", "publication"},
             "CUSTODY:FILE_PAIR_MISSING_OR_EXTRA")
        enrol = _parse_pinned_evidence(
            evidence["enrollment"], entry["enrollment_evidence_sha256"],
            "CUSTODY:ENROLLMENT")
        pub = _parse_pinned_evidence(
            evidence["publication"], entry["publication_evidence_sha256"],
            "CUSTODY:PUBLICATION")
        keys(enrol, [
            "schema", "custodian_ref", "witness_pubkey_spki_sha256",
            "organization_ref", "administrator_domain_ref",
            "failure_domain_ref", "custody_domain_ref",
            "enrollment_authority_ref", "reviewer_approval_ref",
            "session_nonce_hex", "source_sha256"
        ], "CUSTODY:ENROLLMENT_RECORD")
        need(enrol["schema"] == ENROLLMENT_SCHEMA,
             "CUSTODY:ENROLLMENT_SCHEMA")
        for name in ("custodian_ref", "witness_pubkey_spki_sha256",
                     "organization_ref", "administrator_domain_ref",
                     "failure_domain_ref", "custody_domain_ref",
                     "enrollment_authority_ref", "reviewer_approval_ref"):
            need(enrol[name] == entry[name],
                 "CUSTODY:ENROLLMENT_CLAIM_MISMATCH:" + name)
        need(enrol["session_nonce_hex"] == dossier["session_nonce_hex"] and
             enrol["source_sha256"] == dossier["source_sha256"],
             "CUSTODY:ENROLLMENT_SESSION_OR_SOURCE_MISMATCH")
        keys(pub, [
            "schema", "custodian_ref", "witness_pubkey_spki_sha256",
            "witness_receipt_head_sha256", "witness_round",
            "session_nonce_hex", "source_sha256",
            "publication_channel_ref"
        ], "CUSTODY:PUBLICATION_RECORD")
        need(pub["schema"] == PUBLICATION_SCHEMA, "CUSTODY:PUBLICATION_SCHEMA")
        for name in ("custodian_ref", "witness_pubkey_spki_sha256"):
            need(pub[name] == entry[name],
                 "CUSTODY:PUBLICATION_CLAIM_MISMATCH:" + name)
        need(pub["witness_receipt_head_sha256"] ==
             dossier["witness_receipt_head_sha256"] and
             pub["witness_round"] == dossier["witness_round"] and
             pub["session_nonce_hex"] == dossier["session_nonce_hex"] and
             pub["source_sha256"] == dossier["source_sha256"] and
             pub["publication_channel_ref"] == entry["publication_channel_ref"],
             "CUSTODY:PUBLICATION_COMMITMENT_MISMATCH")
    return "\n".join([
        checked,
        "G53_TEST_CUSTODY_PACKET_BYTE_BINDINGS=PASS",
        "G53_TEST_TWO_CUSTODIAN_REF_SEPARATION=PASS",
        "G53_REAL_CUSTODIAN_IDENTITY_AUTHENTICATION=NOT_PROVEN",
        "G53_INDEPENDENT_ADMIN_AND_FAILURE_DOMAINS=NOT_PROVEN",
        "G53_REAL_OUT_OF_BAND_PUBLICATION_PROVENANCE=NOT_PROVEN",
        "G53_EXTERNAL_DOSSIER_PIN_AUTHENTICITY=NOT_PROVEN",
        "G53_PHYSICAL_CROSS_HOST_REPLICATION=NOT_TESTED",
        "G53_GO_NO_GO=HOLD_PENDING_INDEPENDENT_REVIEW",
    ])
