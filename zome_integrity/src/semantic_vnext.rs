// semantic_vnext.rs
// Candidate vNext semantic domain for PROMETHEUS.
// This module implements P0.2/P0.3 invariants without asserting empirical,
// legal, market, or certification status.
//
// Key boundaries:
// - place/system context is required for observations;
// - relationship assertions are attributable and carry zero PRU weight;
// - disturbances are explicit context objects;
// - review attestations are scope- and COI-bounded;
// - semantic claim identity uses immutable claim_uid rather than bare legacy C-numbers.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceContext {
    pub id: String,
    pub version: String,
    pub site_id: String,
    pub geology_context: String,
    pub hydrology_context: String,
    pub climate_context: String,
    pub land_use_history: String,
    pub stewardship_rights_access: String,
    pub uncertainty_note: String,
}

impl PlaceContext {
    pub fn is_valid(&self) -> bool {
        !self.id.is_empty()
            && !self.version.is_empty()
            && !self.site_id.is_empty()
            && !self.hydrology_context.is_empty()
            && !self.climate_context.is_empty()
            && !self.land_use_history.is_empty()
            && !self.stewardship_rights_access.is_empty()
            && !self.uncertainty_note.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemBoundary {
    pub id: String,
    pub version: String,
    pub site_id: String,
    pub effective_from: i64,
    pub spatial_definition: String,
    pub included_processes: Vec<String>,
    pub excluded_processes: Vec<String>,
    pub external_dependencies: Vec<String>,
    pub rationale: String,
}

impl SystemBoundary {
    pub fn is_valid(&self) -> bool {
        !self.id.is_empty()
            && !self.version.is_empty()
            && !self.site_id.is_empty()
            && self.effective_from > 0
            && !self.spatial_definition.is_empty()
            && !self.included_processes.is_empty()
            && !self.rationale.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservationVnext {
    pub id: String,
    pub subject_id: String,
    pub place_context_version: String,
    pub system_boundary_version: String,
    pub method_id: String,
    pub observed_at: i64,
    pub source_class: String,
    pub observer_or_instrument_id: String,
    pub management_state_version: String,
    pub raw_evidence_refs: Vec<String>,
    pub disturbance_id: Option<String>,
    pub uncertainty_note: String,
}

impl ObservationVnext {
    pub fn is_valid(&self) -> bool {
        !self.id.is_empty()
            && !self.subject_id.is_empty()
            && !self.place_context_version.is_empty()
            && !self.system_boundary_version.is_empty()
            && !self.method_id.is_empty()
            && self.observed_at > 0
            && matches!(
                self.source_class.as_str(),
                "sensor" | "lab" | "earth_observation" | "human_observation"
                    | "operational_log" | "derived_model"
            )
            && !self.observer_or_instrument_id.is_empty()
            && !self.management_state_version.is_empty()
            && !self.raw_evidence_refs.is_empty()
            && !self.uncertainty_note.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationshipAssertion {
    pub id: String,
    pub source_actor_or_method: String,
    pub subject_id: String,
    pub predicate: String,
    pub object_id_or_literal: String,
    pub valid_from: i64,
    pub valid_to: Option<i64>,
    pub provenance_refs: Vec<String>,
    pub uncertainty: String,
    pub epistemic_status: String,
    pub zero_weight_for_pru: bool,
}

impl RelationshipAssertion {
    pub fn is_valid(&self) -> bool {
        let time_ok = self.valid_to.map(|t| t >= self.valid_from).unwrap_or(true);
        !self.id.is_empty()
            && !self.source_actor_or_method.is_empty()
            && !self.subject_id.is_empty()
            && !self.predicate.is_empty()
            && !self.object_id_or_literal.is_empty()
            && self.valid_from > 0
            && time_ok
            && !self.provenance_refs.is_empty()
            && !self.uncertainty.is_empty()
            && matches!(
                self.epistemic_status.as_str(),
                "context" | "hypothesis" | "observation" | "evidence"
            )
            && self.zero_weight_for_pru
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Disturbance {
    pub id: String,
    pub disturbance_type: String,
    pub start_time: i64,
    pub end_time: Option<i64>,
    pub severity_method_id: String,
    pub severity_value: Option<f64>,
    pub spatial_scope: String,
    pub source_refs: Vec<String>,
    pub uncertainty: String,
}

impl Disturbance {
    pub fn is_valid(&self) -> bool {
        let time_ok = self.end_time.map(|t| t >= self.start_time).unwrap_or(true);
        !self.id.is_empty()
            && matches!(
                self.disturbance_type.as_str(),
                "drought" | "heatwave" | "extreme_rainfall" | "flooding"
                    | "pest_or_disease" | "irrigation_interruption"
                    | "management_change" | "market_or_logistics"
                    | "wildfire_or_smoke" | "other"
            )
            && self.start_time > 0
            && time_ok
            && !self.severity_method_id.is_empty()
            && !self.spatial_scope.is_empty()
            && !self.source_refs.is_empty()
            && !self.uncertainty.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewAttestation {
    pub id: String,
    pub reviewer_id: String,
    pub scope: String,
    pub coi_status: String,
    pub subject_refs: Vec<String>,
    pub decision: String,
    pub limitations: String,
    pub independent_for_scope: bool,
    pub reviewed_at: i64,
}

impl ReviewAttestation {
    pub fn is_valid(&self) -> bool {
        !self.id.is_empty()
            && !self.reviewer_id.is_empty()
            && !self.scope.is_empty()
            && matches!(self.coi_status.as_str(), "independent" | "advisory_only" | "conflicted")
            && !self.subject_refs.is_empty()
            && matches!(self.decision.as_str(), "pass" | "conditional" | "fail" | "not_reviewed")
            && !self.limitations.is_empty()
            && self.reviewed_at > 0
            && (self.coi_status != "independent" || self.independent_for_scope)
            && (self.coi_status == "independent" || !self.independent_for_scope)
            && !(self.coi_status == "conflicted" && self.decision == "pass")
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidencePackage {
    pub id: String,
    pub subject_id: String,
    pub claim_uids: Vec<String>,
    pub place_context_version: String,
    pub system_boundary_version: String,
    pub observation_refs: Vec<String>,
    pub method_refs: Vec<String>,
    pub raw_data_hashes: Vec<String>,
    pub transformed_data_hashes: Vec<String>,
    pub package_hash: String,
    pub claims_registry_hash: String,
    pub missing_data_statement: String,
    pub adverse_event_statement: String,
    pub created_at: i64,
}

impl EvidencePackage {
    pub fn is_valid(&self) -> bool {
        !self.id.is_empty()
            && !self.subject_id.is_empty()
            && !self.claim_uids.is_empty()
            && self.claim_uids.iter().all(|uid| valid_semantic_claim_uid(uid))
            && !self.place_context_version.is_empty()
            && !self.system_boundary_version.is_empty()
            && !self.observation_refs.is_empty()
            && !self.method_refs.is_empty()
            && !self.raw_data_hashes.is_empty()
            && !self.package_hash.is_empty()
            && !self.claims_registry_hash.is_empty()
            && self.claims_registry_hash.len() == 64
            && !self.missing_data_statement.is_empty()
            && !self.adverse_event_statement.is_empty()
            && self.created_at > 0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AdmissibilityDecision {
    pub id: String,
    pub subject_id: String,
    pub claim_uid: String,
    pub evidence_package_refs: Vec<String>,
    pub review_attestation_refs: Vec<String>,
    pub legal_gate: bool,
    pub mrv_gate: bool,
    pub confidence: f64,
    pub decision: String,
    pub blockers: Vec<String>,
    pub decided_at: i64,
    pub authority_boundary: String,
}

impl AdmissibilityDecision {
    pub fn is_valid(&self) -> bool {
        let decision_ok = matches!(self.decision.as_str(), "admissible" | "blocked" | "pending");
        let authority_ok = self.authority_boundary == "admissibility_only_no_value";
        let consistency_ok = match self.decision.as_str() {
            "admissible" => self.legal_gate && self.mrv_gate && !self.review_attestation_refs.is_empty() && self.blockers.is_empty(),
            "blocked" => !self.legal_gate || !self.mrv_gate || !self.blockers.is_empty(),
            "pending" => true,
            _ => false,
        };
        !self.id.is_empty()
            && !self.subject_id.is_empty()
            && valid_semantic_claim_uid(&self.claim_uid)
            && !self.evidence_package_refs.is_empty()
            && self.confidence >= 0.0
            && self.confidence <= 1.0
            && decision_ok
            && consistency_ok
            && self.decided_at > 0
            && authority_ok
    }
}

pub fn valid_semantic_claim_uid(uid: &str) -> bool {
    uid.starts_with("prometheus.")
        && uid.len() > "prometheus.".len()
        && !uid.starts_with("prometheus.C-")
        && !uid.contains(' ')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relationship_must_be_zero_weight_for_pru() {
        let base = RelationshipAssertion {
            id: "rel-1".into(),
            source_actor_or_method: "method-place-v0.1".into(),
            subject_id: "ohe-1".into(),
            predicate: "depends_on".into(),
            object_id_or_literal: "external-irrigation".into(),
            valid_from: 1,
            valid_to: None,
            provenance_refs: vec!["note:1".into()],
            uncertainty: "medium".into(),
            epistemic_status: "context".into(),
            zero_weight_for_pru: true,
        };
        assert!(base.is_valid());
        assert!(!RelationshipAssertion { zero_weight_for_pru: false, ..base }.is_valid());
    }

    #[test]
    fn observation_requires_context_and_boundary() {
        let base = ObservationVnext {
            id: "obs-1".into(),
            subject_id: "ohe-1".into(),
            place_context_version: "pc-v0.1".into(),
            system_boundary_version: "sb-v0.1".into(),
            method_id: "method-1".into(),
            observed_at: 1,
            source_class: "sensor".into(),
            observer_or_instrument_id: "sensor-1".into(),
            management_state_version: "ms-v0.1".into(),
            raw_evidence_refs: vec!["sha256:x".into()],
            disturbance_id: None,
            uncertainty_note: "synthetic".into(),
        };
        assert!(base.is_valid());
        assert!(!ObservationVnext { place_context_version: "".into(), ..base }.is_valid());
    }

    #[test]
    fn conflicted_reviewer_cannot_pass_scope() {
        let r = ReviewAttestation {
            id: "rev-1".into(),
            reviewer_id: "reviewer-1".into(),
            scope: "H1 method".into(),
            coi_status: "conflicted".into(),
            subject_refs: vec!["method:h1".into()],
            decision: "pass".into(),
            limitations: "co-designed method".into(),
            independent_for_scope: false,
            reviewed_at: 1,
        };
        assert!(!r.is_valid());
    }

    #[test]
    fn semantic_claim_uid_rejects_legacy_id() {
        assert!(valid_semantic_claim_uid("prometheus.runtime.evaluation_not_certification"));
        assert!(!valid_semantic_claim_uid("C-013"));
        assert!(!valid_semantic_claim_uid("prometheus.C-013"));
    }
}
