#!/usr/bin/env bash
set -euo pipefail

echo "=== PROMETHEUS LOCAL RUNTIME STATUS ==="

echo
echo "=== PORTS ==="
ss -ltnp | grep -E '14600|14602|8787' || echo "No Prometheus local runtime ports are listening."

echo
echo "=== PROCESSES ==="
ps aux | grep -E 'holochain|lair-keystore|hc sandbox|node bridge/server.mjs' | grep -v grep || echo "No Prometheus local runtime processes found."

echo
echo "STATUS_LOCAL_RUNTIME: COMPLETE"
