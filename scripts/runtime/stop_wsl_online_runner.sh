#!/usr/bin/env bash
set -euo pipefail

HAPP_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
STATE_DIR="$HAPP_ROOT/.runtime/online"

stop_one(){
  local name="$1" f="$STATE_DIR/$1.pid"
  [ -f "$f" ] || { echo "$name: not tracked"; return 0; }
  local p; p="$(cat "$f" 2>/dev/null || true)"
  if [ -n "$p" ] && kill -0 "$p" 2>/dev/null; then
    kill "$p" 2>/dev/null || true
    for _ in {1..10}; do kill -0 "$p" 2>/dev/null || break; sleep .2; done
    kill -9 "$p" 2>/dev/null || true
    echo "$name: stopped"
  else
    echo "$name: already stopped"
  fi
  rm -f "$f"
}

stop_one ingress
stop_one public-bridge
stop_one runtime-adapter
stop_one conductor

echo "PROMETHEUS WSL RUNNER: STOPPED"
