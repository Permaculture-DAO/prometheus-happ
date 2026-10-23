#!/usr/bin/env python3
"""G5.3 TEST-only custodian-signed publication receipt verification.

A valid signature proves key possession for supplied bytes, NOT that the
custodian exists independently or that any record was actually published.
"""
import re
import tempfile
from pathlib import Path

from g53_attestation import canonical, json_file, openssl, sha256, strict_file
from g53_custody_preclearance import verify_custody_preclearance
from g53_external_witness import witness_head
from g53_preflight import keys, need, sha

SCHEMA = "PRM-G53-CUSTODIAN-PUBLICATION-SIGNOFF-v1"
GENESIS = "0" * 64
MAX_FILE_BYTES = 32768
SIGNATURE_HEX = re.compile(r"^[0-9a-f]{128}$")


def publication_message(statement):
    return b"PRM-G53-CUSTODIAN-PUBLICATION-SIGNOFF-v1\x00" + bytes.fromhex(
        sha256(canonical(statement)))


def publication_head(signed_record):
    return sha256(b"PRM-G53-SIGNED-PUBLICATION-HEAD-v1\x00"
                  + canonical(signed_record))


def verify_publication_signature(signed_record, pub_pem):
    keys(signed_record, ["statement", "signature_hex"], "PUBLICATION:SIGNED_RECORD")
    statement = signed_record["statement"]
    fields = ["schema", "custodian_ref", "witness_pubkey_spki_sha256",
              "publication_channel_ref", "publication_evidence_sha256",
              "witness_receipt_head_sha256", "witness_round",
              "session_nonce_hex", "publication_sequence",
              "previous_publication_head_sha256", "issued_at_epoch"]
    keys(statement, fields, "PUBLICATION:STATEMENT")
    need(statement["schema"] == SCHEMA, "PUBLICATION:SCHEMA")
    sig = signed_record["signature_hex"]
    need(type(sig) is str and SIGNATURE_HEX.fullmatch(sig) is not None,
         "PUBLICATION:SIGNATURE_FORMAT")
    with tempfile.TemporaryDirectory(prefix="prm-g53-publication-") as tmp:
        msg = Path(tmp) / "message"
        signature_file = Path(tmp) / "signature"
        msg.write_bytes(publication_message(statement))
        signature_file.write_bytes(bytes.fromhex(sig))
        openssl(["pkeyutl", "-verify", "-pubin", "-inkey", str(pub_pem),
                 "-sigfile", str(signature_file), "-rawin", "-in", str(msg)])
    return statement


def verify_custodian_publications(
        db_path, epochs, transitions, receipt, dossier, *,
        custodian_evidence_files,
        signed_publication_files,
        externally_pinned_publication_heads,
        externally_pinned_publication_sequences,
        externally_pinned_previous_publication_heads,
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
    """Verify two publication signatures AFTER the complete custody TEST audit."""
    prior = verify_custody_preclearance(
        db_path, epochs, transitions, receipt, dossier,
        custodian_evidence_files=custodian_evidence_files,
        externally_approved_dossier_sha256=externally_approved_dossier_sha256,
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
        now_epoch=now_epoch)
    custodians = dossier["custodians"]
    ids = {x["custodian_ref"] for x in custodians}
    for name, obj in (
        ("SIGNED_FILES", signed_publication_files),
        ("EXTERNAL_HEADS", externally_pinned_publication_heads),
        ("EXTERNAL_SEQUENCES", externally_pinned_publication_sequences),
        ("EXTERNAL_PREDECESSORS", externally_pinned_previous_publication_heads),
    ):
        need(type(obj) is dict and set(obj) == ids,
             "PUBLICATION:" + name + "_INCOMPLETE")
    for signed_head in externally_pinned_publication_heads.values():
        sha(signed_head, "PUBLICATION:EXTERNAL_HEAD_FORMAT")
    need(len(set(externally_pinned_publication_heads.values())) == 2,
         "PUBLICATION:EXTERNAL_HEAD_COLLISION")
    expected_witness_head = witness_head(receipt)
    session = receipt["statement"]
    for item in custodians:
        ref = item["custodian_ref"]
        raw = strict_file(Path(signed_publication_files[ref]), MAX_FILE_BYTES)
        signed = json_file(Path(signed_publication_files[ref]))
        need(type(signed) is dict, "PUBLICATION:NOT_OBJECT")
        need(raw == canonical(signed), "PUBLICATION:NON_CANONICAL_BYTES")
        statement = signed.get("statement")
        need(type(statement) is dict, "PUBLICATION:STATEMENT_NOT_OBJECT")
        expected_head = externally_pinned_publication_heads[ref]
        sha(expected_head, "PUBLICATION:EXTERNAL_HEAD_FORMAT")
        need(publication_head(signed) == expected_head,
             "PUBLICATION:EXTERNAL_HEAD_MISMATCH")
        signer = item["witness_pubkey_spki_sha256"]
        need(signer in witness_public_pems,
             "PUBLICATION:WITNESS_SIGNER_NOT_ENROLLED")
        statement = verify_publication_signature(signed, witness_public_pems[signer])
        require = [
            ("custodian_ref", ref),
            ("witness_pubkey_spki_sha256", signer),
            ("publication_channel_ref", item["publication_channel_ref"]),
            ("publication_evidence_sha256", item["publication_evidence_sha256"]),
            ("witness_receipt_head_sha256", expected_witness_head),
            ("witness_round", session["witness_round"]),
            ("session_nonce_hex", session["session_nonce_hex"]),
        ]
        for name, value in require:
            need(statement[name] == value,
                 "PUBLICATION:SIGNED_CLAIM_MISMATCH:" + name)
        sequence = statement["publication_sequence"]
        pinned_seq = externally_pinned_publication_sequences[ref]
        need(type(sequence) is int and sequence > 0
             and type(pinned_seq) is int and sequence == pinned_seq,
             "PUBLICATION:SEQUENCE_NOT_PINNED")
        previous = statement["previous_publication_head_sha256"]
        sha(previous, "PUBLICATION:PREDECESSOR_FORMAT")
        sha(externally_pinned_previous_publication_heads[ref],
            "PUBLICATION:EXTERNAL_PREDECESSOR_FORMAT")
        need(previous == externally_pinned_previous_publication_heads[ref],
             "PUBLICATION:PREDECESSOR_NOT_PINNED")
        need((sequence == 1 and previous == GENESIS) or
             (sequence > 1 and previous != GENESIS),
             "PUBLICATION:INVALID_GENESIS_LINK")
        issued = statement["issued_at_epoch"]
        need(type(issued) is int and
             session["issued_at_epoch"] <= issued <= session["expires_at_epoch"],
             "PUBLICATION:TIMESTAMP_OUTSIDE_WITNESS_SESSION")
    return "\n".join([
        prior,
        "G53_TEST_TWO_SIGNED_PUBLICATIONS=PASS",
        "G53_TEST_PER_CUSTODIAN_PUBLICATION_PINS=PASS",
        "G53_EXTERNAL_PUBLICATION_RETRIEVAL=NOT_PERFORMED",
        "G53_PUBLICATION_ORIGIN_AUTHENTICITY=NOT_PROVEN",
        "G53_CUSTODIAN_REAL_INDEPENDENCE=NOT_PROVEN",
        "G53_GLOBAL_PUBLICATION_ANTI_EQUIVOCATION=NOT_PROVEN",
        "G53_GO_NO_GO=HOLD_PENDING_INDEPENDENT_REVIEW",
    ])
