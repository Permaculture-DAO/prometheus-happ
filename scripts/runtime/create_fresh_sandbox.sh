#!/usr/bin/env bash
set -euo pipefail

APP_ID="hearth_prometheus"
SANDBOX_DIR=".runtime/sandbox-main-runtime"

echo "=== PROMETHEUS CREATE FRESH SANDBOX ==="

if [ ! -f "hearth_prometheus.happ" ]; then
  echo "ERROR: hearth_prometheus.happ not found. Run scripts/runtime/build_pack_local.sh first."
  exit 1
fi

echo
echo "=== CLEAN OLD SANDBOX ==="
rm -rf "$SANDBOX_DIR"
mkdir -p "$SANDBOX_DIR"

echo
echo "=== COPY HAPP ==="
cp hearth_prometheus.happ "$SANDBOX_DIR/"

echo
echo "=== GENERATE SANDBOX ==="
cd "$SANDBOX_DIR"
hc sandbox generate --app-id "$APP_ID" hearth_prometheus.happ

echo
echo "CREATE_FRESH_SANDBOX: PASSED"
echo
echo "Next terminal command:"
echo "cd \"$(pwd)\""
echo "hc sandbox -f 14600 run"
