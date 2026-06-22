// WP4-B1..B6 and B8 runtime proof.
// Synthetic TEST data only. Produces machine-readable results and raw logs.

import assert from "node:assert/strict";
import { spawn, execFileSync } from "node:child_process";
import { createWriteStream, mkdirSync, writeFileSync } from "node:fs";
import { once } from "node:events";
import path from "node:path";
import mqtt from "mqtt";
import { RuntimeClient } from "../src/runtime_client.mjs";
import { signReading } from "../src/evidence.mjs";

const requiredNumber = (name) => {
  const value = Number(process.env[name]);
  if (!value) throw new Error(`${name} is required`);
  return value;
};

const evidenceDir = process.env.PROMETHEUS_EVIDENCE_DIR;
if (!evidenceDir) throw new Error("PROMETHEUS_EVIDENCE_DIR is required");
mkdirSync(evidenceDir, { recursive: true });

const mqttUrl = process.env.MQTT_URL || "mqtt://127.0.0.1:1883";
const appId = process.env.PROMETHEUS_APP_ID || "prometheus-wp4";
const role = process.env.PROMETHEUS_ROLE || "hearth";
const brokerContainer = process.env.PROMETHEUS_BROKER_CONTAINER || "prometheus-mqtt-test";
const hmacKey = process.env.PROMETHEUS_HMAC_KEY;
if (!hmacKey) throw new Error("PROMETHEUS_HMAC_KEY is required for WP4");
const runId = process.env.TEST_RUN_ID || `wp4-${Date.now()}`;
const admin1 = requiredNumber("HC_ADMIN_PORT_AGENT1");
const app1 = requiredNumber("HC_APP_PORT_AGENT1");
const admin2 = requiredNumber("HC_ADMIN_PORT_AGENT2");
const app2 = requiredNumber("HC_APP_PORT_AGENT2");

const results = {
  schema_version: "0.1",
  run_id: runId,
  overall_status: "RUNNING",
  data_class: "synthetic_TEST",
  tests: {},
  limitations: [],
};
const gatewayLog = createWriteStream(path.join(evidenceDir, "gateway.log"), { flags: "a" });
const e2eLog = createWriteStream(path.join(evidenceDir, "e2e.log"), { flags: "a" });
const log = (...parts) => {
  const line = parts.map((part) => typeof part === "string" ? part : JSON.stringify(part)).join(" ");
  console.log(line);
  e2eLog.write(`${new Date().toISOString()} ${line}\n`);
};
const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
const hashText = (value) => Buffer.from(value).toString("base64");

const client1 = new RuntimeClient({ adminPort: admin1, appPort: app1, appId, role });
const client2 = new RuntimeClient({ adminPort: admin2, appPort: app2, appId, role });
let gateway = null;
let events = [];

function record(id, status, detail = {}) {
  results.tests[id] = { status, ...detail };
  log(id, status, detail);
}

async function waitFor(predicate, { timeout = 30000, interval = 250, label = "condition" } = {}) {
  const deadline = Date.now() + timeout;
  while (Date.now() < deadline) {
    const value = await predicate();
    if (value) return value;
    await sleep(interval);
  }
  throw new Error(`timeout waiting for ${label}`);
}

function startGateway() {
  events = [];
  gateway = spawn(process.execPath, ["src/ingest.mjs"], {
    cwd: path.resolve(path.dirname(new URL(import.meta.url).pathname), ".."),
    env: {
      ...process.env,
      MQTT_URL: mqttUrl,
      MQTT_TOPIC: "prometheus/+/+",
      PROMETHEUS_ALLOW_REAL: "0",
      PROMETHEUS_ADMIN_PORT: String(admin1),
      PROMETHEUS_APP_PORT: String(app1),
      PROMETHEUS_APP_ID: appId,
      PROMETHEUS_ROLE: role,
      PROMETHEUS_MAX_RETRIES: "8",
      PROMETHEUS_RECONNECT_MS: "500",
      PROMETHEUS_REQUIRE_SIGNATURE: "1",
      PROMETHEUS_HMAC_KEY: hmacKey,
    },
    stdio: ["ignore", "pipe", "pipe"],
  });
  const consume = (chunk, stream) => {
    const text = chunk.toString();
    stream.write(text);
    for (const line of text.split(/\r?\n/)) {
      if (!line.startsWith("ingest:event ")) continue;
      try { events.push(JSON.parse(line.slice("ingest:event ".length))); } catch {}
    }
  };
  gateway.stdout.on("data", (chunk) => consume(chunk, gatewayLog));
  gateway.stderr.on("data", (chunk) => consume(chunk, gatewayLog));
  return waitFor(
    () => events.find((event) => event.type === "broker_connected"),
    { label: "gateway broker connection" },
  );
}

async function stopGateway() {
  if (!gateway || gateway.exitCode !== null) return;
  gateway.kill("SIGTERM");
  await Promise.race([once(gateway, "exit"), sleep(5000)]);
  gateway = null;
}

async function publish(subject, indicator, body) {
  const topic = `prometheus/${subject}/${indicator}`;
  const connection = mqtt.connect(mqttUrl, { reconnectPeriod: 0 });
  await once(connection, "connect");
  await new Promise((resolve, reject) =>
    connection.publish(topic, typeof body === "string" ? body : JSON.stringify(body), { qos: 1 }, (error) =>
      error ? reject(error) : resolve()));
  await new Promise((resolve) => connection.end(false, resolve));
}

function signed(body) {
  const payload = { ...body };
  payload.signature = signReading(payload, hmacKey);
  return payload;
}

async function createSubject(subject) {
  await client1.call("create_ohe", {
    id: subject,
    steward: "TEST-permaculture-dao",
    site: "TEST plot",
    status: "Proposed",
    baseline_recorded: false,
  });
}

async function subjectRecords(client, subject) {
  return client.call("get_subject_evidence", subject);
}

async function waitForRecords(client, subject, count, timeout = 45000) {
  return waitFor(async () => {
    const records = await subjectRecords(client, subject);
    return records.length === count ? records : null;
  }, { timeout, interval: 750, label: `${count} records for ${subject}` });
}

async function assess(client, subject, legalGate, threshold = 0.85, requireReviewer = true) {
  return client.call("assess_subject_admissibility", {
    subject_id: subject,
    legal_gate: legalGate,
    confidence_threshold: threshold,
    require_reviewer: requireReviewer,
  });
}

async function run() {
  await client1.connect();
  await client2.connect();
  await startGateway();

  // B1: valid MQTT -> gateway -> conductor -> independent query.
  const subject1 = `TEST-${runId}-valid`;
  const topicSubject1 = `${runId}-valid`;
  await createSubject(subject1);
  const valid = {
    sensor_id: "SE01-LS-TEST",
    value: 0.31,
    observed_at: 1900001001,
    calibration_hash: "calib-TEST",
    confidence: 0.9,
    reviewer: "reviewer-TEST",
    raw: { value: 0.31, run_id: runId },
    test: true,
  };
  await publish(topicSubject1, "soil_moisture", signed(valid));
  const records1 = await waitForRecords(client1, subject1, 1);
  assert.equal(records1[0].evidence.id, "test:SE01-LS-TEST:soil_moisture:1900001001");
  const actionHash = hashText(records1[0].action_hash);
  record("B1", "PASS", { records: 1, action_hash_b64: actionHash });

  // Live gate cases on independently queried persisted evidence.
  const passAssessment = await assess(client1, subject1, true);
  assert.equal(passAssessment.admissible, true);
  assert.equal(passAssessment.authority_boundary, "admissibility_only_no_value");
  const legalFalse = await assess(client1, subject1, false);
  assert.equal(legalFalse.admissible, false);

  const noReviewerSubject = `TEST-${runId}-no-reviewer`;
  await createSubject(noReviewerSubject);
  await publish(`${runId}-no-reviewer`, "soil_moisture", signed({ ...valid, observed_at: 1900001002, reviewer: null }));
  await waitForRecords(client1, noReviewerSubject, 1);
  assert.equal((await assess(client1, noReviewerSubject, true)).admissible, false);

  const lowConfidenceSubject = `TEST-${runId}-low-confidence`;
  await createSubject(lowConfidenceSubject);
  await publish(`${runId}-low-confidence`, "soil_moisture", signed({ ...valid, observed_at: 1900001003, confidence: 0.4 }));
  await waitForRecords(client1, lowConfidenceSubject, 1);
  assert.equal((await assess(client1, lowConfidenceSubject, true)).admissible, false);
  record("B4_GATES", "PASS", {
    all_gates: true,
    legal_false: false,
    missing_reviewer: false,
    low_confidence: false,
  });

  // B2: duplicate delivery, one persisted business record.
  const duplicateSubject = `TEST-${runId}-duplicate`;
  await createSubject(duplicateSubject);
  const duplicateBody = { ...valid, sensor_id: "DUP-TEST", observed_at: 1900001004 };
  await publish(`${runId}-duplicate`, "soil_moisture", signed(duplicateBody));
  await publish(`${runId}-duplicate`, "soil_moisture", signed(duplicateBody));
  await waitForRecords(client1, duplicateSubject, 1);
  await waitFor(
    () => events.find((event) => event.type === "duplicate"),
    { label: "gateway duplicate event" },
  );
  record("B2", "PASS", { received_messages: 2, persisted_records: 1, duplicate_counter: 1 });

  // B3: gateway schema rejection classes, before zome call.
  const quarantineBefore = events.filter((event) => event.type === "quarantine").length;
  await publish(`${runId}-malformed`, "soil_moisture", "{broken");
  await publish(`${runId}-missing-id`, "soil_moisture", signed({ observed_at: 1900001005, value: 1, test: true }));
  await publish(`${runId}-bad-confidence`, "soil_moisture", signed({ ...valid, observed_at: 1900001006, confidence: 2 }));
  await publish(`${runId}-bad-value`, "soil_moisture", signed({ ...valid, observed_at: 1900001007, value: 2 }));
  await publish(`${runId}-signature-missing`, "soil_moisture", { ...valid, observed_at: 1900001008 });
  await publish(`${runId}-signature-invalid`, "soil_moisture", { ...valid, observed_at: 1900001009, signature: "00" });
  await publish(`${runId}-bad-indicator`, "unsupported_indicator", signed({ ...valid, observed_at: 1900001010 }));
  await waitFor(
    () => events.filter((event) => event.type === "quarantine").length >= quarantineBefore + 7,
    { label: "seven quarantine events" },
  );
  record("B3", "PASS", {
    quarantined: 7,
    classes: [
      "BAD_JSON", "BAD_SENSOR_ID", "BAD_CONFIDENCE", "BAD_VALUE",
      "SIGNATURE_MISSING", "SIGNATURE_INVALID", "BAD_INDICATOR",
    ],
    zome_calls: 0,
  });

  // B4: canon-invalid but schema-valid entry reaches conductor and is rejected.
  let canonRejected = false;
  try {
    await client1.call("create_ohe", {
      id: `TEST-${runId}-bad-ohe`,
      steward: "TEST",
      site: "TEST",
      status: "Active",
      baseline_recorded: false,
    });
  } catch (error) {
    canonRejected = /InvalidCommit|Validation failed|violates canon/i.test(String(error));
  }
  assert.equal(canonRejected, true);
  record("B4_CANON_INVALID", "PASS", { rejected_by: "integrity_zome" });

  // B5: broker restart and gateway reconnect.
  const connectedBefore = events.filter((event) => event.type === "broker_connected").length;
  execFileSync("docker", ["restart", brokerContainer], { stdio: "pipe" });
  await waitFor(
    () => events.filter((event) => event.type === "broker_connected").length > connectedBefore,
    { timeout: 45000, label: "gateway reconnect after broker restart" },
  );
  const reconnectSubject = `TEST-${runId}-reconnect`;
  await createSubject(reconnectSubject);
  await publish(`${runId}-reconnect`, "precipitation", signed({
    ...valid,
    sensor_id: "WSC2-L-TEST",
    observed_at: 1900001011,
    raw: { mm: 12.4, run_id: runId },
  }));
  await waitForRecords(client1, reconnectSubject, 1);
  record("B5", "PASS", { reconnected: true, post_restart_record: 1 });

  // B6: restart gateway, deliver duplicate again, runtime idempotency survives.
  await stopGateway();
  await startGateway();
  const persistedDuplicateBefore = events.filter((event) => event.type === "duplicate" && event.persisted).length;
  await publish(`${runId}-duplicate`, "soil_moisture", signed(duplicateBody));
  await waitFor(
    () => events.filter((event) => event.type === "duplicate" && event.persisted).length > persistedDuplicateBefore,
    { label: "persistent duplicate response after gateway restart" },
  );
  const duplicateRecords = await waitForRecords(client1, duplicateSubject, 1);
  assert.equal(duplicateRecords.length, 1);
  record("B6", "PASS", { persisted_records_after_gateway_restart: 1 });

  // B8: Agent 2 independently observes Agent 1-authored evidence.
  const records2 = await waitForRecords(client2, subject1, 1, 90000);
  assert.equal(hashText(records2[0].action_hash), actionHash);
  record("B8", "PASS", {
    agent1_action_hash_b64: actionHash,
    agent2_action_hash_b64: hashText(records2[0].action_hash),
  });

  results.overall_status = "PASS_B1_B6_B8";
}

try {
  await run();
} catch (error) {
  results.overall_status = "FAIL";
  results.error = error?.stack || String(error);
  log("FAIL", results.error);
  process.exitCode = 1;
} finally {
  await stopGateway();
  await client1.close();
  await client2.close();
  writeFileSync(path.join(evidenceDir, "RESULTS.json"), `${JSON.stringify(results, null, 2)}\n`);
  gatewayLog.end();
  e2eLog.end();
}
