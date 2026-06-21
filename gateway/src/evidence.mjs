// evidence.mjs — pure provenance logic: a sensor reading -> an MRV evidence payload
// matching the runtime MrvEvidenceEntry. Dependency-free (node:crypto only), so it
// is unit-testable without MQTT or a conductor.
//
// Canon: this records EVIDENCE with provenance — never value, never admissibility.
// The conductor's validate callback rejects anything missing provenance; the
// admissibility gate (assess_subject_admissibility) decides admissible/not later.

import { createHash } from "node:crypto";

export const sha256 = (input) =>
  createHash("sha256").update(typeof input === "string" ? input : JSON.stringify(input)).digest("hex");

/**
 * Build an MrvEvidenceEntry payload from a sensor reading.
 * @param {object} reading  { sensor_id, indicator, value, unit, observed_at, raw }
 * @param {object} opts     { subject_id, calibration_hash, confidence, reviewer, test }
 * @returns {object} payload for the coordinator `create_evidence` zome fn.
 */
export function buildEvidence(reading, opts) {
  const { sensor_id, indicator, observed_at, raw } = reading;
  const { subject_id, calibration_hash, confidence, reviewer = null, test = false } = opts;

  if (!sensor_id || !indicator || !observed_at) {
    throw new Error("reading requires sensor_id, indicator, observed_at");
  }
  if (!subject_id) throw new Error("opts.subject_id is required");

  // Deterministic id = sensor:indicator:time → helps the runtime no-double-counting.
  const id = `${test ? "test:" : ""}${sensor_id}:${indicator}:${observed_at}`;

  return {
    id,
    // TEST data is clearly labelled so it can never be mistaken for real evidence.
    subject_id: test ? `TEST-${subject_id}` : subject_id,
    indicator,
    // method_hash = provenance of HOW: sensor model + calibration record.
    method_hash: calibration_hash || sha256(`sensor:${sensor_id}`),
    // data_hash = provenance of WHAT: hash of the raw reading payload.
    data_hash: sha256(raw ?? reading),
    observed_at: Number(observed_at),
    confidence: typeof confidence === "number" ? confidence : 0.7,
    missing_data: reading.value === null || reading.value === undefined,
    reviewer, // null until an independent reviewer attests this batch (Gate T.0)
  };
}
