#!/usr/bin/env bash
set -euo pipefail
export PATH="$HOME/.cargo/bin:$PATH"

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ROOT="${PROMETHEUS_TEST_ROOT:-$REPO/.runtime-test}"
IMAGE="eclipse-mosquitto:2.1.2-alpine@sha256:a908c65cc8e67ec9d292ef27c2c0360dbaaee7eb1b935cdd194e67697f15dea1"
BROKER="prometheus-mqtt-test"
RUN_ID="WP4-$(date -u +%Y%m%dT%H%M%SZ)"
STATE="/tmp/pwp4-${RUN_ID#WP4-}"
EVIDENCE="$ROOT/evidence/$RUN_ID"
APP_ID="prometheus-wp4"
ROLE="hearth"
PASSPHRASE="wp4-$(python3 -c 'import secrets; print(secrets.token_hex(24))')"
HMAC_KEY="$(python3 -c 'import secrets; print(secrets.token_hex(32))')"

mkdir -p "$STATE/sandboxes" "$EVIDENCE/raw-logs"

read -r BOOTSTRAP_PORT ADMIN1 ADMIN2 APP1 APP2 < <(
  python3 - <<'PY'
import socket
sockets = []
ports = []
for _ in range(5):
    s = socket.socket()
    s.bind(("127.0.0.1", 0))
    sockets.append(s)
    ports.append(str(s.getsockname()[1]))
print(" ".join(ports))
for s in sockets:
    s.close()
PY
)

cleanup() {
  set +e
  if [[ -n "${CONDUCTOR_PID:-}" ]]; then
    kill -TERM -- "-$CONDUCTOR_PID" >/dev/null 2>&1 || true
  fi
  if [[ -n "${BOOTSTRAP_PID:-}" ]]; then
    kill -TERM "$BOOTSTRAP_PID" >/dev/null 2>&1 || true
  fi
  docker logs "$BROKER" > "$EVIDENCE/raw-logs/mosquitto.log" 2>&1 || true
  docker rm -f "$BROKER" >/dev/null 2>&1 || true
}
trap cleanup EXIT

redact_sensitive_logs() {
  local log
  for log in "$EVIDENCE"/raw-logs/*.log; do
    [[ -f "$log" ]] || continue
    sed -E -i \
      -e 's#(socket\?k=)[A-Za-z0-9_-]+#\1<REDACTED>#g' \
      -e 's#(connection_url[^#\r\n]*socket\?k=)[^ #\r\n]+#\1<REDACTED>#g' \
      "$log"
  done
}

wait_port() {
  local port="$1"
  local label="$2"
  for _ in $(seq 1 120); do
    if (echo > "/dev/tcp/127.0.0.1/$port") >/dev/null 2>&1; then
      return 0
    fi
    sleep 0.5
  done
  echo "timeout waiting for $label on port $port" >&2
  return 1
}

wait_port_closed() {
  local port="$1"
  local label="$2"
  for _ in $(seq 1 120); do
    if ! (echo > "/dev/tcp/127.0.0.1/$port") >/dev/null 2>&1; then
      return 0
    fi
    sleep 0.5
  done
  echo "timeout waiting for $label to close on port $port" >&2
  return 1
}

cat > "$EVIDENCE/runtime.env" <<EOF
TEST_RUN_ID=$RUN_ID
HC_ADMIN_URL_AGENT1=ws://127.0.0.1:$ADMIN1
HC_ADMIN_URL_AGENT2=ws://127.0.0.1:$ADMIN2
HC_APP_PORT_AGENT1=$APP1
HC_APP_PORT_AGENT2=$APP2
HC_APP_ID=$APP_ID
HC_ROLE_NAME=$ROLE
MQTT_URL=mqtt://127.0.0.1:1883
BOOTSTRAP_URL=http://127.0.0.1:$BOOTSTRAP_PORT/
RELAY_URL=ws://127.0.0.1:$BOOTSTRAP_PORT/
EOF

mkdir -p "$EVIDENCE/b0"

{
  echo "timestamp_utc=$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  echo "repo_sha=$(git -C "$REPO" rev-parse HEAD)"
  echo "branch=$(git -C "$REPO" branch --show-current)"
  echo "holochain=$(holochain --version)"
  echo "hc=$(hc --version)"
  echo "lair=$(lair-keystore --version)"
  echo "node=$(npx --yes node@20.19.4 --version)"
  echo "docker=$(docker --version)"
  echo "mosquitto_image=$IMAGE"
  sha256sum "$REPO/dnas/hearth/hearth.dna" "$REPO/hearth_prometheus.happ"
} > "$EVIDENCE/TOOLCHAIN.txt"

kitsune2-bootstrap-srv --listen "127.0.0.1:$BOOTSTRAP_PORT" --json \
  > "$EVIDENCE/raw-logs/bootstrap.log" 2>&1 &
BOOTSTRAP_PID=$!
wait_port "$BOOTSTRAP_PORT" "bootstrap/relay"

cd "$STATE"
printf '%s' "$PASSPHRASE" | hc sandbox --piped generate \
  --app-id "$APP_ID" \
  --num-sandboxes 2 \
  --root "$STATE/sandboxes" \
  --directories agent1,agent2 \
  --in-process-lair \
  --network-seed "$RUN_ID" \
  "$REPO/hearth_prometheus.happ" \
  network \
  --bootstrap "http://127.0.0.1:$BOOTSTRAP_PORT" \
  quic "ws://127.0.0.1:$BOOTSTRAP_PORT" \
  > "$EVIDENCE/raw-logs/sandbox-generate.log" 2>&1

# Keep every network dependency local for this synthetic proof.
for config in "$STATE"/sandboxes/agent*/conductor-config.yaml; do
  sed -i \
    "s#wss://dev-test-bootstrap2.holochain.org/#ws://127.0.0.1:$BOOTSTRAP_PORT/#g" \
    "$config"
  sed \
    -e "s#$STATE#<RUNTIME_STATE>#g" \
    -e 's#connection_url:.*#connection_url: <REDACTED>#g' \
    "$config" >> "$EVIDENCE/NETWORK_CONFIG_REDACTED.yaml"
  echo "---" >> "$EVIDENCE/NETWORK_CONFIG_REDACTED.yaml"
done

export WP4_PASS="$PASSPHRASE"
setsid bash -c \
  'printf "%s" "$WP4_PASS" | hc sandbox --piped -f="'"$ADMIN1,$ADMIN2"'" run -p="'"$APP1,$APP2"'" -a' \
  > "$EVIDENCE/raw-logs/conductors.log" 2>&1 &
CONDUCTOR_PID=$!

wait_port "$ADMIN1" "agent1 admin"
wait_port "$ADMIN2" "agent2 admin"
wait_port "$APP1" "agent1 app"
wait_port "$APP2" "agent2 app"

docker rm -f "$BROKER" >/dev/null 2>&1 || true
docker run -d \
  --name "$BROKER" \
  -p 127.0.0.1:1883:1883 \
  -v "$REPO/gateway/test/mosquitto:/mosquitto/config:ro" \
  "$IMAGE" > "$EVIDENCE/broker-container-id.txt"
wait_port 1883 "Mosquitto"

TOPIC="prometheus/TEST-site-b0/broker_health"
B0_PAYLOAD="{\"test\":true,\"run_id\":\"$RUN_ID\",\"message\":\"prometheus-broker-loopback\"}"
docker exec -d "$BROKER" sh -c \
  "mosquitto_sub -h 127.0.0.1 -p 1883 -t '$TOPIC' -C 1 -W 10 > /tmp/b0-received.txt"
sleep 0.5
docker exec "$BROKER" mosquitto_pub \
  -h 127.0.0.1 -p 1883 -t "$TOPIC" -m "$B0_PAYLOAD" -q 1
for _ in $(seq 1 40); do
  docker exec "$BROKER" test -s /tmp/b0-received.txt && break
  sleep 0.25
done
docker exec "$BROKER" cat /tmp/b0-received.txt > "$EVIDENCE/b0/received_payload.txt"
B0_RECEIVED="$(tr -d '\r\n' < "$EVIDENCE/b0/received_payload.txt")"
[[ "$B0_RECEIVED" == "$B0_PAYLOAD" ]]
cat > "$EVIDENCE/b0/results.json" <<EOF
{
  "test": "B0",
  "status": "PASS",
  "broker": "eclipse-mosquitto 2.1.2-alpine",
  "image_digest": "sha256:a908c65cc8e67ec9d292ef27c2c0360dbaaee7eb1b935cdd194e67697f15dea1",
  "payload_identical": true,
  "run_id": "$RUN_ID"
}
EOF

cd "$REPO/gateway"
set +e
MQTT_URL="mqtt://127.0.0.1:1883" \
PROMETHEUS_EVIDENCE_DIR="$EVIDENCE/raw-logs" \
PROMETHEUS_BROKER_CONTAINER="$BROKER" \
PROMETHEUS_APP_ID="$APP_ID" \
PROMETHEUS_ROLE="$ROLE" \
PROMETHEUS_HMAC_KEY="$HMAC_KEY" \
HC_ADMIN_PORT_AGENT1="$ADMIN1" \
HC_APP_PORT_AGENT1="$APP1" \
HC_ADMIN_PORT_AGENT2="$ADMIN2" \
HC_APP_PORT_AGENT2="$APP2" \
TEST_RUN_ID="$RUN_ID" \
npx --yes node@20.19.4 scripts/wp4_mqtt_verify.mjs
TEST_EXIT=$?
set -e
echo "$TEST_EXIT" > "$EVIDENCE/exit-code-b1-b6-b8.txt"

if [[ "$TEST_EXIT" -eq 0 ]]; then
  B7_READY="$STATE/b7-ready"
  B7_RESTARTED="$STATE/b7-restarted"
  rm -f "$B7_READY" "$B7_RESTARTED"

  set +e
  MQTT_URL="mqtt://127.0.0.1:1883" \
  PROMETHEUS_EVIDENCE_DIR="$EVIDENCE/raw-logs" \
  PROMETHEUS_APP_ID="$APP_ID" \
  PROMETHEUS_ROLE="$ROLE" \
  PROMETHEUS_HMAC_KEY="$HMAC_KEY" \
  PROMETHEUS_B7_READY_FILE="$B7_READY" \
  PROMETHEUS_B7_RESTARTED_FILE="$B7_RESTARTED" \
  HC_ADMIN_PORT_AGENT1="$ADMIN1" \
  HC_APP_PORT_AGENT1="$APP1" \
  TEST_RUN_ID="$RUN_ID" \
  npx --yes node@20.19.4 scripts/wp4_b7_verify.mjs \
    > "$EVIDENCE/raw-logs/b7-runner.log" 2>&1 &
  B7_PID=$!
  set -e

  for _ in $(seq 1 120); do
    [[ -f "$B7_READY" ]] && break
    sleep 0.5
  done
  [[ -f "$B7_READY" ]]

  kill -TERM -- "-$CONDUCTOR_PID"
  wait "$CONDUCTOR_PID" 2>/dev/null || true
  wait_port_closed "$ADMIN1" "agent1 admin"
  wait_port_closed "$ADMIN2" "agent2 admin"
  wait_port_closed "$APP1" "agent1 app"
  wait_port_closed "$APP2" "agent2 app"

  cd "$STATE"
  setsid bash -c \
    'printf "%s" "$WP4_PASS" | hc sandbox --piped -f="'"$ADMIN1,$ADMIN2"'" run -a' \
    >> "$EVIDENCE/raw-logs/conductors.log" 2>&1 &
  CONDUCTOR_PID=$!

  wait_port "$ADMIN1" "agent1 admin after restart"
  wait_port "$ADMIN2" "agent2 admin after restart"
  wait_port "$APP1" "agent1 app after restart"
  wait_port "$APP2" "agent2 app after restart"
  touch "$B7_RESTARTED"

  set +e
  wait "$B7_PID"
  B7_EXIT=$?
  set -e
else
  B7_EXIT=1
fi

echo "$B7_EXIT" > "$EVIDENCE/exit-code-b7.txt"

cp "$EVIDENCE/raw-logs/RESULTS.json" "$EVIDENCE/RESULTS.json"
if [[ -f "$EVIDENCE/raw-logs/B7_RESULTS.json" ]]; then
  cp "$EVIDENCE/raw-logs/B7_RESULTS.json" "$EVIDENCE/B7_RESULTS.json"
fi

# Freeze logs before hashing. Disable the EXIT trap after explicit cleanup.
cleanup
trap - EXIT
redact_sensitive_logs

cat > "$EVIDENCE/KNOWN_LIMITATIONS.md" <<'EOF'
# Known limitations

- Synthetic TEST data only.
- HMAC proves the bounded test/deployment path; production device identity, key
  provisioning and rotation remain separate security work.
- Durable duplicate suppression is proven for sequential delivery to one
  gateway/agent. Concurrent multi-agent duplicate races are not claimed solved.
- Local peer/session warnings may occur during reconnect. A matching action hash
  is required for B8; broader network-health admission is not inferred.
- Local two-agent bootstrap/relay does not prove WAN/NAT traversal, sustained load,
  long-duration stability or production readiness.
- No empirical, agronomic, legal, market, financial or value claim follows.
EOF

npx --yes node@20.19.4 "$REPO/gateway/scripts/finalize_results.mjs" "$EVIDENCE"

(
  cd "$EVIDENCE"
  find . -type f ! -name SHA256SUMS -print0 \
    | sort -z \
    | xargs -0 sha256sum > SHA256SUMS
)

cat "$EVIDENCE/RESULTS.json"
[[ -f "$EVIDENCE/B7_RESULTS.json" ]] && cat "$EVIDENCE/B7_RESULTS.json"
if [[ "$TEST_EXIT" -ne 0 || "$B7_EXIT" -ne 0 ]]; then
  exit 1
fi
