#!/usr/bin/env python3
"""TEST-only SQLite nonce consume gate: atomic on ONE local DB, not distributed trust."""
import sqlite3
import time
from pathlib import Path

from g53_attestation import canonical, sha256
from g53_preflight import PreflightRejected
from g53_witness_audit import validate_witness_session

class NonceClaimRejected(PreflightRejected):
    pass

DDL = """
CREATE TABLE IF NOT EXISTS test_nonce_claims (
    challenge_sha256 TEXT PRIMARY KEY NOT NULL,
    nonce_hex TEXT UNIQUE NOT NULL,
    approved_test_source_sha TEXT NOT NULL,
    reviewer_pubkey_spki_sha256 TEXT NOT NULL,
    expires_at_epoch INTEGER NOT NULL,
    claimed_at_epoch INTEGER NOT NULL
);
CREATE TRIGGER IF NOT EXISTS test_nonce_claims_no_update
BEFORE UPDATE ON test_nonce_claims
BEGIN SELECT RAISE(ABORT, 'TEST_APPEND_ONLY_UPDATE_REJECTED'); END;
CREATE TRIGGER IF NOT EXISTS test_nonce_claims_no_delete
BEFORE DELETE ON test_nonce_claims
BEGIN SELECT RAISE(ABORT, 'TEST_APPEND_ONLY_DELETE_REJECTED'); END;
"""

def _open(db_path):
    path = Path(db_path)
    if not path.is_file():
        raise NonceClaimRejected("TEST_NONCE_STORE_NOT_PROVISIONED")
    try:
        db = sqlite3.connect(str(path), timeout=8.0, isolation_level=None)
        db.execute("PRAGMA synchronous=FULL")
        db.execute("PRAGMA busy_timeout=8000")
        # Fail closed on an absent/changed local TEST schema, not just on INSERT.
        fields = db.execute("PRAGMA table_info(test_nonce_claims)").fetchall()
        if not any(z[1] == "challenge_sha256" and z[5] > 0 for z in fields):
            db.close()
            raise NonceClaimRejected("TEST_NONCE_STORE_SCHEMA_UNTRUSTED")
        unique_nonce = False
        for idx in db.execute("PRAGMA index_list(test_nonce_claims)").fetchall():
            if idx[2] != 1:
                continue
            # Only names sourced from local SQLite metadata are ever interpolated.
            columns = [col[2] for col in db.execute(
                'SELECT * FROM pragma_index_info(?)', (idx[1],)).fetchall()]
            if columns == ["nonce_hex"]:
                unique_nonce = True
        guards = db.execute(
            "SELECT name,sql FROM sqlite_master WHERE type='trigger' AND "
            "name IN ('test_nonce_claims_no_update','test_nonce_claims_no_delete')").fetchall()
        guard_names = {g[0] for g in guards if "RAISE(ABORT" in g[1].upper()}
        if not unique_nonce or guard_names != {"test_nonce_claims_no_update",
                                                "test_nonce_claims_no_delete"}:
            db.close()
            raise NonceClaimRejected("TEST_NONCE_STORE_SCHEMA_UNTRUSTED")
        return db
    except sqlite3.Error as exc:
        raise NonceClaimRejected("TEST_NONCE_STORE_UNAVAILABLE") from exc

def provision_test_store(db_path):
    """Explicit operator TEST setup. Never called by the claim path."""
    path = Path(db_path)
    if path.exists():
        raise NonceClaimRejected("TEST_NONCE_STORE_ALREADY_EXISTS")
    if not path.parent.is_dir():
        raise NonceClaimRejected("TEST_NONCE_STORE_PARENT_MISSING")
    try:
        with sqlite3.connect(str(path), timeout=8.0) as db:
            db.execute("PRAGMA synchronous=FULL")
            db.executescript(DDL)
    except sqlite3.Error as exc:
        raise NonceClaimRejected("TEST_NONCE_STORE_PROVISION_FAILED") from exc

def consume_verified_test_session(db_path, policy, a, b, challenge, *,
                                  pub_a, pub_b, sig_a, sig_b,
                                  approved_challenge_sha256, registry,
                                  reviewer_pub, reviewer_sig,
                                  reviewer_expected_pubkey_sha256, nonce_snapshot,
                                  now_epoch=None):
    """Always check signatures/witness BEFORE atomic claim; no caller-provided PASS flag."""
    evidence = validate_witness_session(
        policy, a, b, challenge, pub_a=pub_a, pub_b=pub_b,
        sig_a=sig_a, sig_b=sig_b,
        approved_challenge_sha256=approved_challenge_sha256,
        registry=registry, reviewer_pub=reviewer_pub, reviewer_sig=reviewer_sig,
        reviewer_expected_pubkey_sha256=reviewer_expected_pubkey_sha256,
        nonce_snapshot=nonce_snapshot, now_epoch=now_epoch)
    digest = sha256(canonical(challenge))
    # Time is rechecked AFTER locking to prevent expiry during concurrency waits.
    # now_epoch is permitted solely for deterministic fabricated CI fixtures.
    db = _open(db_path)
    try:
        db.execute("BEGIN IMMEDIATE")
        now = int(time.time()) if now_epoch is None else now_epoch
        if type(now) is not int or now < challenge["issued_at_epoch"] or now > challenge["expires_at_epoch"]:
            raise NonceClaimRejected("TEST_NONCE_CHALLENGE_EXPIRED_INSIDE_TRANSACTION")
        db.execute(
            "INSERT INTO test_nonce_claims "
            "(challenge_sha256,nonce_hex,approved_test_source_sha,"
            "reviewer_pubkey_spki_sha256,expires_at_epoch,claimed_at_epoch)"
            " VALUES (?,?,?,?,?,?)",
            (digest, challenge["nonce_hex"], policy["approved_test_source_sha"],
             registry["reviewer_pubkey_spki_sha256"], challenge["expires_at_epoch"], now))
        db.execute("COMMIT")
    except sqlite3.IntegrityError as exc:
        if db.in_transaction:
            db.execute("ROLLBACK")
        raise NonceClaimRejected("TEST_NONCE_OR_CHALLENGE_ALREADY_CONSUMED") from exc
    except (sqlite3.Error, NonceClaimRejected) as exc:
        if db.in_transaction:
            db.execute("ROLLBACK")
        if isinstance(exc, NonceClaimRejected):
            raise
        raise NonceClaimRejected("TEST_NONCE_STORE_TRANSACTION_FAILED") from exc
    finally:
        db.close()
    return "\n".join([
        evidence,
        "G53_TEST_LOCAL_ATOMIC_NONCE_CONSUMPTION=PASS",
        "G53_TEST_DUPLICATE_COMMIT_REJECTED_BY_UNIQUE_INDEX=INVARIANT",
        "G53_DISTRIBUTED_NONCE_CONSENSUS=NOT_PROVEN",
        "G53_LEDGER_OPERATOR_TRUST=NOT_PROVEN",
        "G53_PHYSICAL_INDEPENDENCE=NOT_PROVEN",
        "G53_GO_NO_GO=HOLD_PENDING_INDEPENDENT_REVIEW",
    ])

def test_store_claim_count(db_path):
    """Only a diagnostic for ephemeral CI; never interpret as global usage."""
    db = _open(db_path)
    try:
        return db.execute("SELECT COUNT(*) FROM test_nonce_claims").fetchone()[0]
    except sqlite3.Error as exc:
        raise NonceClaimRejected("TEST_NONCE_STORE_READ_FAILED") from exc
    finally:
        db.close()
