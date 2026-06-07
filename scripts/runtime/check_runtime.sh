#!/usr/bin/env bash
set -euo pipefail

ADMIN_PORT="${HOLOCHAIN_ADMIN_PORT:-14600}"
APP_PORT="${HOLOCHAIN_APP_PORT:-14602}"
APP_ID="hearth_prometheus"

SANDBOX_DIR=".runtime/sandbox-main-runtime"

echo "=== PROMETHEUS RUNTIME CHECK ==="
echo "Admin port: $ADMIN_PORT"
echo "App websocket port: $APP_PORT"

if [ ! -d "$SANDBOX_DIR" ]; then
  echo "ERROR: $SANDBOX_DIR not found. Run create_fresh_sandbox.sh first."
  exit 1
fi

cd "$SANDBOX_DIR"

echo
echo "=== LIST APPS ==="
hc sandbox call --running "$ADMIN_PORT" list-apps

echo
echo "=== LIST CELLS ==="
hc sandbox call --running "$ADMIN_PORT" list-cells

echo
echo "=== LIST APP WEBSOCKETS ==="
APP_WS="$(hc sandbox call --running "$ADMIN_PORT" list-app-ws)"
echo "$APP_WS"

if ! echo "$APP_WS" | grep -q "\"port\":$APP_PORT"; then
  echo
  echo "=== ADD APP WEBSOCKET $APP_PORT ==="
  hc sandbox call --running "$ADMIN_PORT" add-app-ws "$APP_PORT"

  echo
  echo "=== LIST APP WEBSOCKETS AFTER ADD ==="
  hc sandbox call --running "$ADMIN_PORT" list-app-ws
fi

echo
echo "=== AUTHORIZE ZOME CALLS ==="
echo "You will be asked for the zome-call passphrase."
hc sandbox zome-call-auth --running "$ADMIN_PORT" "$APP_ID"

echo
echo "RUNTIME_CHECK: PASSED"
