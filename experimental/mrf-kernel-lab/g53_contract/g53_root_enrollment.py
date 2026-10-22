#!/usr/bin/env python3
"""G5.3 TEST-only root-signed reviewer enrollment and pinned latest-head policy.

This checks a *provided* out-of-band trust anchor; it cannot prove that the
root keys, latest head, or people/failure domains were authenticated externally.
"""
import re
import tempfile
import time
from pathlib import Path

from g53_attestation import canonical, openssl, public_der, sha256
from g53_checkpoint_quorum import verify_quorum_checkpoint_chain
from g53_preflight import PreflightRejected, keys, need, sha

SCHEMA = "PRM-G53-ENROLLMENT-ANCHOR-v1"
GENESIS = "0" * 64
MAX_ROOTS = 8
MAX_REVOKED = 32
MAX_ISSUE_WINDOW_SECONDS = 1800
SIGNATURE = re.compile(r"^[0-9a-f]{128}$")


def enrollment_message(anchor):
    return b"PRM-G53-ROOT-ENROLLMENT-v1\x00" + bytes.fromhex(sha256(canonical(anchor)))


def enrollment_head(anchor):
    return sha256(b"PRM-G53-ROOT-ENROLLMENT-HEAD-v1\x00" + canonical(anchor))


def verify_enrolled_checkpoint_chain(
        db_path, chain, registry, anchor, root_endorsements, *,
        reviewer_public_pems,
        root_public_pems,
        externally_pinned_root_key_sha256,
        externally_pinned_latest_anchor_sha256,
        externally_pinned_latest_anchor_epoch,
        externally_pinned_previous_anchor_sha256,
        externally_approved_source_sha256,
        externally_approved_authorization_ref,
        now_epoch=None):
    """Read-only; no trust-store reads/writes or key generation inside verifier."""
    for field, value in (
        ("LATEST_ANCHOR", externally_pinned_latest_anchor_sha256),
        ("PREVIOUS_ANCHOR", externally_pinned_previous_anchor_sha256),
        ("SOURCE", externally_approved_source_sha256),
    ):
        sha(value, "G53_ROOT_EXTERNAL:" + field)
    need(type(externally_approved_authorization_ref) is str and
         4 <= len(externally_approved_authorization_ref) <= 512 and
         not any(x in externally_approved_authorization_ref for x in "\r\n\x00"),
         "G53_ROOT_EXTERNAL:INVALID_AUTHORIZATION")
    keys(anchor, ["schema", "epoch", "previous_anchor_sha256",
                  "approved_source_sha256", "authorization_ref",
                  "active_reviewer_registry_sha256",
                  "revoked_reviewer_pubkey_spki_sha256",
                  "latest_checkpoint_head_sha256", "latest_checkpoint_sequence",
                  "issued_at_epoch", "expires_at_epoch"], "G53_ROOT_ANCHOR")
    need(anchor["schema"] == SCHEMA, "G53_ROOT_ANCHOR:SCHEMA")
    epoch = anchor["epoch"]
    need(type(epoch) is int and epoch > 0, "G53_ROOT_ANCHOR:EPOCH_TYPE")
    need(type(externally_pinned_latest_anchor_epoch) is int and
         externally_pinned_latest_anchor_epoch == epoch,
         "G53_ROOT_ANCHOR:STALE_OR_FUTURE_EPOCH")
    sha(anchor["previous_anchor_sha256"], "G53_ROOT_ANCHOR:PREDECESSOR")
    need(anchor["previous_anchor_sha256"] == externally_pinned_previous_anchor_sha256,
         "G53_ROOT_ANCHOR:PREDECESSOR_NOT_PINNED")
    need((epoch == 1 and anchor["previous_anchor_sha256"] == GENESIS) or
         (epoch > 1 and anchor["previous_anchor_sha256"] != GENESIS),
         "G53_ROOT_ANCHOR:INVALID_GENESIS_LINK")
    need(enrollment_head(anchor) == externally_pinned_latest_anchor_sha256,
         "G53_ROOT_ANCHOR:EXTERNAL_HEAD_MISMATCH")
    need(anchor["approved_source_sha256"] == externally_approved_source_sha256,
         "G53_ROOT_ANCHOR:SOURCE_MISMATCH")
    need(anchor["authorization_ref"] == externally_approved_authorization_ref,
         "G53_ROOT_ANCHOR:AUTHORIZATION_MISMATCH")
    sha(anchor["active_reviewer_registry_sha256"], "G53_ROOT_ANCHOR:REVIEWER_REGISTRY")
    sha(anchor["latest_checkpoint_head_sha256"], "G53_ROOT_ANCHOR:CHECKPOINT_HEAD")
    need(type(anchor["latest_checkpoint_sequence"]) is int and
         anchor["latest_checkpoint_sequence"] > 0,
         "G53_ROOT_ANCHOR:CHECKPOINT_SEQUENCE")
    issued, expires = anchor["issued_at_epoch"], anchor["expires_at_epoch"]
    need(type(issued) is int and type(expires) is int and issued > 0 and
         0 < expires - issued <= MAX_ISSUE_WINDOW_SECONDS,
         "G53_ROOT_ANCHOR:INVALID_TIME_WINDOW")
    now = int(time.time()) if now_epoch is None else now_epoch
    need(type(now) is int and issued <= now <= expires,
         "G53_ROOT_ANCHOR:STALE_OR_FUTURE_TIME")
    revoked = anchor["revoked_reviewer_pubkey_spki_sha256"]
    need(type(revoked) is list and len(revoked) <= MAX_REVOKED,
         "G53_ROOT_ANCHOR:REVOKED_TYPE_OR_LIMIT")
    for item in revoked:
        sha(item, "G53_ROOT_ANCHOR:REVOKED_FINGERPRINT")
    need(revoked == sorted(set(revoked)),
         "G53_ROOT_ANCHOR:DUPLICATE_OR_UNSORTED_REVOCATIONS")
    need(type(registry) is dict and
         type(registry.get("reviewer_pubkey_spki_sha256")) is list,
         "G53_ROOT_ANCHOR:INVALID_ACTIVE_REGISTRY")
    need(set(revoked).isdisjoint(registry["reviewer_pubkey_spki_sha256"]),
         "G53_ROOT_ANCHOR:ACTIVE_REVOKED_OVERLAP")
    need(sha256(canonical(registry)) == anchor["active_reviewer_registry_sha256"],
         "G53_ROOT_ANCHOR:UNAPPROVED_REVIEWER_ROSTER")

    # The root trust list is supplied by an independent authority in a real
    # deployment. The CI fixtures explicitly do not constitute such authority.
    roots = externally_pinned_root_key_sha256
    need(type(roots) is list and 2 <= len(roots) <= MAX_ROOTS,
         "G53_ROOT_EXTERNAL:INVALID_ROOT_SET")
    for root in roots:
        sha(root, "G53_ROOT_EXTERNAL:ROOT_FINGERPRINT")
    need(roots == sorted(set(roots)),
         "G53_ROOT_EXTERNAL:DUPLICATE_OR_UNSORTED_ROOTS")
    need(type(root_public_pems) is dict and set(root_public_pems) == set(roots),
         "G53_ROOT_EXTERNAL:MISSING_OR_EXTRA_ROOT_KEY")
    need(type(reviewer_public_pems) is dict,
         "G53_ROOT_EXTERNAL:REVIEWER_KEYS_INVALID")
    need(set(roots).isdisjoint(registry["reviewer_pubkey_spki_sha256"]),
         "G53_ROOT_EXTERNAL:ROOT_IS_ACTIVE_REVIEWER")
    for root, pem in root_public_pems.items():
        need(sha256(public_der(Path(pem))) == root,
             "G53_ROOT_EXTERNAL:ROOT_KEY_MISMATCH")
    need(type(root_endorsements) is list and
         2 <= len(root_endorsements) <= len(roots),
         "G53_ROOT:INSUFFICIENT_OR_EXCESS_ENDORSEMENTS")
    observed = []
    for endorsement in root_endorsements:
        keys(endorsement, ["root_pubkey_spki_sha256", "signature_hex"],
             "G53_ROOT_ENDORSEMENT")
        signer = endorsement["root_pubkey_spki_sha256"]
        sha(signer, "G53_ROOT_ENDORSEMENT:SIGNER")
        need(signer in roots, "G53_ROOT:UNRECOGNIZED_SIGNER")
        observed.append(signer)
        value = endorsement["signature_hex"]
        need(type(value) is str and SIGNATURE.fullmatch(value) is not None,
             "G53_ROOT:INVALID_SIGNATURE_ENCODING")
    need(observed == sorted(set(observed)),
         "G53_ROOT:DUPLICATE_OR_UNSORTED_SIGNERS")
    with tempfile.TemporaryDirectory(prefix="prm-g53-trust-") as directory:
        message = Path(directory) / "anchor-message"
        signature_file = Path(directory) / "signature"
        message.write_bytes(enrollment_message(anchor))
        for endorsement in root_endorsements:
            root = endorsement["root_pubkey_spki_sha256"]
            signature_file.write_bytes(bytes.fromhex(endorsement["signature_hex"]))
            openssl(["pkeyutl", "-verify", "-pubin", "-inkey",
                     str(root_public_pems[root]), "-sigfile",
                     str(signature_file), "-rawin", "-in", str(message)])
    # Only root-signed values are passed into the quorum verifier. This prevents
    # self-asserted registry/latest-head pins from substituting for this anchor.
    prior_result = verify_quorum_checkpoint_chain(
        db_path, chain, registry,
        reviewer_public_pems=reviewer_public_pems,
        externally_approved_registry_sha256=anchor["active_reviewer_registry_sha256"],
        externally_pinned_latest_head_sha256=anchor["latest_checkpoint_head_sha256"],
        externally_pinned_latest_sequence=anchor["latest_checkpoint_sequence"],
        independently_approved_test_source_sha256=anchor["approved_source_sha256"],
        externally_approved_authorization_ref=anchor["authorization_ref"],
        now_epoch=now)
    return "\n".join([
        prior_result,
        "G53_TEST_ROOT_QUORUM_ENROLLMENT=PASS",
        "G53_TEST_ROOT_SIGNED_LATEST_HEAD=PASS",
        "G53_TEST_REVOKED_SIGNER_EXCLUSION=PASS",
        "G53_ROOT_SIGNER_HUMAN_INDEPENDENCE=NOT_PROVEN",
        "G53_EXTERNAL_TRUST_ANCHOR_PROVENANCE=NOT_PROVEN",
        "G53_LATEST_ANCHOR_FRESHNESS_PROVENANCE=NOT_PROVEN",
        "G53_ROOT_KEY_REVOCATION_AND_ROTATION=NOT_TESTED",
        "G53_PHYSICAL_INDEPENDENCE=NOT_PROVEN",
        "G53_GO_NO_GO=HOLD_PENDING_INDEPENDENT_REVIEW",
    ])
