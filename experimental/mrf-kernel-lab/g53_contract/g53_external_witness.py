#!/usr/bin/env python3
"""G5.3 synthetic external-witness receipt verifier (NO real external custody).

The separate witnesses sign ONE shared bundle commitment. CI fixtures cannot
prove independent operators, independent trusted clocks or trustworthy pins.
"""
import re
import tempfile
import time
from pathlib import Path

from g53_attestation import canonical, openssl, public_der, sha256
from g53_epoch_history import verify_historical_epoch_chain
from g53_preflight import keys, need, sha
from g53_root_enrollment import enrollment_head

SCHEMA = "PRM-G53-EXTERNAL-WITNESS-RECEIPT-v1"
GENESIS = "0" * 64
MAX_WITNESSES = 4
MAX_AGE_SECONDS = 1800
SIGNATURE_HEX = re.compile(r"^[0-9a-f]{128}$")
NONCE_HEX = re.compile(r"^[0-9a-f]{64}$")


def history_hash(epochs, transitions):
    return sha256(b"PRM-G53-WITNESSED-HISTORY-v1\x00" +
                  canonical({"epochs": epochs, "transitions": transitions}))


def witness_message(statement):
    return b"PRM-G53-INDEPENDENT-WITNESS-CLAIM-v1\x00" + bytes.fromhex(
        sha256(canonical(statement)))


def witness_head(receipt):
    return sha256(b"PRM-G53-WITNESS-RECEIPT-HEAD-v1\x00" + canonical(receipt))


def verify_witnessed_history(
        db_path, epochs, transitions, receipt, *,
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
    """Offline witness proof-of-possession + delegated historical TEST audit."""
    keys(receipt, ["statement", "endorsements"], "G53_EXT_WITNESS:RECEIPT")
    statement = receipt["statement"]
    keys(statement, [
        "schema", "witness_round", "previous_witness_head_sha256",
        "latest_root_anchor_sha256", "latest_root_epoch",
        "genesis_root_fingerprints_sha256",
        "latest_root_fingerprints_sha256", "history_bundle_sha256",
        "source_sha256", "authorization_ref",
        "session_nonce_hex", "issued_at_epoch", "expires_at_epoch",
    ], "G53_EXT_WITNESS:STATEMENT")
    need(statement["schema"] == SCHEMA, "G53_EXT_WITNESS:SCHEMA")
    for name, value in (
        ("EXTERNAL_HEAD", externally_pinned_latest_witness_head_sha256),
        ("EXTERNAL_PREVIOUS", externally_pinned_previous_witness_head_sha256),
        ("EXTERNAL_SOURCE", independently_approved_source_sha256),
    ):
        sha(value, "G53_EXT_WITNESS:" + name)
    seq = statement["witness_round"]
    need(type(seq) is int and seq > 0 and
         type(externally_pinned_latest_witness_round) is int and
         seq == externally_pinned_latest_witness_round,
         "G53_EXT_WITNESS:STALE_OR_FUTURE_ROUND")
    sha(statement["previous_witness_head_sha256"],
       "G53_EXT_WITNESS:PREVIOUS_HEAD")
    need(statement["previous_witness_head_sha256"] ==
         externally_pinned_previous_witness_head_sha256,
         "G53_EXT_WITNESS:PREDECESSOR_NOT_PINNED")
    need((seq == 1 and statement["previous_witness_head_sha256"] == GENESIS)
         or (seq > 1 and statement["previous_witness_head_sha256"] != GENESIS),
         "G53_EXT_WITNESS:GENESIS_LINK")
    need(witness_head(receipt) == externally_pinned_latest_witness_head_sha256,
         "G53_EXT_WITNESS:EXTERNAL_HEAD_MISMATCH")
    need(statement["source_sha256"] == independently_approved_source_sha256,
         "G53_EXT_WITNESS:SOURCE_MISMATCH")
    need(statement["authorization_ref"] ==
         independently_approved_authorization_ref,
         "G53_EXT_WITNESS:AUTHORIZATION_MISMATCH")
    for name in ("latest_root_anchor_sha256",
                 "genesis_root_fingerprints_sha256",
                 "latest_root_fingerprints_sha256", "history_bundle_sha256"):
        sha(statement[name], "G53_EXT_WITNESS:" + name)
    need(type(epochs) is list and len(epochs) > 0,
         "G53_EXT_WITNESS:NO_EPOCHS")
    need(type(statement["latest_root_epoch"]) is int and
         statement["latest_root_epoch"] == len(epochs),
         "G53_EXT_WITNESS:EPOCH_COUNT_MISMATCH")
    need(statement["latest_root_anchor_sha256"] ==
         enrollment_head(epochs[-1]["anchor"]),
         "G53_EXT_WITNESS:ROOT_HEAD_MISMATCH")
    need(statement["genesis_root_fingerprints_sha256"] ==
         sha256(canonical(externally_pinned_genesis_root_fingerprints)),
         "G53_EXT_WITNESS:GENESIS_ROOT_SET_MISMATCH")
    need(statement["latest_root_fingerprints_sha256"] ==
         sha256(canonical(externally_pinned_latest_root_fingerprints)),
         "G53_EXT_WITNESS:LATEST_ROOT_SET_MISMATCH")
    need(statement["history_bundle_sha256"] == history_hash(epochs, transitions),
         "G53_EXT_WITNESS:HISTORY_BUNDLE_MISMATCH")
    nonce = statement["session_nonce_hex"]
    need(type(nonce) is str and NONCE_HEX.fullmatch(nonce) is not None
         and nonce != GENESIS, "G53_EXT_WITNESS:NONCE_INVALID")
    issued, expires = statement["issued_at_epoch"], statement["expires_at_epoch"]
    now = int(time.time()) if now_epoch is None else now_epoch
    need(type(issued) is int and type(expires) is int and
         type(now) is int and now > 0 and issued > 0 and
         0 < expires - issued <= MAX_AGE_SECONDS,
         "G53_EXT_WITNESS:TIME_WINDOW")
    need(issued <= now <= expires, "G53_EXT_WITNESS:EXPIRED_OR_FUTURE")

    witness_pins = externally_pinned_witness_fingerprints
    need(type(witness_pins) is list and
         2 <= len(witness_pins) <= MAX_WITNESSES,
         "G53_EXT_WITNESS:WITNESS_COUNT")
    for fingerprint in witness_pins:
        sha(fingerprint, "G53_EXT_WITNESS:KEY_FINGERPRINT")
    need(witness_pins == sorted(set(witness_pins)),
         "G53_EXT_WITNESS:WITNESS_KEYS_NOT_UNIQUE")
    need(type(witness_public_pems) is dict and
         set(witness_public_pems) == set(witness_pins),
         "G53_EXT_WITNESS:KEY_SET_MISMATCH")
    root_keys = set(root_public_pems)
    reviewer_keys = set(reviewer_public_pems)
    need(set(witness_pins).isdisjoint(root_keys | reviewer_keys),
         "G53_EXT_WITNESS:WITNESS_ROLE_COLLISION")
    for fingerprint, pub in witness_public_pems.items():
        need(sha256(public_der(Path(pub))) == fingerprint,
             "G53_EXT_WITNESS:KEY_SUBSTITUTION")

    endorsements = receipt["endorsements"]
    need(type(endorsements) is list and len(endorsements) == len(witness_pins),
         "G53_EXT_WITNESS:MISSING_WITNESS_ENDORSEMENT")
    signers = []
    for item in endorsements:
        keys(item, ["witness_pubkey_spki_sha256", "signature_hex"],
             "G53_EXT_WITNESS:ENDORSEMENT")
        signer = item["witness_pubkey_spki_sha256"]
        need(signer in witness_pins, "G53_EXT_WITNESS:UNAUTHORIZED_SIGNER")
        signers.append(signer)
        signature = item["signature_hex"]
        need(type(signature) is str and
             SIGNATURE_HEX.fullmatch(signature) is not None,
             "G53_EXT_WITNESS:INVALID_SIGNATURE_ENCODING")
    need(signers == witness_pins,
         "G53_EXT_WITNESS:MISSING_DUPLICATE_OR_UNSORTED_SIGNERS")
    with tempfile.TemporaryDirectory(prefix="g53-external-witness-test-") as temp:
        msgfile = Path(temp) / "message"
        sigfile = Path(temp) / "signature"
        msgfile.write_bytes(witness_message(statement))
        for item in endorsements:
            sigfile.write_bytes(bytes.fromhex(item["signature_hex"]))
            openssl(["pkeyutl", "-verify", "-pubin", "-inkey",
                     str(witness_public_pems[item["witness_pubkey_spki_sha256"]]),
                     "-sigfile", str(sigfile), "-rawin", "-in", str(msgfile)])
    outcome = verify_historical_epoch_chain(
        db_path, epochs, root_public_pems=root_public_pems,
        reviewer_public_pems=reviewer_public_pems,
        externally_pinned_root_key_sha256=externally_pinned_genesis_root_fingerprints,
        externally_pinned_latest_root_key_sha256=externally_pinned_latest_root_fingerprints,
        root_rotations=transitions,
        externally_pinned_latest_root_head_sha256=statement["latest_root_anchor_sha256"],
        externally_pinned_latest_epoch=statement["latest_root_epoch"],
        independently_approved_source_sha256=independently_approved_source_sha256,
        independently_approved_authorization_ref=independently_approved_authorization_ref,
        now_epoch=now,
    )
    return "\n".join([
        outcome,
        "G53_TEST_TWO_WITNESS_HISTORY_COMMITMENTS=PASS",
        "G53_TEST_WITNESS_FRESHNESS_AND_PIN_MATCH=PASS",
        "G53_EXTERNAL_WITNESS_KEY_ENROLLMENT=NOT_PROVEN",
        "G53_EXTERNAL_LATEST_WITNESS_HEAD_AUTHENTICITY=NOT_PROVEN",
        "G53_WITNESS_OPERATOR_INDEPENDENCE=NOT_PROVEN",
        "G53_GLOBAL_NONCE_REUSE_PREVENTION=NOT_PROVEN",
        "G53_PHYSICAL_INDEPENDENCE=NOT_PROVEN",
        "G53_GO_NO_GO=HOLD_PENDING_INDEPENDENT_REVIEW",
    ])
