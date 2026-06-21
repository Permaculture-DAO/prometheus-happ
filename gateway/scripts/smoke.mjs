// smoke.mjs — tests the pure pipeline (no broker, no conductor, no installs needed).
import assert from "node:assert/strict";
import { buildEvidence, sha256 } from "../src/evidence.mjs";
import { readingToEvidence } from "../src/ingest.mjs";
import { submitEvidence } from "../src/conductor.mjs";

try {
  // 1. buildEvidence: provenance present, deterministic id
  const reading = { sensor_id: "SE01-LS-01", indicator: "soil_moisture", observed_at: 1900000000, value: 0.31, raw: { v: 0.31 } };
  const ev = buildEvidence(reading, { subject_id: "ohe-1", calibration_hash: "calib-abc", confidence: 0.9, reviewer: "rev-1" });
  assert.equal(ev.subject_id, "ohe-1");
  assert.equal(ev.indicator, "soil_moisture");
  assert.equal(ev.method_hash, "calib-abc", "method_hash = calibration provenance");
  assert.equal(ev.data_hash, sha256({ v: 0.31 }), "data_hash = raw reading hash");
  assert.equal(ev.observed_at, 1900000000);
  assert.equal(ev.id, "SE01-LS-01:soil_moisture:1900000000", "deterministic id (no-double-counting friendly)");
  assert.equal(ev.reviewer, "rev-1");

  // 2. missing provenance / required fields throw
  assert.throws(() => buildEvidence({ indicator: "x", observed_at: 1 }, { subject_id: "s" }), /sensor_id/);
  assert.throws(() => buildEvidence(reading, {}), /subject_id/);

  // 3. test data is clearly labelled (never mistaken for real evidence)
  const t = buildEvidence(reading, { subject_id: "ohe-1", test: true });
  assert.ok(t.subject_id.startsWith("TEST-"), "test data is labelled TEST-");
  assert.ok(t.id.startsWith("test:"), "test id labelled");

  // 4. MQTT topic -> evidence
  const fromTopic = readingToEvidence("prometheus/ohe-1/precipitation",
    JSON.stringify({ sensor_id: "WSC2-L-01", observed_at: 1900000100, value: 12.4, raw: { mm: 12.4 } }));
  assert.equal(fromTopic.subject_id, "ohe-1");
  assert.equal(fromTopic.indicator, "precipitation");
  assert.equal(fromTopic.data_hash, sha256({ mm: 12.4 }));

  // 5. dry-run submit never touches a conductor
  const res = await submitEvidence(ev, { dryRun: true });
  assert.equal(res.dryRun, true);
  assert.equal(res.would_call, "zome_coordinator.create_evidence");

  console.log("GATEWAY_SMOKE: PASS");
  process.exit(0);
} catch (e) {
  console.error("GATEWAY_SMOKE: FAIL —", e.message);
  process.exit(1);
}
