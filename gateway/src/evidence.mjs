// evidence.mjs — pure provenance logic + input validation: a sensor reading -> an
// MRV evidence payload matching the runtime MrvEvidenceEntry. Dependency-free
// (node:crypto only), so it is unit-testable without MQTT or a conductor.
//
// Canon: this records EVIDENCE with provenance — never value, never admissibility.
// The conductor's validate callback rejects anything missing provenance; the
// admissibility gate (assess_subject_admissibility) decides admissible/not later.

import { createHash, createHmac, timingSafeEqual } from "node:crypto";

export const sha256 = (input) =>
  createHash("sha256").update(typeof input === "string" ? input : JSON.stringify(input)).digest("hex");

// Indicators the pilot recognises (from MRV_BASELINE_PROTOCOL_SICILY.md). Unsupported
// indicators are rejected — they cannot silently become evidence. Override/extend via
// PROMETHEUS_INDICATORS (comma-separated) only for a controlled measurement campaign.
export const ALLOWED_INDICATORS = new Set(
  (process.env.PROMETHEUS_INDICATORS
    ? process.env.PROMETHEUS_INDICATORS.split(",").map((s) => s.trim())
    : [
        "soil_moisture", "precipitation", "air_temperature", "solar_radiation",
        "wind", "atmospheric_pressure", "water_holding_capacity", "humidity",
        "co2", "voc", "soil_organic_matter", "bulk_density", "texture", "pH",
        "N_P_K", "infiltration_rate", "ndvi", "biodiversity_count",
        "water_input", "fertiliser_input", "fuel_input", "labour_hours",
        "yield_per_species", "output_variance", "stress_event",
        "land_equivalent_ratio", "geo_photo",
      ]).filter(Boolean)
);

const ID_RE = /^[A-Za-z0-9._:-]{1,128}$/;
const SHA256_RE = /^[a-f0-9]{64}$/i;
const INDICATOR_RANGES = new Map([
  ["soil_moisture", [0, 1]],
  ["humidity", [0, 100]],
  ["precipitation", [0, Number.POSITIVE_INFINITY]],
]);

export class EvidenceError extends Error {
  constructor(message, code) { super(message); this.name = "EvidenceError"; this.code = code; }
}

function canonicalJson(value) {
  if (Array.isArray(value)) return `[${value.map(canonicalJson).join(",")}]`;
  if (value && typeof value === "object") {
    return `{${Object.keys(value).sort().map((key) =>
      `${JSON.stringify(key)}:${canonicalJson(value[key])}`).join(",")}}`;
  }
  return JSON.stringify(value);
}

export function signReading(reading, key, routing = {}) {
  const unsigned = { ...reading };
  delete unsigned.signature;
  const envelope = {
    subject_id: routing.subject_id ?? null,
    indicator: routing.indicator ?? null,
    reading: unsigned,
  };
  return createHmac("sha256", key).update(canonicalJson(envelope)).digest("hex");
}

export function verifyReadingSignature(reading, key, routing = {}) {
  if (!reading?.signature || typeof reading.signature !== "string") return false;
  const expected = Buffer.from(signReading(reading, key, routing), "hex");
  let supplied;
  try { supplied = Buffer.from(reading.signature, "hex"); } catch { return false; }
  return supplied.length === expected.length && timingSafeEqual(supplied, expected);
}

/** Validate a raw reading. Throws EvidenceError (fail-closed). */
export function validateReading(reading, opts) {
  const fail = (m, c) => { throw new EvidenceError(m, c); };
  if (!reading || typeof reading !== "object") fail("reading must be an object", "BAD_READING");
  const { sensor_id, indicator, observed_at, value } = reading;
  if (!sensor_id || !ID_RE.test(String(sensor_id))) fail("invalid sensor_id", "BAD_SENSOR_ID");
  if (!indicator || !ALLOWED_INDICATORS.has(String(indicator))) fail(`unsupported indicator: ${indicator}`, "BAD_INDICATOR");
  const ts = Number(observed_at);
  if (!Number.isInteger(ts) || ts <= 0 || ts > 4102444800) fail("observed_at must be a unix-second integer", "BAD_TIMESTAMP");
  if (value !== null && value !== undefined) {
    const numeric = Number(value);
    if (!Number.isFinite(numeric)) fail("value must be finite", "BAD_VALUE");
    const range = INDICATOR_RANGES.get(String(indicator));
    if (range && (numeric < range[0] || numeric > range[1])) {
      fail(`value outside admitted range for ${indicator}`, "BAD_VALUE");
    }
  }
  const confidence = opts?.confidence;
  if (confidence !== undefined && confidence !== null) {
    const c = Number(confidence);
    if (!Number.isFinite(c) || c < 0 || c > 1) fail("confidence must be finite in [0,1]", "BAD_CONFIDENCE");
  }
  if (!opts || !opts.subject_id || !ID_RE.test(String(opts.subject_id))) fail("invalid subject_id", "BAD_SUBJECT_ID");
  if (typeof opts.test !== "boolean") fail("opts.test must be an explicit boolean (no silent default)", "TEST_FLAG_REQUIRED");

  // Synthetic records may use generated method provenance. Real field-sensor evidence may
  // not: it must bind to an actual method/calibration artefact with a SHA-256 digest.
  if (opts.test === false) {
    if (!opts.calibration_hash) fail("real sensor evidence requires calibration_hash", "REAL_CALIBRATION_REQUIRED");
    if (!SHA256_RE.test(String(opts.calibration_hash))) {
      fail("real calibration_hash must be a SHA-256 hex digest", "REAL_CALIBRATION_HASH_INVALID");
    }
  }
}

/**
 * Build an MrvEvidenceEntry payload from a sensor reading. Validates first (fail-closed).
 * @param {object} reading  { sensor_id, indicator, value, unit, observed_at, raw }
 * @param {object} opts     { subject_id, test (REQUIRED boolean), calibration_hash, confidence }
 * @returns {object} payload for the coordinator `create_evidence` zome fn.
 */
export function buildEvidence(reading, opts) {
  validateReading(reading, opts);
  const { sensor_id, indicator, observed_at, raw } = reading;
  const { subject_id, calibration_hash, confidence, test } = opts;

  // Gateway correlation/dedup identity includes subject + sensor + indicator + time.
  // Durable Holochain idempotency is enforced separately at the persistence boundary.
  const id = `${test ? "test:" : ""}${subject_id}:${sensor_id}:${indicator}:${Number(observed_at)}`;

  return {
    id,
    // TEST data is namespaced so it can never be mistaken for real evidence.
    subject_id: test ? `TEST-${subject_id}` : subject_id,
    sensor_id,
    indicator,
    evidence_class: test ? "TEST" : "REAL",
    calibration_hash: calibration_hash || null,
    method_hash: calibration_hash || sha256(`sensor:${sensor_id}`), // TEST fallback only
    data_hash: sha256(raw ?? reading),                              // provenance of WHAT
    observed_at: Number(observed_at),
    confidence: typeof confidence === "number" ? confidence : 0.7,
    missing_data: reading.value === null || reading.value === undefined,
    // Acquisition payloads cannot self-declare review/admissibility.
    // Independent reviewer attestation is a separate persistence workflow.
    reviewer: null,
  };
}
