#!/usr/bin/env bash
set -euo pipefail

ADMIN_PORT="${ADMIN_PORT:-14600}"
APP_ID="${PROMETHEUS_APP_ID:-hearth_prometheus}"
ROLE="${PROMETHEUS_ROLE:-hearth}"
RUN_ID="${PROMETHEUS_TEST_RUN_ID:-$(date -u +%Y%m%dT%H%M%SZ)}"
EVIDENCE_DIR="${P2_2B_EVIDENCE_DIR:-.runtime/p2-2b-evidence-${RUN_ID}}"

mkdir -p "$EVIDENCE_DIR"

echo "=== PROMETHEUS P2.2B-2 STANDALONE PROOF ===" | tee "$EVIDENCE_DIR/00_run.log"
echo "run_id=$RUN_ID" | tee -a "$EVIDENCE_DIR/00_run.log"
git rev-parse HEAD | tee "$EVIDENCE_DIR/git_head.txt"
holochain --version | tee "$EVIDENCE_DIR/holochain_version.txt"
hc --version | tee "$EVIDENCE_DIR/hc_version.txt"
node --version | tee "$EVIDENCE_DIR/node_version.txt"
rustc --version | tee "$EVIDENCE_DIR/rustc_version.txt"
cargo --version | tee "$EVIDENCE_DIR/cargo_version.txt"

scripts/runtime/build_pack_local.sh 2>&1 | tee "$EVIDENCE_DIR/10_build_pack.log"
cp P2_2B_ARTIFACT_SHA256SUMS.txt "$EVIDENCE_DIR/"

scripts/runtime/create_fresh_sandbox.sh 2>&1 | tee "$EVIDENCE_DIR/20_sandbox_generate.log"

pushd .runtime/sandbox-main-runtime >/dev/null
hc sandbox -f "$ADMIN_PORT" run >"../../$EVIDENCE_DIR/30_conductor.log" 2>&1 &
CONDUCTOR_PID=$!
popd >/dev/null

cleanup() {
  if kill -0 "$CONDUCTOR_PID" 2>/dev/null; then
    kill "$CONDUCTOR_PID" 2>/dev/null || true
    wait "$CONDUCTOR_PID" 2>/dev/null || true
  fi
}
trap cleanup EXIT

READY=0
for _ in $(seq 1 60); do
  if grep -Eq "Conductor ready|Conductor is ready|Admin interface|admin.*$ADMIN_PORT|listening.*$ADMIN_PORT" "$EVIDENCE_DIR/30_conductor.log" 2>/dev/null; then
    READY=1
    break
  fi
  if ! kill -0 "$CONDUCTOR_PID" 2>/dev/null; then
    echo "Conductor exited before readiness" | tee -a "$EVIDENCE_DIR/00_run.log"
    tail -n 120 "$EVIDENCE_DIR/30_conductor.log" || true
    exit 1
  fi
  sleep 2
done
if [ "$READY" -ne 1 ]; then
  echo "Conductor readiness not established within timeout" | tee -a "$EVIDENCE_DIR/00_run.log"
  tail -n 120 "$EVIDENCE_DIR/30_conductor.log" || true
  exit 1
fi

pushd gateway >/dev/null
npm ci 2>&1 | tee "../$EVIDENCE_DIR/40_gateway_npm_ci.log"
set +e
ADMIN_PORT="$ADMIN_PORT" PROMETHEUS_APP_ID="$APP_ID" PROMETHEUS_ROLE="$ROLE" PROMETHEUS_TEST_RUN_ID="$RUN_ID" npm run e2e:p2.2b 2>&1 | tee "../$EVIDENCE_DIR/50_appwebsocket_proof.log"
VERIFY_EXIT=${PIPESTATUS[0]}
set -e
popd >/dev/null

echo "$VERIFY_EXIT" | tee "$EVIDENCE_DIR/verifier_exit_code.txt"
if [ "$VERIFY_EXIT" -ne 0 ]; then
  echo "P2.2B-2: FAIL" | tee -a "$EVIDENCE_DIR/00_run.log"
  exit "$VERIFY_EXIT"
fi

grep -q '"overall_status": "PASS"' "$EVIDENCE_DIR/50_appwebsocket_proof.log"
grep -q '"subject_to_package": true' "$EVIDENCE_DIR/50_appwebsocket_proof.log"
grep -q '"package_to_review": true' "$EVIDENCE_DIR/50_appwebsocket_proof.log"
grep -q '"claim_to_decision": true' "$EVIDENCE_DIR/50_appwebsocket_proof.log"
grep -q 'Conflicted passing review rejected by integrity validation' "$EVIDENCE_DIR/50_appwebsocket_proof.log"
grep -q '"expected_decision": "blocked"' "$EVIDENCE_DIR/50_appwebsocket_proof.log"
grep -q '"authority_boundary": "admissibility_only_no_value"' "$EVIDENCE_DIR/50_appwebsocket_proof.log"

echo "P2.2B-2: PASS" | tee -a "$EVIDENCE_DIR/00_run.log"

(
  cd "$EVIDENCE_DIR"
  sha256sum ./* > SHA256SUMS.txt
)

echo "evidence_dir=$EVIDENCE_DIR"
