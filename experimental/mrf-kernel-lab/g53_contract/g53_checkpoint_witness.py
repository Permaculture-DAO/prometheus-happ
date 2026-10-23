#!/usr/bin/env python3
"""TEST-only signed checkpoint witness; requires an OUT-OF-BAND latest-head pin."""
import time
import tempfile
from pathlib import Path

from g53_attestation import canonical, openssl, public_der, sha256, strict_file
from g53_ledger_checkpoint import verify_checkpoint
from g53_preflight import PreflightRejected, keys, need, sha

SCHEMA = "PRM-G53-CHECKPOINT-WITNESS-v1"
GENESIS = "0" * 64
MAX_FUTURE_SKEW_SECONDS = 60

def witness_message(statement):
    """Explicit domain separation protects against host/reviewer signature reuse."""
    return b"PRM-G53-CHECKPOINT-WITNESS-v1\x00" + bytes.fromhex(
        sha256(canonical(statement)))

def witness_head_digest(statement):
    """Content-addressed witness head (not independently trusted by itself)."""
    return sha256(witness_message(statement))

def verify_signed_checkpoint(db_path, statement, *, witness_pub, witness_sig,
                             externally_pinned_witness_pubkey_sha256,
                             externally_pinned_latest_head_sha256,
                             externally_pinned_latest_sequence,
                             externally_pinned_previous_head_sha256,
                             approved_test_source_sha256, now_epoch=None):
    """No writes. Neither statement nor signature can supply its own trust anchor."""
    keys(statement, ["schema", "sequence", "previous_head_sha256", "checkpoint",
                     "approved_test_source_sha256", "issued_at_epoch",
                     "witness_pubkey_spki_sha256", "authorization_ref"],
         "TEST_WITNESS")
    need(statement["schema"] == SCHEMA, "TEST_WITNESS:SCHEMA_MISMATCH")
    for field in ("previous_head_sha256", "witness_pubkey_spki_sha256"):
        sha(statement[field], "TEST_WITNESS:" + field)
    for label, value in (
        ("EXTERNAL_WITNESS_KEY", externally_pinned_witness_pubkey_sha256),
        ("EXTERNAL_HEAD", externally_pinned_latest_head_sha256),
        ("EXTERNAL_PREDECESSOR", externally_pinned_previous_head_sha256),
        ("EXTERNAL_SOURCE", approved_test_source_sha256),
    ):
        sha(value, "TEST_WITNESS:" + label)
    n = statement["sequence"]
    need(type(n) is int and n > 0, "TEST_WITNESS:INVALID_SEQUENCE")
    need(type(externally_pinned_latest_sequence) is int and
         externally_pinned_latest_sequence > 0, "TEST_WITNESS:INVALID_EXTERNAL_SEQUENCE")
    need(n == externally_pinned_latest_sequence, "TEST_WITNESS:STALE_OR_FUTURE_SEQUENCE")
    previous = statement["previous_head_sha256"]
    need(previous == externally_pinned_previous_head_sha256,
         "TEST_WITNESS:PREDECESSOR_NOT_PINNED")
    need((n == 1 and previous == GENESIS) or (n > 1 and previous != GENESIS),
         "TEST_WITNESS:INVALID_GENESIS_LINK")
    need(statement["approved_test_source_sha256"] == approved_test_source_sha256,
         "TEST_WITNESS:SOURCE_MISMATCH")
    need(type(statement["authorization_ref"]) is str and
         4 <= len(statement["authorization_ref"]) <= 512,
         "TEST_WITNESS:AUTHORIZATION_REF")
    issued = statement["issued_at_epoch"]
    now = int(time.time()) if now_epoch is None else now_epoch
    need(type(issued) is int and issued > 0 and type(now) is int and
         issued <= now + MAX_FUTURE_SKEW_SECONDS,
         "TEST_WITNESS:INVALID_OR_FUTURE_ISSUED_AT")
    need(witness_head_digest(statement) == externally_pinned_latest_head_sha256,
         "TEST_WITNESS:LATEST_EXTERNAL_HEAD_MISMATCH")
    witness_der = public_der(witness_pub)
    digest = sha256(witness_der)
    need(digest == statement["witness_pubkey_spki_sha256"] and
         digest == externally_pinned_witness_pubkey_sha256,
         "TEST_WITNESS:WITNESS_KEY_NOT_EXTRINSICALLY_PINNED")
    signature = strict_file(witness_sig, 128)
    need(len(signature) == 64, "TEST_WITNESS:INVALID_SIGNATURE_LENGTH")
    with tempfile.TemporaryDirectory(prefix="g53-checkpoint-witness-") as tmp:
        message = Path(tmp) / "witness-message"
        message.write_bytes(witness_message(statement))
        openssl(["pkeyutl", "-verify", "-pubin", "-inkey", str(witness_pub),
                 "-sigfile", str(witness_sig), "-rawin", "-in", str(message)])
    # Check entire signed checkpoint AND current on-disk SQLite state.
    checkpoint = statement["checkpoint"]
    need(type(checkpoint) is dict, "TEST_WITNESS:CHECKPOINT_NOT_OBJECT")
    result = verify_checkpoint(
        db_path, checkpoint,
        externally_approved_digest=sha256(canonical(checkpoint)),
        reviewed_source_sha256=approved_test_source_sha256)
    return "\n".join([
        result,
        "G53_TEST_SIGNED_CHECKPOINT_WITNESS=PASS",
        "G53_TEST_PINNED_SEQUENCE_AND_PREDECESSOR=PASS",
        "G53_AUTHENTIC_OUT_OF_BAND_LATEST_HEAD=NOT_PROVEN",
        "G53_INDEPENDENT_WITNESS_KEY_ENROLLMENT=NOT_PROVEN",
        "G53_DISTRIBUTED_HEAD_CONSENSUS=NOT_PROVEN",
        "G53_PHYSICAL_INDEPENDENCE=NOT_PROVEN",
        "G53_GO_NO_GO=HOLD_PENDING_INDEPENDENT_REVIEW",
    ])
