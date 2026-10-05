// smoke.mjs — tests the pure + hardened pipeline (no broker, no conductor, no installs).
import assert from "node:assert/strict";
import {
  buildEvidence, sha256, validateReading, EvidenceError, signReading, verifyReadingSignature,
} from "../src/evidence.mjs";

// DRY must be set before importing ingest.mjs (it reads env at module load).
process.env.PROMETHEUS_DRY_RUN = "1";
const { handleMessage, readingToEvidence, validateRealSubject, stats } = await import("../src/ingest.mjs");
const { submitEvidence } = await import("../src/conductor.mjs");

const throwsCode = (fn, code) => {
  try { fn(); } catch (e) { assert.ok(e instanceof EvidenceError && e.code === code, `expected ${code}, got ${e.code}`); return; }
  assert.fail(`expected throw ${code}`);
};

try {
  const base = { sensor_id: "SE01-LS-01", indicator: "soil_moisture", observed_at: 1900000000, value: 0.31, raw: { v: 0.31 } };
  const calibrationHash = sha256("SE01-LS-01 calibration record v1");

  // 1. buildEvidence: provenance + deterministic id (test flag explicit)
  const ev = buildEvidence(base, { subject_id: "ohe-1", calibration_hash: calibrationHash, confidence: 0.9, test: false });
  assert.equal(ev.sensor_id, "SE01-LS-01");
  assert.equal(ev.evidence_class, "REAL");
  assert.equal(ev.calibration_hash, calibrationHash);
  assert.equal(ev.method_hash, calibrationHash);
  assert.equal(ev.data_hash, sha256({ v: 0.31 }));
  assert.equal(ev.id, "ohe-1:SE01-LS-01:soil_moisture:1900000000");
  assert.equal(ev.reviewer, null, "acquisition cannot self-attest reviewer");

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
  throwsCode(() => validateReading(base, { subject_id: "s", test: false }), "REAL_CALIBRATION_REQUIRED");
  throwsCode(() => validateReading(base, { subject_id: "s", test: false, calibration_hash: "not-a-sha256" }), "REAL_CALIBRATION_HASH_INVALID");

  // 3. test data structurally namespaced and may use generated TEST method provenance
  const t = buildEvidence(base, { subject_id: "ohe-1", test: true });
  assert.ok(t.subject_id.startsWith("TEST-") && t.id.startsWith("test:"), "test data namespaced");
  assert.equal(t.sensor_id, "SE01-LS-01");
  assert.equal(t.evidence_class, "TEST");
  assert.equal(t.calibration_hash, null);

  // 4. topic parsing + bad topic / bad json rejected
  const msg = JSON.stringify({ sensor_id: "WSC2-L-01", observed_at: 1900000100, value: 12.4, raw: { mm: 12.4 } });
  const fromTopic = readingToEvidence("prometheus/ohe-1/precipitation", msg);
  assert.equal(fromTopic.data_hash, sha256({ mm: 12.4 }));
  throwsCode(() => readingToEvidence("bad/topic", msg), "BAD_TOPIC");
  throwsCode(() => readingToEvidence("prometheus/ohe-1/precipitation", "{not json"), "BAD_JSON");

  // 5. real-evidence guard: without PROMETHEUS_ALLOW_REAL, test:false is forced to test
  const guarded = readingToEvidence("prometheus/ohe-1/precipitation", JSON.stringify({ sensor_id: "WSC2-L-01", observed_at: 1900000100, value: 1, test: false }));
  assert.ok(guarded.subject_id.startsWith("TEST-"), "real evidence forced to TEST until authorized");

  // 6. real-mode subject policy is explicit and fail-closed
  throwsCode(() => validateRealSubject("ohe-1", { allowReal: true, requestedTest: false, allowedSubjects: new Set() }), "REAL_SUBJECTS_CONFIG");
  throwsCode(() => validateRealSubject("rogue-subject", { allowReal: true, requestedTest: false, allowedSubjects: new Set(["ohe-1", "cmp-1"]) }), "SUBJECT_NOT_AUTHORIZED");
  validateRealSubject("ohe-1", { allowReal: true, requestedTest: false, allowedSubjects: new Set(["ohe-1", "cmp-1"]) });

  // 7. handleMessage: ok -> dedup -> quarantine, stats tracked
  const r1 = await handleMessage("prometheus/ohe-1/soil_moisture", JSON.stringify({ sensor_id: "SE01-LS-01", observed_at: 1900000200, value: 0.3, raw: { v: 0.3 } }));
  assert.ok(r1.ok, "valid message ingested");
  const r2 = await handleMessage("prometheus/ohe-1/soil_moisture", JSON.stringify({ sensor_id: "SE01-LS-01", observed_at: 1900000200, value: 0.3, raw: { v: 0.3 } }));
  assert.ok(r2.deduped, "duplicate deduped (idempotent)");
  const r3 = await handleMessage("prometheus/ohe-1/soil_moisture", "{broken");
  assert.ok(r3.quarantined && r3.code === "BAD_JSON", "malformed quarantined");
  assert.equal(stats.ingested, 1); assert.equal(stats.deduped, 1); assert.equal(stats.quarantined, 1);

  // 8. dry-run submit never touches a conductor
  const res = await submitEvidence(ev, { dryRun: true });
  assert.equal(res.dryRun, true);
  assert.equal(res.created, true);

  // 9. deterministic test HMAC support (enabled by deployment policy when configured).
  const signed = { ...base, signature: "" };
  const routing = { subject_id: "ohe-1", indicator: "soil_moisture" };
  signed.signature = signReading(signed, "TEST-key", routing);
  assert.equal(verifyReadingSignature(signed, "TEST-key", routing), true);
  assert.equal(verifyReadingSignature({ ...signed, value: 0.99 }, "TEST-key", routing), false);
  assert.equal(
    verifyReadingSignature(signed, "TEST-key", { subject_id: "cmp-1", indicator: "soil_moisture" }),
    false,
    "signed payload cannot be rerouted to another subject"
  );
  assert.equal(
    verifyReadingSignature(signed, "TEST-key", { subject_id: "ohe-1", indicator: "precipitation" }),
    false,
    "signed payload cannot be rerouted to another indicator"
  );

  // 10. reviewer supplied by a sensor/operator has no admissibility effect.
  const selfReviewed = readingToEvidence(
    "prometheus/ohe-1/soil_moisture",
    JSON.stringify({ sensor_id: "SE01-LS-02", observed_at: 1900000300, value: 0.4, reviewer: "self-declared" })
  );
  assert.equal(selfReviewed.reviewer, null);

  console.log("GATEWAY_SMOKE: PASS");
  process.exit(0);
} catch (e) {
  console.error("GATEWAY_SMOKE: FAIL —", e.message);
  process.exit(1);
}
