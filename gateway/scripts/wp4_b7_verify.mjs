// WP4-B7: keep the gateway alive while the conductor is restarted, then prove
// reauthorization/reconnection by persisting and independently querying one event.

import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import { createWriteStream, existsSync, mkdirSync, writeFileSync } from "node:fs";
import { once } from "node:events";
import path from "node:path";
import mqtt from "mqtt";
import { RuntimeClient } from "../src/runtime_client.mjs";
import { signReading } from "../src/evidence.mjs";

const evidenceDir = process.env.PROMETHEUS_EVIDENCE_DIR;
const readyFile = process.env.PROMETHEUS_B7_READY_FILE;
const restartedFile = process.env.PROMETHEUS_B7_RESTARTED_FILE;
const runId = process.env.TEST_RUN_ID;
const hmacKey = process.env.PROMETHEUS_HMAC_KEY;
const mqttUrl = process.env.MQTT_URL || "mqtt://127.0.0.1:1883";
const adminPort = Number(process.env.HC_ADMIN_PORT_AGENT1);
const appPort = Number(process.env.HC_APP_PORT_AGENT1);
const appId = process.env.PROMETHEUS_APP_ID || "prometheus-wp4";
const role = process.env.PROMETHEUS_ROLE || "hearth";

for (const [name, value] of Object.entries({
  PROMETHEUS_EVIDENCE_DIR: evidenceDir,
  PROMETHEUS_B7_READY_FILE: readyFile,
  PROMETHEUS_B7_RESTARTED_FILE: restartedFile,
  TEST_RUN_ID: runId,
  PROMETHEUS_HMAC_KEY: hmacKey,
  HC_ADMIN_PORT_AGENT1: adminPort,
  HC_APP_PORT_AGENT1: appPort,
})) {
  if (!value) throw new Error(`${name} is required`);
}

mkdirSync(evidenceDir, { recursive: true });
const gatewayLog = createWriteStream(path.join(evidenceDir, "gateway-b7.log"), { flags: "a" });
const events = [];
const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

async function waitFor(predicate, label, timeout = 60000) {
  const deadline = Date.now() + timeout;
  while (Date.now() < deadline) {
    const value = await predicate();
    if (value) return value;
    await sleep(250);
  }
  throw new Error(`timeout waiting for ${label}`);
}

const gateway = spawn(process.execPath, ["src/ingest.mjs"], {
  cwd: path.resolve(path.dirname(new URL(import.meta.url).pathname), ".."),
  env: {
    ...process.env,
    MQTT_URL: mqttUrl,
    MQTT_TOPIC: "prometheus/+/+",
    PROMETHEUS_ALLOW_REAL: "0",
    PROMETHEUS_REQUIRE_SIGNATURE: "1",
    PROMETHEUS_ADMIN_PORT: String(adminPort),
    PROMETHEUS_APP_PORT: String(appPort),
    PROMETHEUS_APP_ID: appId,
    PROMETHEUS_ROLE: role,
    PROMETHEUS_MAX_RETRIES: "12",
    PROMETHEUS_RECONNECT_MS: "500",
  },
  stdio: ["ignore", "pipe", "pipe"],
});

const consume = (chunk) => {
  const text = chunk.toString();
  gatewayLog.write(text);
  for (const line of text.split(/\r?\n/)) {
    if (!line.startsWith("ingest:event ")) continue;
    try { events.push(JSON.parse(line.slice("ingest:event ".length))); } catch {}
  }
};
gateway.stdout.on("data", consume);
gateway.stderr.on("data", consume);

async function publish(subject, body) {
  const connection = mqtt.connect(mqttUrl, { reconnectPeriod: 0 });
  await once(connection, "connect");
  const payload = { ...body };
  payload.signature = signReading(payload, hmacKey);
  await new Promise((resolve, reject) =>
    connection.publish(
      `prometheus/${subject}/soil_moisture`,
      JSON.stringify(payload),
      { qos: 1 },
      (error) => error ? reject(error) : resolve(),
    ));
  await new Promise((resolve) => connection.end(false, resolve));
}

const result = { test: "B7", status: "RUNNING", run_id: runId };
let control;
try {
  await waitFor(() => events.find((event) => event.type === "broker_connected"), "gateway ready");
  writeFileSync(readyFile, `${new Date().toISOString()}\n`);
  await waitFor(() => existsSync(restartedFile), "conductor restart flag", 120000);

  control = new RuntimeClient({ adminPort, appPort, appId, role });
  const subject = `TEST-${runId}-conductor-restart`;
  await control.call("create_ohe", {
    id: subject,
    steward: "TEST-permaculture-dao",
    site: "TEST plot",
    status: "Proposed",
    baseline_recorded: false,
  });
  await publish(`${runId}-conductor-restart`, {
    sensor_id: "RESTART-TEST",
    value: 0.44,
    observed_at: 1900001012,
    calibration_hash: "calib-TEST",
    confidence: 0.9,
    reviewer: "reviewer-TEST",
    raw: { value: 0.44, run_id: runId },
    test: true,
  });
  await waitFor(
    () => events.find((event) => event.type === "stored" && event.id.includes("RESTART-TEST")),
    "post-restart stored event",
    90000,
  );
  const records = await waitFor(async () => {
    const found = await control.call("get_subject_evidence", subject);
    return found.length === 1 ? found : null;
  }, "post-restart independent query", 90000);
  assert.equal(records.length, 1);
  result.status = "PASS";
  result.persisted_records = 1;
  result.gateway_survived_restart = true;
  result.reauthorized = true;
} catch (error) {
  result.status = "FAIL";
  result.error = error?.stack || String(error);
  process.exitCode = 1;
} finally {
  if (gateway.exitCode === null) {
    gateway.kill("SIGTERM");
    await Promise.race([once(gateway, "exit"), sleep(5000)]);
  }
  if (control) await control.close();
  gatewayLog.end();
  writeFileSync(path.join(evidenceDir, "B7_RESULTS.json"), `${JSON.stringify(result, null, 2)}\n`);
  console.log(JSON.stringify(result, null, 2));
}
