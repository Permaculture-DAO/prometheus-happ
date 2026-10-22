#!/usr/bin/env python3
"""G5.3 TEST-only read-only checkpoint comparison; cannot establish external trust."""
import hashlib
import sqlite3
from pathlib import Path

from g53_attestation import canonical, sha256
from g53_preflight import PreflightRejected, keys, need, sha

SCHEMA = "PRM-G53-LEDGER-CHECKPOINT-v1"
COLUMNS = ("challenge_sha256", "nonce_hex", "approved_test_source_sha",
           "reviewer_pubkey_spki_sha256", "expires_at_epoch", "claimed_at_epoch")
EXPECTED_TABLE = "test_nonce_claims"
TRIGGERS = {"test_nonce_claims_no_update", "test_nonce_claims_no_delete"}

class CheckpointRejected(PreflightRejected):
    pass

def _read_test_ledger(path):
    """Single SQLite read transaction; no database creation and no writes."""
    path = Path(path)
    need(path.is_file(), "TEST_CHECKPOINT_STORE_MISSING")
    try:
        connection = sqlite3.connect(path.resolve().as_uri() + "?mode=ro",
                                     uri=True, isolation_level=None, timeout=5)
    except sqlite3.Error as exc:
        raise CheckpointRejected("TEST_CHECKPOINT_STORE_UNAVAILABLE") from exc
    try:
        connection.execute("PRAGMA query_only=ON")
        connection.execute("BEGIN")
        ok = connection.execute("PRAGMA integrity_check").fetchall()
        need(ok == [("ok",)], "TEST_CHECKPOINT_INTEGRITY_FAILED")
        fields = connection.execute("PRAGMA table_info(test_nonce_claims)").fetchall()
        need(tuple(row[1] for row in fields) == COLUMNS,
             "TEST_CHECKPOINT_COLUMNS_CHANGED")
        need(fields[0][5] == 1, "TEST_CHECKPOINT_PRIMARY_KEY_MISSING")
        indexes = connection.execute("PRAGMA index_list(test_nonce_claims)").fetchall()
        unique_nonce = any(
            idx[2] == 1 and
            [col[2] for col in connection.execute(
                "SELECT * FROM pragma_index_info(?)", (idx[1],))] == ["nonce_hex"]
            for idx in indexes)
        need(unique_nonce, "TEST_CHECKPOINT_NONCE_UNIQUE_MISSING")
        schema_rows = connection.execute(
            "SELECT type,name,sql FROM sqlite_master WHERE "
            "(type='table' AND name=?) OR (type='trigger' AND name IN (?,?)) "
            "ORDER BY type,name",
            (EXPECTED_TABLE, *sorted(TRIGGERS))).fetchall()
        need(len(schema_rows) == 3 and
             {x[1] for x in schema_rows} == TRIGGERS | {EXPECTED_TABLE},
             "TEST_CHECKPOINT_SCHEMA_GUARDS_MISSING")
        need(all(row[2] is not None for row in schema_rows),
             "TEST_CHECKPOINT_SCHEMA_GUARDS_MISSING")
        rows = connection.execute(
            "SELECT " + ",".join(COLUMNS) + " FROM test_nonce_claims "
            "ORDER BY challenge_sha256").fetchall()
        for row in rows:
            for index in (0, 1, 2, 3):
                sha(row[index], "TEST_CHECKPOINT_ROW_SHA256_" + str(index))
            need(type(row[4]) is int and type(row[5]) is int and row[4] > 0
                 and row[5] > 0, "TEST_CHECKPOINT_ROW_TIME_INVALID")
        connection.execute("COMMIT")
        return schema_rows, rows
    except (sqlite3.Error, TypeError, ValueError) as exc:
        if connection.in_transaction:
            connection.execute("ROLLBACK")
        if isinstance(exc, PreflightRejected):
            raise
        raise CheckpointRejected("TEST_CHECKPOINT_READ_FAILED") from exc
    finally:
        connection.close()

def observe_checkpoint(db_path, reviewed_source_sha256):
    """Observation must be pinned outside this database to detect rollback."""
    sha(reviewed_source_sha256, "TEST_CHECKPOINT_SOURCE")
    schema_rows, rows = _read_test_ledger(db_path)
    return {
        "schema": SCHEMA,
        "test_source_sha256": reviewed_source_sha256,
        "row_count": len(rows),
        "rows_sha256": sha256(b"PRM-G53-LEDGER-ROWS-v1\x00" + canonical(rows)),
        "db_schema_sha256": sha256(b"PRM-G53-LEDGER-SCHEMA-v1\x00" +
                                  canonical(schema_rows)),
    }

def verify_checkpoint(db_path, checkpoint, *, externally_approved_digest,
                      reviewed_source_sha256):
    """Fail closed on changed/rolled-back TEST state relative to supplied pin."""
    keys(checkpoint, ["schema", "test_source_sha256", "row_count", "rows_sha256",
                      "db_schema_sha256"], "TEST_CHECKPOINT")
    need(checkpoint["schema"] == SCHEMA, "TEST_CHECKPOINT_WRONG_SCHEMA")
    sha(externally_approved_digest, "TEST_CHECKPOINT_EXTERNAL_PIN")
    sha(checkpoint["test_source_sha256"], "TEST_CHECKPOINT_SOURCE")
    sha(checkpoint["rows_sha256"], "TEST_CHECKPOINT_ROWS")
    sha(checkpoint["db_schema_sha256"], "TEST_CHECKPOINT_SCHEMA_DIGEST")
    need(type(checkpoint["row_count"]) is int and checkpoint["row_count"] >= 0,
         "TEST_CHECKPOINT_INVALID_COUNT")
    need(checkpoint["test_source_sha256"] == reviewed_source_sha256,
         "TEST_CHECKPOINT_SOURCE_MISMATCH")
    need(sha256(canonical(checkpoint)) == externally_approved_digest,
         "TEST_CHECKPOINT_EXTERNAL_DIGEST_MISMATCH")
    current = observe_checkpoint(db_path, reviewed_source_sha256)
    need(current["row_count"] == checkpoint["row_count"],
         "TEST_CHECKPOINT_ROLLBACK_OR_COUNT_CHANGE")
    need(current["db_schema_sha256"] == checkpoint["db_schema_sha256"],
         "TEST_CHECKPOINT_SCHEMA_DRIFT")
    need(current["rows_sha256"] == checkpoint["rows_sha256"],
         "TEST_CHECKPOINT_CONTENT_DRIFT")
    return "\n".join([
        "G53_TEST_LEDGER_CHECKPOINT_MATCH=PASS",
        "G53_LATEST_EXTERNAL_CHECKPOINT_PROVENANCE=NOT_PROVEN",
        "G53_INDEPENDENT_WITNESS_TRUST=NOT_PROVEN",
        "G53_DISTRIBUTED_ROLLBACK_RESISTANCE=NOT_PROVEN",
        "G53_PHYSICAL_INDEPENDENCE=NOT_PROVEN",
        "G53_GO_NO_GO=HOLD_PENDING_INDEPENDENT_REVIEW",
    ])
