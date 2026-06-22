// smoke.mjs — tests the pure + hardened pipeline (no broker, no conductor, no installs).
import assert from "node:assert/strict";
import {
  buildEvidence, sha256, validateReading, EvidenceError, signReading, verifyReadingSignature,
} from "../src/evidence.mjs";

// DRY must be set before importing ingest.mjs (it reads env at module load).
process.env.PROMETHEUS_DRY_RUN = "1";
const { handleMessage, readingToEvidence, stats } = await import("../src/ingest.mjs");
const { submitEvidence } = await import("../src/conductor.mjs");

const throwsCode = (fn, code) => {
  try { fn(); } catch (e) { assert.ok(e instanceof EvidenceError && e.code === code, `expected ${code}, got ${e.code}`); return; }
  assert.fail(`expected throw ${code}`);
};

try {
  const base = { sensor_id: "SE01-LS-01", indicator: "soil_moisture", observed_at: 1900000000, value: 0.31, raw: { v: 0.31 } };

  // 1. buildEvidence: provenance + deterministic id (test flag explicit)
  const ev = buildEvidence(base, { subject_id: "ohe-1", calibration_hash: "calib-abc", confidence: 0.9, reviewer: "rev-1", test: false });
  assert.equal(ev.method_hash, "calib-abc");
  assert.equal(ev.data_hash, sha256({ v: 0.31 }));
  assert.equal(ev.id, "SE01-LS-01:soil_moisture:1900000000");

  // 2. validation is fail-closed
  throwsCode(() => validateReading({ ...base, indicator: "unknown_x" }, { subject_id: "s", test: false }), "BAD_INDICATOR");
  throwsCode(() => validateReading({ ...base, observed_at: -1 }, { subject_id: "s", test: false }), "BAD_TIMESTAMP");
  throwsCode(() => validateReading({ ...base, observed_at: 1.5 }, { subject_id: "s", test: false }), "BAD_TIMESTAMP");
  throwsCode(() => validateReading(base, { subject_id: "s", test: false, confidence: 2 }), "BAD_CONFIDENCE");
  throwsCode(() => validateReading(base, { subject_id: "s", test: false, confidence: NaN }), "BAD_CONFIDENCE");
  throwsCode(() => validateReading({ ...base, value: 2 }, { subject_id: "s", test: false }), "BAD_VALUE");
  throwsCode(() => validateReading(base, { subject_id: "s" }), "TEST_FLAG_REQUIRED");
  throwsCode(() => validateReading({ ...base, sensor_id: "" }, { subject_id: "s", test: false }), "BAD_SENSOR_ID");
  throwsCode(() => validateReading(base, { subject_id: "bad id!", test: false }), "BAD_SUBJECT_ID");

  // 3. test data structurally namespaced
  const t = buildEvidence(base, { subject_id: "ohe-1", test: true });
  assert.ok(t.subject_id.startsWith("TEST-") && t.id.startsWith("test:"), "test data namespaced");

  // 4. topic parsing + bad topic / bad json rejected
  const msg = JSON.stringify({ sensor_id: "WSC2-L-01", observed_at: 1900000100, value: 12.4, raw: { mm: 12.4 } });
  const fromTopic = readingToEvidence("prometheus/ohe-1/precipitation", msg);
  assert.equal(fromTopic.data_hash, sha256({ mm: 12.4 }));
  throwsCode(() => readingToEvidence("bad/topic", msg), "BAD_TOPIC");
  throwsCode(() => readingToEvidence("prometheus/ohe-1/precipitation", "{not json"), "BAD_JSON");

  // 5. real-evidence guard: without PROMETHEUS_ALLOW_REAL, test:false is forced to test
  const guarded = readingToEvidence("prometheus/ohe-1/precipitation", JSON.stringify({ sensor_id: "WSC2-L-01", observed_at: 1900000100, value: 1, test: false }));
  assert.ok(guarded.subject_id.startsWith("TEST-"), "real evidence forced to TEST until authorized");

  // 6. handleMessage: ok -> dedup -> quarantine, stats tracked
  const r1 = await handleMessage("prometheus/ohe-1/soil_moisture", JSON.stringify({ sensor_id: "SE01-LS-01", observed_at: 1900000200, value: 0.3, raw: { v: 0.3 } }));
  assert.ok(r1.ok, "valid message ingested");
  const r2 = await handleMessage("prometheus/ohe-1/soil_moisture", JSON.stringify({ sensor_id: "SE01-LS-01", observed_at: 1900000200, value: 0.3, raw: { v: 0.3 } }));
  assert.ok(r2.deduped, "duplicate deduped (idempotent)");
  const r3 = await handleMessage("prometheus/ohe-1/soil_moisture", "{broken");
  assert.ok(r3.quarantined && r3.code === "BAD_JSON", "malformed quarantined");
  assert.equal(stats.ingested, 1); assert.equal(stats.deduped, 1); assert.equal(stats.quarantined, 1);

  // 7. dry-run submit never touches a conductor
  const res = await submitEvidence(ev, { dryRun: true });
  assert.equal(res.dryRun, true);
  assert.equal(res.created, true);

  // 8. deterministic test HMAC support (enabled by deployment policy, not by default).
  const signed = { ...base, signature: "" };
  signed.signature = signReading(signed, "TEST-key");
  assert.equal(verifyReadingSignature(signed, "TEST-key"), true);
  assert.equal(verifyReadingSignature({ ...signed, value: 0.99 }, "TEST-key"), false);

  console.log("GATEWAY_SMOKE: PASS");
  process.exit(0);
} catch (e) {
  console.error("GATEWAY_SMOKE: FAIL —", e.message);
  process.exit(1);
}
