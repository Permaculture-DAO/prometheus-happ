#!/usr/bin/env bash
set -euo pipefail

EVIDENCE_DIR="${1:-}"
OUT_DIR="${2:-.runtime/p2-3-release-candidate}"

if [ -z "$EVIDENCE_DIR" ] || [ ! -d "$EVIDENCE_DIR" ]; then
  echo "usage: $0 <p2.2b-evidence-dir> [output-dir]" >&2
  exit 2
fi

mkdir -p "$OUT_DIR"
SOURCE_SHA="$(cat "$EVIDENCE_DIR/git_head.txt")"
VERIFY_EXIT="$(tr -d '[:space:]' < "$EVIDENCE_DIR/verifier_exit_code.txt")"
RECONCILED=false

if [ "$VERIFY_EXIT" != "0" ]; then
  echo "P2.3 refuses non-zero P2.2B verifier exit: $VERIFY_EXIT" >&2
  exit 1
fi

set +e
(
  cd "$EVIDENCE_DIR"
  sha256sum -c SHA256SUMS.txt
)
EVIDENCE_RC=$?
set -e

if [ "$EVIDENCE_RC" -ne 0 ]; then
  RECON="$EVIDENCE_DIR/POST_SEAL_RECONCILIATION.txt"
  RECON_HASH="$EVIDENCE_DIR/POST_SEAL_RECONCILIATION.sha256"
  [ -s "$RECON" ] && [ -s "$RECON_HASH" ] || {
    echo "P2.3 refuses evidence checksum mismatch without reconciliation addendum" >&2
    exit 1
  }
  (
    cd "$EVIDENCE_DIR"
    sha256sum -c POST_SEAL_RECONCILIATION.sha256
  )
  grep -q '^affected_file=00_run.log$' "$RECON"
  grep -q '^scope=00_run.log only; all other files in the original SHA256SUMS.txt verified OK\.$' "$RECON"
  EXPECTED_FINAL="$(sed -n 's/^final_post_append_sha256=//p' "$RECON")"
  ACTUAL_FINAL="$(sha256sum "$EVIDENCE_DIR/00_run.log" | awk '{print $1}')"
  [ -n "$EXPECTED_FINAL" ] && [ "$EXPECTED_FINAL" = "$ACTUAL_FINAL" ] || {
    echo "P2.3 reconciliation final hash does not match 00_run.log" >&2
    exit 1
  }
  grep -q '^P2.2B-2: PASS$' "$EVIDENCE_DIR/00_run.log"
  RECONCILED=true
fi

cp "$EVIDENCE_DIR/SHA256SUMS.txt" "$OUT_DIR/P2_2B_EVIDENCE_SHA256SUMS.txt"
cp "$EVIDENCE_DIR/P2_2B_ARTIFACT_SHA256SUMS.txt" "$OUT_DIR/P2_2B_ARTIFACT_SHA256SUMS.txt"
if [ "$RECONCILED" = true ]; then
  cp "$EVIDENCE_DIR/POST_SEAL_RECONCILIATION.txt" "$OUT_DIR/"
  cp "$EVIDENCE_DIR/POST_SEAL_RECONCILIATION.sha256" "$OUT_DIR/"
fi

cat > "$OUT_DIR/RELEASE_MANIFEST.json" <<EOF
{
  "schema_version": "prometheus.p2.3.release-manifest-candidate.v1",
  "release_status": "candidate_pending_remote_ci",
  "source_commit": "$SOURCE_SHA",
  "p2_2b_run_id": "$(basename "$EVIDENCE_DIR" | sed 's/^p2-2b-evidence-//')",
  "p2_2b_verifier_exit_code": 0,
  "evidence_integrity": {
    "original_manifest": "$([ "$RECONCILED" = true ] && echo "RECONCILED_POST_SEAL_APPEND" || echo "PASS")",
    "reconciliation_present": $RECONCILED
  },
  "data_class": "synthetic_TEST",
  "runtime_proof": {
    "standalone_appwebsocket": "PASS",
    "subject_to_package": true,
    "package_to_review": true,
    "claim_to_decision": true,
    "conflicted_reviewer_rejected": true,
    "expected_decision": "blocked"
  },
  "authority_boundary": "admissibility_only_no_value",
  "claim_boundary": {
    "ecological_truth": false,
    "scientific_validation": false,
    "certification": false,
    "legal_admission": false,
    "market_admission": false,
    "pru_value": false
  },
  "remote_ci": {
    "status": "PENDING_EXTERNAL_VERIFICATION",
    "required": ["secret-scan", "gateway-ci", "happ-ci"]
  },
  "promotion_rule": "Do not promote to integrated release until all required workflows for source_commit are completed successfully and the SweetConductor reconstruction step is confirmed executed and passed."
}
EOF

FILES=(
  "$OUT_DIR/RELEASE_MANIFEST.json"
  "$OUT_DIR/P2_2B_EVIDENCE_SHA256SUMS.txt"
  "$OUT_DIR/P2_2B_ARTIFACT_SHA256SUMS.txt"
)
if [ "$RECONCILED" = true ]; then
  FILES+=("$OUT_DIR/POST_SEAL_RECONCILIATION.txt" "$OUT_DIR/POST_SEAL_RECONCILIATION.sha256")
fi
sha256sum "${FILES[@]}" > "$OUT_DIR/P2_3_SHA256SUMS.txt"

echo "P2.3 RELEASE MANIFEST CANDIDATE: GENERATED"
echo "source_commit=$SOURCE_SHA"
echo "evidence_reconciled=$RECONCILED"
echo "status=candidate_pending_remote_ci"
echo "output_dir=$OUT_DIR"
