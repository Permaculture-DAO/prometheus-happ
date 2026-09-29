// p2_2_e2e_verify.mjs
// Live TEST-only Evidence Spine reconstruction against a running Holochain conductor.
// Proves digital lineage and integrity callbacks only.
// Does NOT prove ecological truth, scientific validation, legal admission, or value.

import { createHash } from "node:crypto";
import { RuntimeClient } from "../src/runtime_client.mjs";

const ADMIN_PORT = Number(process.env.ADMIN_PORT || process.env.PROMETHEUS_ADMIN_PORT || 0);
const APP_PORT = Number(process.env.APP_PORT || process.env.PROMETHEUS_APP_PORT || 0);
if (!ADMIN_PORT) throw new Error("P2.2B requires ADMIN_PORT/PROMETHEUS_ADMIN_PORT for a running TEST conductor");

const client = new RuntimeClient({
  adminPort: ADMIN_PORT,
  appPort: APP_PORT,
  appId: process.env.PROMETHEUS_APP_ID || "prometheus",
  role: process.env.PROMETHEUS_ROLE || "hearth",
  zome: "zome_coordinator",
  origin: process.env.PROMETHEUS_WS_ORIGIN || "http://localhost",
});

const ok = (message) => console.log("  OK  " + message);
const fail = (message) => { console.error("  FAIL " + message); process.exitCode = 1; };
const sha256 = (value) => createHash("sha256").update(value).digest("hex");
const now = Math.floor(Date.now() / 1000);
const run = process.env.PROMETHEUS_TEST_RUN_ID || String(now);
const subject = `TEST-ohe-p2-2-${run}`;
const claimUid = "prometheus.runtime.evaluation_not_certification";
const placeVersion = `pc-TEST-${run}`;
const boundaryVersion = `sb-TEST-${run}`;
const observationId = `obs-TEST-${run}`;
const packageId = `ep-TEST-${run}`;
const reviewId = `rev-TEST-${run}`;
const decisionId = `adm-TEST-${run}`;
const rawHash = sha256(`TEST-raw-${run}`);
const registryHash = sha256("TEST-semantic-registry-v0.2-candidate");
const packageHash = sha256(JSON.stringify({
  subject, claimUid, placeVersion, boundaryVersion, observationId, rawHash, registryHash,
}));

console.log("=== P2.2B live conductor Evidence Spine reconstruction (TEST data only) ===");

try {
  const placeAction = await client.call("create_place_context", {
    id: `pc-entry-${run}`,
    version: placeVersion,
    site_id: subject,
    geology_context: "TEST context only",
    hydrology_context: "TEST context only",
    climate_context: "TEST context only",
    land_use_history: "TEST context only",
    stewardship_rights_access: "TEST access statement",
    uncertainty_note: "synthetic TEST record; no Sicily field inference",
  });
  if (placeAction) ok("PlaceContext committed"); else fail("PlaceContext action hash missing");

  const boundaryAction = await client.call("create_system_boundary", {
    id: `sb-entry-${run}`,
    version: boundaryVersion,
    site_id: subject,
    effective_from: now,
    spatial_definition: "TEST synthetic polygon placeholder",
    included_processes: ["TEST-observation-flow"],
    excluded_processes: ["real-field-performance"],
    external_dependencies: ["TEST conductor"],
    rationale: "P2.2B synthetic lineage proof only",
  });
  if (boundaryAction) ok("SystemBoundary committed"); else fail("SystemBoundary action hash missing");

  const observationAction = await client.call("create_observation", {
    id: observationId,
    subject_id: subject,
    place_context_version: placeVersion,
    system_boundary_version: boundaryVersion,
    method_id: "method-TEST-p2-2-v0.1",
    observed_at: now,
    source_class: "operational_log",
    observer_or_instrument_id: "instrument-TEST-p2-2",
    management_state_version: "ms-TEST-v0.1",
    raw_evidence_refs: [`sha256:${rawHash}`],
    disturbance_id: null,
    uncertainty_note: "synthetic TEST observation",
  });
  if (observationAction) ok("Observation committed"); else fail("Observation action hash missing");

  const packageAction = await client.call("create_evidence_package", {
    id: packageId,
    subject_id: subject,
    claim_uids: [claimUid],
    place_context_version: placeVersion,
    system_boundary_version: boundaryVersion,
    observation_refs: [observationId],
    method_refs: ["method-TEST-p2-2-v0.1"],
    raw_data_hashes: [rawHash],
    transformed_data_hashes: [],
    package_hash: packageHash,
    claims_registry_hash: registryHash,
    missing_data_statement: "synthetic TEST: no missing input declared",
    adverse_event_statement: "synthetic TEST: no adverse-event inference",
    created_at: now,
  });
  if (packageAction) ok("EvidencePackage committed with real ActionHash"); else fail("EvidencePackage action hash missing");

  const packages = await client.call("get_subject_evidence_packages", subject);
  const packageMatch = Array.isArray(packages) && packages.some((row) => row?.package?.id === packageId);
  if (packageMatch) ok("Subject -> EvidencePackage reconstruction"); else fail("EvidencePackage not reconstructable from subject");

  const reviewAction = await client.call("create_review_attestation", {
    id: reviewId,
    reviewer_id: "reviewer-TEST-independent",
    scope: "P2.2B synthetic digital-lineage structure only",
    coi_status: "independent",
    subject_refs: [packageId],
    decision: "pass",
    limitations: "Does not review ecological truth, science, law, market status, or value.",
    independent_for_scope: true,
    reviewed_at: now,
  });
  if (reviewAction) ok("ReviewAttestation committed"); else fail("ReviewAttestation action hash missing");

  const reviews = await client.call("get_package_reviews", packageId);
  const reviewMatch = Array.isArray(reviews) && reviews.some((row) => row?.review?.id === reviewId);
  if (reviewMatch) ok("EvidencePackage -> ReviewAttestation reconstruction"); else fail("Review not reconstructable from package");

  // Deliberately blocked: the synthetic proof has no independent legal admission.
  const decisionAction = await client.call("create_admissibility_decision", {
    id: decisionId,
    subject_id: subject,
    claim_uid: claimUid,
    evidence_package_refs: [packageId],
    review_attestation_refs: [reviewId],
    legal_gate: false,
    mrv_gate: true,
    confidence: 0.90,
    decision: "blocked",
    blockers: ["legal gate not established"],
    decided_at: now,
    authority_boundary: "admissibility_only_no_value",
  });
  if (decisionAction) ok("Blocked AdmissibilityDecision committed"); else fail("AdmissibilityDecision action hash missing");

  const decisions = await client.call("get_claim_admissibility_decisions", claimUid);
  const decisionMatch = Array.isArray(decisions) && decisions.some((row) =>
    row?.decision?.id === decisionId &&
    row?.decision?.decision === "blocked" &&
    row?.decision?.authority_boundary === "admissibility_only_no_value"
  );
  if (decisionMatch) ok("Claim -> blocked AdmissibilityDecision reconstruction"); else fail("Decision not reconstructable from semantic claim");

  // Negative integrity test: a conflicted reviewer may not pass the same scope.
  try {
    await client.call("create_review_attestation", {
      id: `rev-TEST-conflicted-${run}`,
      reviewer_id: "reviewer-TEST-conflicted",
      scope: "same synthetic scope",
      coi_status: "conflicted",
      subject_refs: [packageId],
      decision: "pass",
      limitations: "must be rejected",
      independent_for_scope: false,
      reviewed_at: now,
    });
    fail("conflicted passing review was not rejected");
  } catch (_error) {
    ok("Conflicted passing review rejected by integrity validation");
  }

  const result = {
    schema_version: "p2.2b-0.1",
    run_id: run,
    overall_status: process.exitCode ? "FAIL" : "PASS",
    data_class: "synthetic_TEST",
    subject_id: subject,
    semantic_claim_uid: claimUid,
    action_presence: {
      place_context: Boolean(placeAction),
      system_boundary: Boolean(boundaryAction),
      observation: Boolean(observationAction),
      evidence_package: Boolean(packageAction),
      review_attestation: Boolean(reviewAction),
      admissibility_decision: Boolean(decisionAction),
    },
    reconstruction: {
      subject_to_package: packageMatch,
      package_to_review: reviewMatch,
      claim_to_decision: decisionMatch,
    },
    expected_decision: "blocked",
    authority_boundary: "admissibility_only_no_value",
    claim_boundary: {
      ecological_truth: false,
      scientific_validation: false,
      certification: false,
      legal_admission: false,
      market_admission: false,
      pru_value: false,
    },
  };
  console.log(JSON.stringify(result, null, 2));
  console.log(process.exitCode ? "P2.2B: FAIL" : "P2.2B: PASS");
} finally {
  await client.close();
  process.exit(process.exitCode || 0);
}
