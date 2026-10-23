#!/usr/bin/env python3
"""Dual-quorum TEST-only root-key handover. No real custodian enrollment/GO."""
import re
import tempfile
from pathlib import Path

from g53_attestation import canonical, openssl, public_der, sha256
from g53_preflight import keys, need, sha
from g53_root_enrollment import enrollment_head

SCHEMA = "PRM-G53-ROOT-HANDOVER-v1"
SIGNATURE = re.compile(r"^[0-9a-f]{128}$")
MAX_ROOT_KEYS = 8


def handover_message(statement):
    return b"PRM-G53-DUAL-ROOT-HANDOVER-v1\x00" + bytes.fromhex(
        sha256(canonical(statement)))


def _roots(value, marker):
    need(type(value) is list and 2 <= len(value) <= MAX_ROOT_KEYS,
         marker + ":COUNT")
    for fingerprint in value:
        sha(fingerprint, marker + ":FINGERPRINT")
    need(value == sorted(set(value)), marker + ":NOT_SORTED_UNIQUE")


def _verify_quorum(records, roots, keys_by_digest, message, label):
    need(type(records) is list and 2 <= len(records) <= len(roots),
         label + ":INSUFFICIENT_QUORUM")
    need(type(keys_by_digest) is dict and
         set(roots).issubset(keys_by_digest),
         label + ":SIGNER_PUBLIC_KEY_MISSING")
    signer_ids = []
    for item in records:
        keys(item, ["root_pubkey_spki_sha256", "signature_hex"], label)
        signer = item["root_pubkey_spki_sha256"]
        sha(signer, label + ":SIGNER")
        need(signer in roots, label + ":UNAUTHORIZED_SIGNER")
        signer_ids.append(signer)
        sig = item["signature_hex"]
        need(type(sig) is str and SIGNATURE.fullmatch(sig) is not None,
             label + ":SIGNATURE_FORMAT")
    need(signer_ids == sorted(set(signer_ids)),
         label + ":DUPLICATE_OR_UNSORTED_SIGNER")
    with tempfile.TemporaryDirectory(prefix="prm-g53-root-handover-") as tmp:
        msgfile = Path(tmp) / "message"
        sigfile = Path(tmp) / "signature"
        msgfile.write_bytes(message)
        for item in records:
            pubkey = keys_by_digest[item["root_pubkey_spki_sha256"]]
            sigfile.write_bytes(bytes.fromhex(item["signature_hex"]))
            openssl(["pkeyutl", "-verify", "-pubin", "-inkey",
                     str(pubkey), "-sigfile", str(sigfile),
                     "-rawin", "-in", str(msgfile)])


def verify_root_handover_history(
        epochs, rotations, *,
        genesis_root_fingerprints,
        latest_root_fingerprints,
        root_public_pems,
        reviewed_source_sha256,
        authorization_ref):
    """Every transition needs an OLD and NEW quorum on the identical record.

    This is a bridge verification helper for g53_epoch_history, not a standalone
    authorization to run physical G5.3 or relax REAL/production boundaries.
    """
    _roots(genesis_root_fingerprints, "HANDOVER:GENESIS_ROOTS")
    _roots(latest_root_fingerprints, "HANDOVER:LATEST_ROOTS")
    need(type(root_public_pems) is dict,
         "HANDOVER:PUBLIC_KEY_MAP_INVALID")
    sha(reviewed_source_sha256, "HANDOVER:SOURCE")
    need(type(authorization_ref) is str and 4 <= len(authorization_ref) <= 512
         and not any(c in authorization_ref for c in "\r\n\x00"),
         "HANDOVER:AUTHORIZATION_REF")
    need(type(epochs) is list and 2 <= len(epochs) <= 8,
         "HANDOVER:INVALID_EPOCH_COUNT")
    need(type(rotations) is list and len(rotations) == len(epochs)-1,
         "HANDOVER:TRANSITION_GAP")
    current_roots = genesis_root_fingerprints
    root_sets = [current_roots]
    for epoch_no, transition in enumerate(rotations, start=2):
        keys(transition, ["statement", "old_endorsements", "new_endorsements"],
             "HANDOVER:TRANSITION")
        statement = transition["statement"]
        keys(statement, ["schema", "epoch", "previous_anchor_head_sha256",
                         "new_anchor_head_sha256", "old_root_fingerprints",
                         "new_root_fingerprints", "source_sha256",
                         "authorization_ref"], "HANDOVER:STATEMENT")
        need(statement["schema"] == SCHEMA and
             type(statement["epoch"]) is int and statement["epoch"] == epoch_no,
             "HANDOVER:INVALID_EPOCH")
        need(statement["source_sha256"] == reviewed_source_sha256 and
             statement["authorization_ref"] == authorization_ref,
             "HANDOVER:SOURCE_OR_AUTHORIZATION_MISMATCH")
        need(statement["old_root_fingerprints"] == current_roots,
             "HANDOVER:OLD_ROOTS_NOT_CURRENT")
        next_roots = statement["new_root_fingerprints"]
        _roots(next_roots, "HANDOVER:NEW_ROOTS")
        # TEST contract deliberately demands full root-set replacement:
        # a partial overlap cannot be misreported as independently rotated roots.
        need(set(current_roots).isdisjoint(next_roots),
             "HANDOVER:OLD_NEW_ROOT_SET_OVERLAP")
        # Reject substituted public PEMs before cryptographic verification,
        # so signer identity mismatches have a precise fail-closed diagnostic.
        for fingerprint in current_roots + next_roots:
            need(fingerprint in root_public_pems,
                 "HANDOVER:PUBLIC_KEY_MAP_MISMATCH")
            need(sha256(public_der(Path(root_public_pems[fingerprint]))) ==
                 fingerprint, "HANDOVER:PUBLIC_KEY_NOT_PINNED")
        for name in ("previous_anchor_head_sha256", "new_anchor_head_sha256"):
            sha(statement[name], "HANDOVER:" + name)
        need(statement["previous_anchor_head_sha256"] ==
             enrollment_head(epochs[epoch_no-2]["anchor"]),
             "HANDOVER:PREVIOUS_ANCHOR_MISMATCH")
        need(statement["new_anchor_head_sha256"] ==
             enrollment_head(epochs[epoch_no-1]["anchor"]),
             "HANDOVER:NEW_ANCHOR_MISMATCH")
        msg = handover_message(statement)
        # Two independently checked quorums sign the SAME transfer statement.
        _verify_quorum(transition["old_endorsements"], current_roots,
                       root_public_pems, msg, "HANDOVER:OUTGOING")
        _verify_quorum(transition["new_endorsements"], next_roots,
                       root_public_pems, msg, "HANDOVER:INCOMING")
        current_roots = next_roots
        root_sets.append(current_roots)
    need(current_roots == latest_root_fingerprints,
         "HANDOVER:EXTERNAL_LATEST_ROOT_SET_MISMATCH")
    all_roots = set().union(*(set(value) for value in root_sets))
    need(type(root_public_pems) is dict and
         set(root_public_pems) == all_roots,
         "HANDOVER:PUBLIC_KEY_MAP_MISMATCH")
    for fingerprint, pem in root_public_pems.items():
        need(sha256(public_der(Path(pem))) == fingerprint,
             "HANDOVER:PUBLIC_KEY_NOT_PINNED")
    return root_sets
