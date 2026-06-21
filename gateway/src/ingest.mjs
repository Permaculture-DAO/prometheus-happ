// ingest.mjs — MQTT -> evidence -> conductor. Runs on the gateway (Raspberry Pi 5).
// Subscribes to the Mosquitto broker, builds a provenance-anchored MRV evidence for
// each aggregated reading, and submits it to the local conductor.
//
// Env: MQTT_URL (default mqtt://127.0.0.1:1883), MQTT_TOPIC (default prometheus/+/+),
//      PROMETHEUS_SUBJECT_ID, PROMETHEUS_DRY_RUN=1 to not touch the conductor.
//
// Topic convention: prometheus/<subject_id>/<indicator>, payload JSON:
//   { sensor_id, value, unit, observed_at, calibration_hash?, confidence?, raw? }

import { buildEvidence } from "./evidence.mjs";
import { submitEvidence, close } from "./conductor.mjs";

const DRY = process.env.PROMETHEUS_DRY_RUN === "1";

export function readingToEvidence(topic, msg, { defaultSubject } = {}) {
  const parts = topic.split("/"); // prometheus/<subject>/<indicator>
  const subject_id = parts[1] || defaultSubject;
  const indicator = parts[2];
  const body = typeof msg === "string" ? JSON.parse(msg) : msg;
  return buildEvidence(
    { sensor_id: body.sensor_id, indicator, observed_at: body.observed_at, value: body.value, raw: body.raw ?? body },
    { subject_id, calibration_hash: body.calibration_hash, confidence: body.confidence, reviewer: body.reviewer ?? null, test: body.test === true }
  );
}

export async function start() {
  const { default: mqtt } = await import("mqtt");
  const url = process.env.MQTT_URL || "mqtt://127.0.0.1:1883";
  const topic = process.env.MQTT_TOPIC || "prometheus/+/+";
  const client = mqtt.connect(url);
  client.on("connect", () => { client.subscribe(topic); console.log(`ingest: subscribed ${topic} on ${url} (dryRun=${DRY})`); });
  client.on("message", async (t, payload) => {
    try {
      const ev = readingToEvidence(t, payload.toString(), { defaultSubject: process.env.PROMETHEUS_SUBJECT_ID });
      const res = await submitEvidence(ev, { dryRun: DRY });
      console.log("ingest:", ev.id, DRY ? "(dry-run)" : (res.actionHash ? "stored" : "?"));
    } catch (e) { console.error("ingest error:", e.message); }
  });
  process.on("SIGINT", async () => { await close(); process.exit(0); });
}

if (process.argv[1] && process.argv[1].endsWith("ingest.mjs")) start();
