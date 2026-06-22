// ingest.mjs — MQTT -> evidence -> conductor. Runs on the gateway (Raspberry Pi 5).
// Hardened: topic/payload validation, fail-closed, idempotency, quarantine, bounded
// retry, safe logging (never logs raw payloads or credentials).
//
// Canon guard: until authorized field collection, the gateway is STRUCTURALLY
// incapable of writing unlabelled real evidence — set PROMETHEUS_ALLOW_REAL=1 to
// permit real (non-test) evidence. Otherwise every reading is forced to test.
//
// Topic: prometheus/<subject_id>/<indicator>  payload JSON:
//   { sensor_id, value, unit, observed_at, calibration_hash?, confidence?, reviewer?, raw?, test? }

import { buildEvidence, EvidenceError, verifyReadingSignature } from "./evidence.mjs";
import { submitEvidence, close } from "./conductor.mjs";

const DRY = process.env.PROMETHEUS_DRY_RUN === "1";
const ALLOW_REAL = process.env.PROMETHEUS_ALLOW_REAL === "1";
const MAX_RETRIES = Number(process.env.PROMETHEUS_MAX_RETRIES || 3);
const REQUIRE_SIGNATURE = process.env.PROMETHEUS_REQUIRE_SIGNATURE === "1";
const HMAC_KEY = process.env.PROMETHEUS_HMAC_KEY || "";
const TOPIC_RE = /^prometheus\/[A-Za-z0-9._:-]{1,128}\/[A-Za-z0-9._:-]{1,64}$/;

const seen = new Set();          // idempotency: evidence ids already submitted
export const stats = { ingested: 0, deduped: 0, quarantined: 0, failed: 0 };

const log = (...a) => console.log("ingest:", ...a); // status only; never raw payloads
const event = (type, detail = {}) =>
  console.log("ingest:event", JSON.stringify({ type, ...detail }));

function safeBrokerUrl(value) {
  try {
    const parsed = new URL(value);
    parsed.username = "";
    parsed.password = "";
    return parsed.toString();
  } catch {
    return "<invalid-mqtt-url>";
  }
}

/** Parse a topic+message into a validated evidence payload. Throws EvidenceError. */
export function readingToEvidence(topic, msg, { defaultSubject } = {}) {
  if (!TOPIC_RE.test(topic)) throw new EvidenceError(`bad topic shape: ${topic}`, "BAD_TOPIC");
  const [, subject_id, indicator] = topic.split("/");
  let body;
  try { body = typeof msg === "string" ? JSON.parse(msg) : msg; }
  catch { throw new EvidenceError("payload is not valid JSON", "BAD_JSON"); }
  if (!body || typeof body !== "object") throw new EvidenceError("payload not an object", "BAD_JSON");
  if (REQUIRE_SIGNATURE) {
    if (!HMAC_KEY) throw new EvidenceError("signature verification key unavailable", "SIGNATURE_CONFIG");
    if (!body.signature) throw new EvidenceError("payload signature is required", "SIGNATURE_MISSING");
    if (!verifyReadingSignature(body, HMAC_KEY)) throw new EvidenceError("payload signature is invalid", "SIGNATURE_INVALID");
  }
  // Fail-closed real-evidence guard: real only with explicit authorization.
  const requestedTest = body.test === true;
  const test = ALLOW_REAL ? requestedTest : true;
  return buildEvidence(
    {
      sensor_id: body.sensor_id,
      indicator,
      observed_at: body.observed_at,
      value: body.value,
      raw: body.raw ?? body,
    },
    { subject_id: subject_id || defaultSubject, calibration_hash: body.calibration_hash,
      confidence: body.confidence, reviewer: body.reviewer ?? null, test }
  );
}

/** Submit with bounded retry on transient errors. Validation errors do NOT retry. */
async function submitWithRetry(ev) {
  for (let attempt = 1; ; attempt++) {
    try { return await submitEvidence(ev, { dryRun: DRY }); }
    catch (e) {
      if (attempt >= MAX_RETRIES) throw e;
      await new Promise((r) => setTimeout(r, 500 * attempt));
    }
  }
}

export async function handleMessage(topic, raw, opts = {}) {
  let ev;
  try { ev = readingToEvidence(topic, raw, opts); }
  catch (e) {
    stats.quarantined++;
    event("quarantine", { code: e.code || "ERR", error: e.message });
    log("QUARANTINE", e.code || "ERR", "-", e.message);
    return { quarantined: true, code: e.code };
  }
  if (seen.has(ev.id)) {
    stats.deduped++;
    event("duplicate", { id: ev.id, persisted: false });
    log("dedup", ev.id);
    return { deduped: true, id: ev.id };
  }
  try {
    const res = await submitWithRetry(ev);
    seen.add(ev.id);
    if (res.created === false) {
      stats.deduped++;
      event("duplicate", { id: ev.id, persisted: true });
      return { deduped: true, persisted: true, id: ev.id, res };
    }
    stats.ingested++;
    event("stored", { id: ev.id, created: true });
    log(ev.id, DRY ? "(dry-run)" : (res.actionHash ? "stored" : "ok"));
    return { ok: true, id: ev.id, res };
  } catch (e) {
    stats.failed++;
    event("failed", { id: ev.id, error: e.message });
    log("FAILED", ev.id, e.message);
    return { failed: true, id: ev.id };
  }
}

export async function start() {
  const { default: mqtt } = await import("mqtt");
  const url = process.env.MQTT_URL || "mqtt://127.0.0.1:1883";
  const topic = process.env.MQTT_TOPIC || "prometheus/+/+";
  const client = mqtt.connect(url, {
    username: process.env.MQTT_USERNAME,
    password: process.env.MQTT_PASSWORD,
    reconnectPeriod: Number(process.env.MQTT_RECONNECT_MS || 1000),
  });
  client.on("connect", () => {
    client.subscribe(topic, { qos: 1 });
    const loggedUrl = safeBrokerUrl(url);
    event("broker_connected", { url: loggedUrl, topic });
    log(`subscribed ${topic} on ${loggedUrl} (dryRun=${DRY}, allowReal=${ALLOW_REAL})`);
  });
  client.on("reconnect", () => event("broker_reconnect"));
  client.on("offline", () => event("broker_offline"));
  client.on("error", (error) => event("broker_error", { error: error.message }));
  client.on("message", (t, payload) =>
    handleMessage(t, payload.toString(), { defaultSubject: process.env.PROMETHEUS_SUBJECT_ID }));
  const shutdown = async () => {
    log("stats", JSON.stringify(stats));
    client.end(true);
    await close();
    process.exit(0);
  };
  process.on("SIGINT", shutdown);
  process.on("SIGTERM", shutdown);
  return client;
}

if (process.argv[1] && process.argv[1].endsWith("ingest.mjs")) start();
