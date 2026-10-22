#!/usr/bin/env python3
"""G5.3 TEST-only multi-reviewer sequential checkpoint chain audit.

Never confuses a caller-supplied reviewer registry or latest-head pin with
independent trust, distributed consensus, REAL data or production authority.
"""
import re
import tempfile
import time
from pathlib import Path

from g53_attestation import canonical, openssl, public_der, sha256
from g53_ledger_checkpoint import SCHEMA as CHECKPOINT_SCHEMA, verify_checkpoint
from g53_preflight import PreflightRejected, keys, need, sha

SCHEMA = "PRM-G53-QUORUM-CHECKPOINT-v1"
REGISTRY_SCHEMA = "PRM-G53-QUORUM-REVIEWERS-v1"
GENESIS = "0" * 64
MAX_CHAIN = 32
MAX_REVIEWERS = 8
MAX_FUTURE_SKEW_SECONDS = 60
SIG_HEX = re.compile(r"^[0-9a-f]{128}$")


def signing_message(statement):
    return b"PRM-G53-QUORUM-CHECKPOINT-SIGN-v1\x00" + bytes.fromhex(
        sha256(canonical(statement)))


def entry_head(entry):
    """Signed record head, including the exact signer identities and signatures."""
    return sha256(b"PRM-G53-QUORUM-CHECKPOINT-ENTRY-v1\x00" + canonical(entry))


def _check_checkpoint_shape(data, source):
    keys(data, ["schema", "test_source_sha256", "row_count",
                "rows_sha256", "db_schema_sha256"], "QUORUM_CHECKPOINT")
    need(data["schema"] == CHECKPOINT_SCHEMA,
         "QUORUM_CHECKPOINT:WRONG_SCHEMA")
    need(data["test_source_sha256"] == source,
         "QUORUM_CHECKPOINT:SOURCE_MISMATCH")
    need(type(data["row_count"]) is int and data["row_count"] >= 0,
         "QUORUM_CHECKPOINT:BAD_ROW_COUNT")
    for field in ("rows_sha256", "db_schema_sha256"):
        sha(data[field], "QUORUM_CHECKPOINT:" + field)


def verify_quorum_checkpoint_chain(
        db_path, chain, registry, *,
        reviewer_public_pems,
        externally_approved_registry_sha256,
        externally_pinned_latest_head_sha256,
        externally_pinned_latest_sequence,
        independently_approved_test_source_sha256,
        externally_approved_authorization_ref,
        now_epoch=None):
    """Read-only chain verification. Pins/keys are NOT authenticated by this tool."""
    for name, val in (
        ("APPROVED_REGISTRY", externally_approved_registry_sha256),
        ("LATEST_HEAD", externally_pinned_latest_head_sha256),
        ("APPROVED_SOURCE", independently_approved_test_source_sha256),
    ):
        sha(val, "QUORUM_EXTERNAL:" + name)
    need(type(externally_approved_authorization_ref) is str and
         4 <= len(externally_approved_authorization_ref) <= 512 and
         not any(c in externally_approved_authorization_ref for c in "\r\n\x00"),
         "QUORUM_EXTERNAL:AUTHORIZATION_REF_INVALID")
    keys(registry, ["schema", "test_source_sha256", "authorization_ref",
                    "reviewer_pubkey_spki_sha256", "required_quorum"], "QUORUM_REGISTRY")
    need(registry["schema"] == REGISTRY_SCHEMA, "QUORUM_REGISTRY:SCHEMA")
    need(sha256(canonical(registry)) == externally_approved_registry_sha256,
         "QUORUM_REGISTRY:UNAPPROVED_REGISTRY_DIGEST")
    need(registry["test_source_sha256"] == independently_approved_test_source_sha256,
         "QUORUM_REGISTRY:SOURCE_MISMATCH")
    need(registry["authorization_ref"] == externally_approved_authorization_ref,
         "QUORUM_REGISTRY:AUTHORIZATION_MISMATCH")
    reviewers = registry["reviewer_pubkey_spki_sha256"]
    need(type(reviewers) is list and 2 <= len(reviewers) <= MAX_REVIEWERS,
         "QUORUM_REGISTRY:REVIEWER_COUNT")
    for reviewer in reviewers:
        sha(reviewer, "QUORUM_REGISTRY:REVIEWER_FINGERPRINT")
    need(reviewers == sorted(set(reviewers)),
         "QUORUM_REGISTRY:REVIEWERS_NOT_SORTED_UNIQUE")
    quorum = registry["required_quorum"]
    need(type(quorum) is int and 2 <= quorum <= len(reviewers),
         "QUORUM_REGISTRY:INVALID_QUORUM")
    need(type(reviewer_public_pems) is dict and
         set(reviewer_public_pems) == set(reviewers),
         "QUORUM_REGISTRY:MISSING_OR_EXTRA_REVIEWER_KEY")
    for digest, path in reviewer_public_pems.items():
        need(sha256(public_der(Path(path))) == digest,
             "QUORUM_REGISTRY:REVIEWER_KEY_NOT_PINNED")
    need(type(chain) is list and 1 <= len(chain) <= MAX_CHAIN,
         "QUORUM_CHAIN:EMPTY_OR_TOO_LONG")
    need(type(externally_pinned_latest_sequence) is int and
         externally_pinned_latest_sequence == len(chain),
         "QUORUM_CHAIN:EXTERNAL_SEQUENCE_MISMATCH")
    now = int(time.time()) if now_epoch is None else now_epoch
    need(type(now) is int and now > 0, "QUORUM_CHAIN:INVALID_NOW")
    previous = GENESIS
    last_rows = -1
    previous_issued = 0
    latest_checkpoint = None
    for seq, entry in enumerate(chain, start=1):
        keys(entry, ["statement", "endorsements"], "QUORUM_ENTRY")
        statement, endorsements = entry["statement"], entry["endorsements"]
        keys(statement, ["schema", "sequence", "previous_head_sha256",
                         "checkpoint", "approved_test_source_sha256",
                         "issued_at_epoch", "authorization_ref"], "QUORUM_STATEMENT")
        need(statement["schema"] == SCHEMA, "QUORUM_STATEMENT:SCHEMA")
        need(type(statement["sequence"]) is int and
             statement["sequence"] == seq, "QUORUM_CHAIN:SEQUENCE_GAP_OR_REORDER")
        sha(statement["previous_head_sha256"], "QUORUM_CHAIN:PREVIOUS_HEAD")
        need(statement["previous_head_sha256"] == previous,
             "QUORUM_CHAIN:BROKEN_PREDECESSOR_OR_FORK")
        need(statement["approved_test_source_sha256"] ==
             independently_approved_test_source_sha256,
             "QUORUM_CHAIN:SOURCE_MISMATCH")
        need(statement["authorization_ref"] == externally_approved_authorization_ref,
             "QUORUM_CHAIN:AUTHORIZATION_MISMATCH")
        issued = statement["issued_at_epoch"]
        need(type(issued) is int and previous_issued <= issued <=
             now + MAX_FUTURE_SKEW_SECONDS,
             "QUORUM_CHAIN:INVALID_OR_REVERSED_TIME")
        previous_issued = issued
        cp = statement["checkpoint"]
        _check_checkpoint_shape(cp, independently_approved_test_source_sha256)
        need(cp["row_count"] >= last_rows,
             "QUORUM_CHAIN:ROW_COUNT_REGRESSION")
        last_rows = cp["row_count"]
        need(type(endorsements) is list and quorum <= len(endorsements) <=
             len(reviewers), "QUORUM_CHAIN:INSUFFICIENT_OR_EXCESS_SIGNERS")
        signer_ids = []
        for endorsement in endorsements:
            keys(endorsement, ["reviewer_pubkey_spki_sha256",
                               "signature_hex"], "QUORUM_ENDORSEMENT")
            signer = endorsement["reviewer_pubkey_spki_sha256"]
            sha(signer, "QUORUM_ENDORSEMENT:REVIEWER")
            need(signer in reviewers, "QUORUM_ENDORSEMENT:UNAUTHORIZED_REVIEWER")
            signer_ids.append(signer)
            signature = endorsement["signature_hex"]
            need(type(signature) is str and SIG_HEX.fullmatch(signature) is not None,
                 "QUORUM_ENDORSEMENT:INVALID_SIGNATURE_ENCODING")
        need(signer_ids == sorted(set(signer_ids)),
             "QUORUM_ENDORSEMENT:DUPLICATE_OR_UNSORTED_SIGNERS")
        with tempfile.TemporaryDirectory(prefix="g53-quorum-check-") as directory:
            message = Path(directory) / "statement"
            message.write_bytes(signing_message(statement))
            signature_file = Path(directory) / "signature"
            for endorsement in endorsements:
                signer = endorsement["reviewer_pubkey_spki_sha256"]
                signature_file.write_bytes(bytes.fromhex(endorsement["signature_hex"]))
                openssl(["pkeyutl", "-verify", "-pubin", "-inkey",
                         str(reviewer_public_pems[signer]), "-sigfile",
                         str(signature_file), "-rawin", "-in", str(message)])
        previous = entry_head(entry)
        latest_checkpoint = cp
    need(previous == externally_pinned_latest_head_sha256,
         "QUORUM_CHAIN:EXTERNAL_LATEST_HEAD_MISMATCH")
    # Earlier checkpoints are signed historical claims. Only the TERMINAL one
    # can be compared to the currently supplied local TEST SQLite file.
    result = verify_checkpoint(
        db_path, latest_checkpoint,
        externally_approved_digest=sha256(canonical(latest_checkpoint)),
        reviewed_source_sha256=independently_approved_test_source_sha256)
    return "\n".join([
        result,
        "G53_TEST_FULL_QUORUM_CHECKPOINT_CHAIN=PASS",
        "G53_TEST_QUORUM_SIGNATURES_AND_LINKS=PASS",
        "G53_EXTERNAL_TRUST_REGISTRY_AUTHENTICITY=NOT_PROVEN",
        "G53_EXTERNAL_LATEST_HEAD_AUTHENTICITY=NOT_PROVEN",
        "G53_REVIEWER_INDEPENDENCE=NOT_PROVEN",
        "G53_HISTORICAL_CHECKPOINT_DATA_INDEPENDENTLY_REPLAYED=NOT_PROVEN",
        "G53_PHYSICAL_INDEPENDENCE=NOT_PROVEN",
        "G53_GO_NO_GO=HOLD_PENDING_INDEPENDENT_REVIEW",
    ])
