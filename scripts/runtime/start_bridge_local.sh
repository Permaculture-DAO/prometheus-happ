#!/usr/bin/env bash
set -euo pipefail

export HOLOCHAIN_ADMIN_PORT="${HOLOCHAIN_ADMIN_PORT:-14600}"
export HOLOCHAIN_APP_PORT="${HOLOCHAIN_APP_PORT:-14602}"
export PROMETHEUS_BRIDGE_PORT="${PROMETHEUS_BRIDGE_PORT:-8787}"

if [ -z "${PROMETHEUS_HC_PASSPHRASE:-}" ]; then
  read -s -p "Enter Holochain zome-call passphrase: " PROMETHEUS_HC_PASSPHRASE
  echo
  export PROMETHEUS_HC_PASSPHRASE
fi

echo "=== PROMETHEUS START LOCAL BRIDGE ==="
echo "Admin port: $HOLOCHAIN_ADMIN_PORT"
echo "App websocket port: $HOLOCHAIN_APP_PORT"
echo "Bridge port: $PROMETHEUS_BRIDGE_PORT"

node bridge/server.mjs
