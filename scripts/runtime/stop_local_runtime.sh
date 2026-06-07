#!/usr/bin/env bash
set -euo pipefail

echo "=== PROMETHEUS STOP LOCAL RUNTIME ==="

echo
echo "=== STOP BRIDGE / HOLOCHAIN / LAIR / SANDBOX ==="
pkill -f "node bridge/server.mjs" || true
pkill -f "holochain" || true
pkill -f "lair-keystore" || true
pkill -f "hc sandbox" || true

sleep 2

echo
echo "=== REMAINING PORTS ==="
ss -ltnp | grep -E '14600|14602|8787' || echo "No Prometheus local runtime ports are listening."

echo
echo "STOP_LOCAL_RUNTIME: COMPLETE"
