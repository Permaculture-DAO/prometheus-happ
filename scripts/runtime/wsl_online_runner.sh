#!/usr/bin/env bash
set -euo pipefail

APP_ID="hearth_prometheus"
ADMIN_PORT="${HOLOCHAIN_ADMIN_PORT:-14600}"
APP_PORT="${HOLOCHAIN_APP_PORT:-14602}"
ADAPTER_PORT="${PROMETHEUS_RUNTIME_ADAPTER_PORT:-8788}"
PUBLIC_BRIDGE_PORT="${PROMETHEUS_PUBLIC_BRIDGE_PORT:-8787}"
INGRESS_PORT="${PROMETHEUS_INGRESS_PORT:-8080}"

HAPP_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
WORKSPACE_ROOT="${PROMETHEUS_WORKSPACE_ROOT:-$(cd "$HAPP_ROOT/.." && pwd)}"
BRIDGE_ROOT="${PROMETHEUS_BRIDGE_ROOT:-$WORKSPACE_ROOT/prometheus-bridge}"
CONSOLE_ROOT="${PROMETHEUS_CONSOLE_ROOT:-$WORKSPACE_ROOT/prometheus-console}"
SANDBOX_DIR="$HAPP_ROOT/.runtime/sandbox-main-runtime"
STATE_DIR="$HAPP_ROOT/.runtime/online"
LOG_DIR="$STATE_DIR/logs"

mkdir -p "$LOG_DIR"
fail(){ echo "RUNNER_ERROR: $*" >&2; exit 1; }
need(){ command -v "$1" >/dev/null 2>&1 || fail "missing command: $1"; }
wait_http(){
  local url="$1" name="$2" tries="${3:-40}"
  for ((i=1;i<=tries;i++)); do
    if curl -fsS --max-time 2 "$url" >/dev/null 2>&1; then echo "READY: $name"; return 0; fi
    sleep 1
  done
  fail "$name did not become ready: $url"
}
stop_pidfile(){
  local f="$1"
  [ -f "$f" ] || return 0
  local p; p="$(cat "$f" 2>/dev/null || true)"
  if [ -n "$p" ] && kill -0 "$p" 2>/dev/null; then kill "$p" 2>/dev/null || true; sleep 1; fi
  rm -f "$f"
}

printf '\n=== PROMETHEUS WSL ONLINE RUNNER ===\n'
if ! grep -qiE 'microsoft|wsl' /proc/version 2>/dev/null; then
  echo "WARN: WSL signature not detected; continuing on Linux-compatible host."
fi

for cmd in holochain hc lair-keystore node npm curl caddy; do need "$cmd"; done
if [ ! -f "$HAPP_ROOT/hearth_prometheus.happ" ] || [ "${PROMETHEUS_REBUILD:-0}" = "1" ]; then
  need cargo; need zip
fi
[ -d "$BRIDGE_ROOT" ] || fail "missing sibling repository: $BRIDGE_ROOT"
[ -d "$CONSOLE_ROOT" ] || fail "missing sibling repository: $CONSOLE_ROOT"
[ -f "$CONSOLE_ROOT/deploy/Caddyfile.wsl" ] || fail "missing console ingress config"

HC_VERSION="$(hc --version 2>/dev/null || true)"
HOLOCHAIN_VERSION="$(holochain --version 2>/dev/null || true)"
echo "hc: $HC_VERSION"
echo "holochain: $HOLOCHAIN_VERSION"
if [[ "$HC_VERSION" != *"0.6.1"* || "$HOLOCHAIN_VERSION" != *"0.6.1"* ]]; then
  fail "Genesis runner is pinned to Holochain/hc 0.6.1. Do not silently upgrade the signed runtime line."
fi

if [ -z "${PROMETHEUS_HC_PASSPHRASE:-}" ]; then
  read -s -p "Holochain sandbox passphrase: " PROMETHEUS_HC_PASSPHRASE
  echo
  export PROMETHEUS_HC_PASSPHRASE
fi
[ -n "$PROMETHEUS_HC_PASSPHRASE" ] || fail "empty Holochain passphrase"

cd "$HAPP_ROOT"
if [ ! -f hearth_prometheus.happ ] || [ "${PROMETHEUS_REBUILD:-0}" = "1" ]; then
  echo "Building and packing Genesis runtime…"
  ./scripts/runtime/build_pack_local.sh
fi

if [ ! -d "$SANDBOX_DIR" ] || [ ! -f "$SANDBOX_DIR/.hc" ]; then
  echo "Creating persistent sandbox…"
  ./scripts/runtime/create_fresh_sandbox.sh
else
  echo "Reusing persistent sandbox: $SANDBOX_DIR"
fi

stop_pidfile "$STATE_DIR/ingress.pid"
stop_pidfile "$STATE_DIR/public-bridge.pid"
stop_pidfile "$STATE_DIR/runtime-adapter.pid"
stop_pidfile "$STATE_DIR/conductor.pid"

printf '\n--- starting Holochain conductor ---\n'
(
  cd "$SANDBOX_DIR"
  export PROMETHEUS_HC_PASSPHRASE ADMIN_PORT
  nohup bash -c 'printf "%s\n" "$PROMETHEUS_HC_PASSPHRASE" | RUST_LOG="${RUST_LOG:-warn}" hc sandbox --piped -f "$ADMIN_PORT" run' \
    >"$LOG_DIR/conductor.log" 2>&1 & echo $! >"$STATE_DIR/conductor.pid"
)

for ((i=1;i<=45;i++)); do
  if (cd "$SANDBOX_DIR" && hc sandbox call --running "$ADMIN_PORT" list-apps >/dev/null 2>&1); then break; fi
  [ "$i" -eq 45 ] && fail "Holochain conductor did not become ready; inspect $LOG_DIR/conductor.log"
  sleep 1
done
echo "READY: Holochain admin interface on loopback:$ADMIN_PORT"

APP_WS_JSON="$(cd "$SANDBOX_DIR" && hc sandbox call --running "$ADMIN_PORT" list-app-ws 2>/dev/null || true)"
if ! grep -q "${APP_PORT}" <<<"$APP_WS_JSON"; then
  (cd "$SANDBOX_DIR" && hc sandbox call --running "$ADMIN_PORT" add-app-ws "$APP_PORT" >/dev/null)
fi
echo "READY: Holochain app interface on loopback:$APP_PORT"

DNA_JSON="$(cd "$SANDBOX_DIR" && hc sandbox call --running "$ADMIN_PORT" list-dnas)"
PROMETHEUS_EXPECTED_DNA_HASH="$(printf '%s' "$DNA_JSON" | node -e 'let s="";process.stdin.on("data",d=>s+=d).on("end",()=>{const a=JSON.parse(s);if(!Array.isArray(a)||a.length!==1)process.exit(2);process.stdout.write(a[0]);});')"
[ -n "$PROMETHEUS_EXPECTED_DNA_HASH" ] || fail "could not derive installed DNA hash"
export PROMETHEUS_EXPECTED_DNA_HASH

echo "READY: installed DNA identity derived from conductor"

printf '%s\n' "$PROMETHEUS_HC_PASSPHRASE" | \
  (cd "$SANDBOX_DIR" && hc sandbox zome-call-auth --piped --running "$ADMIN_PORT" "$APP_ID") \
  >"$LOG_DIR/zome-auth.log" 2>&1 || echo "WARN: zome-call auth was not refreshed; runtime health can still operate."

printf '\n--- starting loopback runtime adapter ---\n'
(
  cd "$HAPP_ROOT"
  export HOLOCHAIN_ADMIN_PORT="$ADMIN_PORT" HOLOCHAIN_APP_PORT="$APP_PORT"
  export PROMETHEUS_RUNTIME_ADAPTER_PORT="$ADAPTER_PORT" PROMETHEUS_HC_PASSPHRASE
  export PROMETHEUS_HC_SANDBOX_DIR="$SANDBOX_DIR" PROMETHEUS_EXPECTED_DNA_HASH
  nohup node bridge/server.mjs >"$LOG_DIR/runtime-adapter.log" 2>&1 & echo $! >"$STATE_DIR/runtime-adapter.pid"
)
wait_http "http://127.0.0.1:$ADAPTER_PORT/health" "Holochain runtime adapter"
wait_http "http://127.0.0.1:$ADAPTER_PORT/smoke-test" "Holochain zome smoke test"

printf '\n--- starting public read-only bridge ---\n'
(
  cd "$BRIDGE_ROOT"
  export PROMETHEUS_BRIDGE_HOST="127.0.0.1"
  export PROMETHEUS_BRIDGE_PORT="$PUBLIC_BRIDGE_PORT"
  export PROMETHEUS_RUNTIME_STATUS_URL="http://127.0.0.1:$ADAPTER_PORT/health"
  nohup node src/index.mjs >"$LOG_DIR/public-bridge.log" 2>&1 & echo $! >"$STATE_DIR/public-bridge.pid"
)
wait_http "http://127.0.0.1:$PUBLIC_BRIDGE_PORT/health" "public bridge"

printf '\n--- starting local web ingress ---\n'
(
  export PROMETHEUS_CONSOLE_ROOT="$CONSOLE_ROOT" PROMETHEUS_INGRESS_PORT="$INGRESS_PORT"
  nohup caddy run --config "$CONSOLE_ROOT/deploy/Caddyfile.wsl" --adapter caddyfile \
    >"$LOG_DIR/ingress.log" 2>&1 & echo $! >"$STATE_DIR/ingress.pid"
)
wait_http "http://127.0.0.1:$INGRESS_PORT/" "console ingress"
wait_http "http://127.0.0.1:$INGRESS_PORT/api/runtime" "end-to-end runtime route"

printf '\n=== RUNNER READY ===\n'
echo "Console ingress:       http://127.0.0.1:$INGRESS_PORT/"
echo "Public bridge:         http://127.0.0.1:$PUBLIC_BRIDGE_PORT/"
echo "Runtime adapter:       http://127.0.0.1:$ADAPTER_PORT/"
echo "Holochain admin:       loopback:$ADMIN_PORT (never expose publicly)"
echo "Holochain app ws:      loopback:$APP_PORT"
echo "Logs:                  $LOG_DIR"
echo
echo "External ingress/tunnel may now map app.heart-intelligence.earth to 127.0.0.1:$INGRESS_PORT."
echo "Do not publish ports $ADMIN_PORT, $APP_PORT or $ADAPTER_PORT."
