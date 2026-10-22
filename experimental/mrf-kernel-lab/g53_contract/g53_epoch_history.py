#!/usr/bin/env python3
"""TEST-only multi-epoch enrollment and immutable historical signatures.

Offline audit of supplied claims, not proof of real reviewers/independent pins.
"""
import re
import tempfile
import time
from pathlib import Path

from g53_attestation import canonical, openssl, public_der, sha256
from g53_checkpoint_quorum import (REGISTRY_SCHEMA, SCHEMA as RECORD_SCHEMA,
                                   _check_checkpoint_shape, entry_head,
                                   signing_message)
from g53_ledger_checkpoint import verify_checkpoint
from g53_preflight import keys, need, sha
from g53_root_enrollment import (GENESIS, SCHEMA as ROOT_SCHEMA,
                                 enrollment_head, enrollment_message,
                                 MAX_ISSUE_WINDOW_SECONDS)

MAX_EPOCHS = 8
MAX_RECORDS = 32
SIGNATURE_HEX = re.compile(r"^[0-9a-f]{128}$")

def _key_signature(pem, signature_hex, message, directory):
    msg = Path(directory) / "message"
    sig = Path(directory) / "signature"
    msg.write_bytes(message)
    sig.write_bytes(bytes.fromhex(signature_hex))
    openssl(["pkeyutl", "-verify", "-pubin", "-inkey", str(pem),
             "-sigfile", str(sig), "-rawin", "-in", str(msg)])

def verify_historical_epoch_chain(
        db_path, epochs, *, root_public_pems, reviewer_public_pems,
        externally_pinned_root_key_sha256,
        externally_pinned_latest_root_head_sha256,
        externally_pinned_latest_epoch,
        independently_approved_source_sha256,
        independently_approved_authorization_ref,
        now_epoch=None):
    """Verify historical epoch policies and original signatures WITHOUT rewriting.

    Root public keys and terminal head/epoch are presumed independently
    authenticated. CI does not establish this critical premise.
    """
    for name, value in (
        ("ROOT_HEAD", externally_pinned_latest_root_head_sha256),
        ("SOURCE", independently_approved_source_sha256),
    ):
        sha(value, "G53_EPOCH_EXTERNAL:" + name)
    need(type(externally_pinned_latest_epoch) is int and
         1 <= externally_pinned_latest_epoch <= MAX_EPOCHS,
         "G53_EPOCH_EXTERNAL:INVALID_EPOCH")
    need(type(independently_approved_authorization_ref) is str and
         4 <= len(independently_approved_authorization_ref) <= 512 and
         not any(c in independently_approved_authorization_ref for c in "\x00\r\n"),
         "G53_EPOCH_EXTERNAL:INVALID_AUTHORIZATION")
    need(type(epochs) is list and
         len(epochs) == externally_pinned_latest_epoch,
         "G53_EPOCH:HISTORY_MISSING_OR_STALE")
    need(type(root_public_pems) is dict and
         type(reviewer_public_pems) is dict,
         "G53_EPOCH:KEY_MAP_INVALID")
    roots = externally_pinned_root_key_sha256
    need(type(roots) is list and 2 <= len(roots) <= 8,
         "G53_EPOCH:INVALID_ROOT_SET")
    for root in roots:
        sha(root, "G53_EPOCH:ROOT_FINGERPRINT")
    need(roots == sorted(set(roots)) and set(root_public_pems) == set(roots),
         "G53_EPOCH:ROOT_SET_MISMATCH")
    for root, pem in root_public_pems.items():
        need(sha256(public_der(Path(pem))) == root,
             "G53_EPOCH:ROOT_KEY_MISMATCH")

    now = int(time.time()) if now_epoch is None else now_epoch
    need(type(now) is int and now > 0, "G53_EPOCH:NOW_INVALID")
    prior_root = GENESIS
    prior_entry = GENESIS
    prior_revoked = set()
    last_record_time = 0
    last_root_time = 0
    last_row_count = -1
    total = 0
    terminal_checkpoint = None

    for number, epoch in enumerate(epochs, start=1):
        keys(epoch, ["anchor", "root_endorsements", "registry", "records"],
             "G53_EPOCH:ENVELOPE")
        anchor = epoch["anchor"]
        keys(anchor, [
            "schema", "epoch", "previous_anchor_sha256",
            "approved_source_sha256", "authorization_ref",
            "active_reviewer_registry_sha256",
            "revoked_reviewer_pubkey_spki_sha256",
            "latest_checkpoint_head_sha256", "latest_checkpoint_sequence",
            "issued_at_epoch", "expires_at_epoch"
        ], "G53_EPOCH:ROOT_ANCHOR")
        need(anchor["schema"] == ROOT_SCHEMA and
             type(anchor["epoch"]) is int and anchor["epoch"] == number,
             "G53_EPOCH:ROOT_EPOCH_GAP")
        sha(anchor["previous_anchor_sha256"], "G53_EPOCH:PREDECESSOR")
        need(anchor["previous_anchor_sha256"] == prior_root,
             "G53_EPOCH:ROOT_FORK_OR_MISSING_PREDECESSOR")
        need(anchor["approved_source_sha256"] ==
             independently_approved_source_sha256,
             "G53_EPOCH:ROOT_SOURCE_MISMATCH")
        need(anchor["authorization_ref"] ==
             independently_approved_authorization_ref,
             "G53_EPOCH:ROOT_AUTHORIZATION_MISMATCH")
        issued, expires = anchor["issued_at_epoch"], anchor["expires_at_epoch"]
        need(type(issued) is int and type(expires) is int and issued > 0 and
             0 < expires-issued <= MAX_ISSUE_WINDOW_SECONDS and
             issued >= last_root_time and issued >= last_record_time and
             issued <= now + 60, "G53_EPOCH:ROOT_TIME_INVALID")
        if number == len(epochs):
            need(issued <= now <= expires, "G53_EPOCH:LATEST_ROOT_EXPIRED")
        last_root_time = issued

        registry = epoch["registry"]
        keys(registry, ["schema", "test_source_sha256", "authorization_ref",
                        "reviewer_pubkey_spki_sha256", "required_quorum"],
             "G53_EPOCH:REVIEWER_REGISTRY")
        need(registry["schema"] == REGISTRY_SCHEMA and
             registry["test_source_sha256"] ==
             independently_approved_source_sha256 and
             registry["authorization_ref"] ==
             independently_approved_authorization_ref,
             "G53_EPOCH:REVIEWER_REGISTRY_SCOPE")
        need(sha256(canonical(registry)) ==
             anchor["active_reviewer_registry_sha256"],
             "G53_EPOCH:ROSTER_HASH_MISMATCH")
        roster = registry["reviewer_pubkey_spki_sha256"]
        need(type(roster) is list and 2 <= len(roster) <= 8,
             "G53_EPOCH:ROSTER_SIZE")
        for fingerprint in roster:
            sha(fingerprint, "G53_EPOCH:REVIEWER")
        need(roster == sorted(set(roster)) and
             set(roster).issubset(reviewer_public_pems),
             "G53_EPOCH:ROSTER_KEYS_MISSING_OR_DUPLICATED")
        quorum = registry["required_quorum"]
        need(type(quorum) is int and 2 <= quorum <= len(roster),
             "G53_EPOCH:INVALID_QUORUM")
        revoked = anchor["revoked_reviewer_pubkey_spki_sha256"]
        need(type(revoked) is list and len(revoked) <= 32,
             "G53_EPOCH:REVOCATION_LIMIT")
        for fingerprint in revoked:
            sha(fingerprint, "G53_EPOCH:REVOKED_FINGERPRINT")
        need(revoked == sorted(set(revoked)),
             "G53_EPOCH:REVOKED_DUPLICATE")
        current_revoked = set(revoked)
        need(prior_revoked.issubset(current_revoked),
             "G53_EPOCH:REVOCATION_ROLLBACK")
        need(set(roster).isdisjoint(current_revoked),
             "G53_EPOCH:REVOKED_REENROLLED")
        need(set(roster).isdisjoint(roots),
             "G53_EPOCH:ROOT_REVIEWER_ROLE_COLLISION")
        prior_revoked = current_revoked

        signatures = epoch["root_endorsements"]
        need(type(signatures) is list and 2 <= len(signatures) <= len(roots),
             "G53_EPOCH:ROOT_QUORUM_TOO_SMALL")
        signers = []
        for item in signatures:
            keys(item, ["root_pubkey_spki_sha256", "signature_hex"],
                 "G53_EPOCH:ROOT_ENDORSEMENT")
            signer = item["root_pubkey_spki_sha256"]
            need(signer in roots, "G53_EPOCH:ROOT_NOT_ENROLLED")
            signers.append(signer)
            need(type(item["signature_hex"]) is str and
                 SIGNATURE_HEX.fullmatch(item["signature_hex"]) is not None,
                 "G53_EPOCH:ROOT_SIGNATURE_FORMAT")
        need(signers == sorted(set(signers)),
             "G53_EPOCH:ROOT_DUPLICATE_SIGNER")
        with tempfile.TemporaryDirectory(prefix="g53-epoch-sigs-") as tmp:
            msg = enrollment_message(anchor)
            for item in signatures:
                _key_signature(root_public_pems[item["root_pubkey_spki_sha256"]],
                               item["signature_hex"], msg, tmp)
        prior_root = enrollment_head(anchor)
        records = epoch["records"]
        need(type(records) is list and 1 <= len(records) <= MAX_RECORDS and
             total + len(records) <= MAX_RECORDS,
             "G53_EPOCH:RECORD_LIMIT_OR_EMPTY")
        for record in records:
            total += 1
            keys(record, ["statement", "endorsements"], "G53_EPOCH:RECORD")
            statement = record["statement"]
            keys(statement, ["schema", "sequence", "previous_head_sha256",
                             "checkpoint", "approved_test_source_sha256",
                             "issued_at_epoch", "authorization_ref"],
                 "G53_EPOCH:STATEMENT")
            need(statement["schema"] == RECORD_SCHEMA and
                 type(statement["sequence"]) is int and
                 statement["sequence"] == total,
                 "G53_EPOCH:GLOBAL_SEQUENCE_GAP")
            sha(statement["previous_head_sha256"], "G53_EPOCH:RECORD_PREDECESSOR")
            need(statement["previous_head_sha256"] == prior_entry,
                 "G53_EPOCH:RECORD_FORK")
            need(statement["approved_test_source_sha256"] ==
                 independently_approved_source_sha256 and
                 statement["authorization_ref"] ==
                 independently_approved_authorization_ref,
                 "G53_EPOCH:RECORD_SCOPE")
            t = statement["issued_at_epoch"]
            need(type(t) is int and max(issued, last_record_time) <= t <= expires,
                 "G53_EPOCH:RECORD_TIME_OUTSIDE_AUTHORIZED_EPOCH")
            last_record_time = t
            cp = statement["checkpoint"]
            _check_checkpoint_shape(cp, independently_approved_source_sha256)
            need(cp["row_count"] >= last_row_count,
                 "G53_EPOCH:CHECKPOINT_ROW_REGRESSION")
            last_row_count = cp["row_count"]
            endorsements = record["endorsements"]
            need(type(endorsements) is list and
                 quorum <= len(endorsements) <= len(roster),
                 "G53_EPOCH:INSUFFICIENT_HISTORICAL_QUORUM")
            signed_by = []
            for endorsement in endorsements:
                keys(endorsement, ["reviewer_pubkey_spki_sha256", "signature_hex"],
                     "G53_EPOCH:ENDORSEMENT")
                signer = endorsement["reviewer_pubkey_spki_sha256"]
                need(signer in roster, "G53_EPOCH:REVIEWER_WRONG_EPOCH")
                signed_by.append(signer)
                sig = endorsement["signature_hex"]
                need(type(sig) is str and SIGNATURE_HEX.fullmatch(sig) is not None,
                     "G53_EPOCH:REVIEWER_SIGNATURE_FORMAT")
            need(signed_by == sorted(set(signed_by)),
                 "G53_EPOCH:REVIEWER_DUPLICATE_SIGNATURE")
            with tempfile.TemporaryDirectory(prefix="g53-epoch-review-") as tmp:
                msg = signing_message(statement)
                for item in endorsements:
                    signer = item["reviewer_pubkey_spki_sha256"]
                    _key_signature(reviewer_public_pems[signer],
                                   item["signature_hex"], msg, tmp)
            prior_entry = entry_head(record)
            terminal_checkpoint = cp
        need(anchor["latest_checkpoint_head_sha256"] == prior_entry and
             type(anchor["latest_checkpoint_sequence"]) is int and
             anchor["latest_checkpoint_sequence"] == total,
             "G53_EPOCH:ANCHOR_TERMINAL_HEAD_MISMATCH")
    need(prior_root == externally_pinned_latest_root_head_sha256,
         "G53_EPOCH:EXTERNAL_LATEST_ROOT_HEAD_MISMATCH")
    need(terminal_checkpoint is not None, "G53_EPOCH:EMPTY_HISTORY")
    # Earlier checkpoints remain authenticated claims, not replayed state.
    final = verify_checkpoint(
        db_path, terminal_checkpoint,
        externally_approved_digest=sha256(canonical(terminal_checkpoint)),
        reviewed_source_sha256=independently_approved_source_sha256)
    return "\n".join([
        final,
        "G53_TEST_MULTIEPOCH_HISTORICAL_SIGNATURES=PASS",
        "G53_TEST_TEMPORAL_REVOCATION_ENFORCEMENT=PASS",
        "G53_TEST_LINKED_ROOT_AND_CHECKPOINT_CHAINS=PASS",
        "G53_EXTERNAL_ROOT_PIN_AUTHENTICITY=NOT_PROVEN",
        "G53_ACTUAL_REVIEWER_INDEPENDENCE=NOT_PROVEN",
        "G53_DISTRIBUTED_CONSENSUS=NOT_PROVEN",
        "G53_PHYSICAL_INDEPENDENCE=NOT_PROVEN",
        "G53_GO_NO_GO=HOLD_PENDING_INDEPENDENT_REVIEW",
    ])
