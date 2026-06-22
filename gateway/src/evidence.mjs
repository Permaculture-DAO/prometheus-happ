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
// PROMETHEUS_INDICATORS (comma-separated) for new measurement campaigns.
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

export function signReading(reading, key) {
  const unsigned = { ...reading };
  delete unsigned.signature;
  return createHmac("sha256", key).update(canonicalJson(unsigned)).digest("hex");
}

export function verifyReadingSignature(reading, key) {
  if (!reading?.signature || typeof reading.signature !== "string") return false;
  const expected = Buffer.from(signReading(reading, key), "hex");
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
}

/**
 * Build an MrvEvidenceEntry payload from a sensor reading. Validates first (fail-closed).
 * @param {object} reading  { sensor_id, indicator, value, unit, observed_at, raw }
 * @param {object} opts     { subject_id, test (REQUIRED boolean), calibration_hash, confidence, reviewer }
 * @returns {object} payload for the coordinator `create_evidence` zome fn.
 */
export function buildEvidence(reading, opts) {
  validateReading(reading, opts);
  const { sensor_id, indicator, observed_at, raw } = reading;
  const { subject_id, calibration_hash, confidence, reviewer = null, test } = opts;

  // Deterministic id = sensor:indicator:time → correlation id + runtime no-double-counting.
  const id = `${test ? "test:" : ""}${sensor_id}:${indicator}:${Number(observed_at)}`;

  return {
    id,
    // TEST data is namespaced so it can never be mistaken for real evidence.
    subject_id: test ? `TEST-${subject_id}` : subject_id,
    indicator,
    method_hash: calibration_hash || sha256(`sensor:${sensor_id}`), // provenance of HOW
    data_hash: sha256(raw ?? reading),                              // provenance of WHAT
    observed_at: Number(observed_at),
    confidence: typeof confidence === "number" ? confidence : 0.7,
    missing_data: reading.value === null || reading.value === undefined,
    reviewer, // null until an independent reviewer attests this batch (Gate T.0)
  };
}
