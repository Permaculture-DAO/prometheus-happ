// entry_types.rs
// Real Holochain entry types for the integrity zome.
// Candidate vNext additions preserve the authority boundary:
// context/provenance may be recorded; no entry here creates PRU value,
// ecological truth, certification, legal rights, or market admission.

use hdk::prelude::*;
use hdi::prelude::{FlatOp, OpEntry};

use crate::claim::{Claim, ClaimStatus};
use crate::mrv::MrvEvidence;
use crate::ohe::{Ohe, OheStatus};
use crate::semantic_vnext::{
    valid_semantic_claim_uid, AdmissibilityDecision, Disturbance, EvidencePackage,
    ObservationVnext, PlaceContext, RelationshipAssertion, ReviewAttestation, SystemBoundary,
};
use crate::valueflows::ids::AgentId;

// ---------- OHE ----------
#[hdk_entry_helper]
#[derive(Clone)]
pub struct OheEntry {
    pub id: String,
    pub steward: String,
    pub site: String,
    pub status: String,
    pub baseline_recorded: bool,
}

impl OheEntry {
    fn to_domain(&self) -> Option<Ohe> {
        let status = match self.status.as_str() {
            "Proposed" => OheStatus::Proposed,
            "BaselineRecorded" => OheStatus::BaselineRecorded,
            "Active" => OheStatus::Active,
            "Terminated" => OheStatus::Terminated,
            _ => return None,
        };
        Some(Ohe {
            id: self.id.clone(),
            steward: AgentId(self.steward.clone()),
            site: self.site.clone(),
            status,
            baseline_recorded: self.baseline_recorded,
        })
    }
    pub fn validate_entry(&self) -> ValidateCallbackResult {
        match self.to_domain() {
            Some(d) if d.is_valid() => ValidateCallbackResult::Valid,
            Some(_) => ValidateCallbackResult::Invalid(
                "OHE entry violates canon invariants (e.g. Active without baseline, missing steward/site)".into(),
            ),
            None => ValidateCallbackResult::Invalid(format!("unknown OHE status: {}", self.status)),
        }
    }
}

// ---------- MRV EVIDENCE ----------
#[hdk_entry_helper]
#[derive(Clone)]
pub struct MrvEvidenceEntry {
    pub id: String,
    pub subject_id: String,
    pub indicator: String,
    pub method_hash: String,
    pub data_hash: String,
    pub observed_at: i64,
    pub confidence: f64,
    pub missing_data: bool,
    pub reviewer: Option<String>,
}

impl MrvEvidenceEntry {
    pub fn to_domain(&self) -> MrvEvidence {
        use crate::mrv::EvidenceSource;
        MrvEvidence {
            id: self.id.clone(),
            subject_id: self.subject_id.clone(),
            indicator: self.indicator.clone(),
            source: EvidenceSource::OperationalLog,
            method_hash: self.method_hash.clone(),
            data_hash: self.data_hash.clone(),
            observed_at: self.observed_at,
            confidence: self.confidence,
            missing_data: self.missing_data,
            reviewer: self.reviewer.clone(),
        }
    }
    pub fn validate_entry(&self) -> ValidateCallbackResult {
        if self.to_domain().is_valid() {
            ValidateCallbackResult::Valid
        } else {
            ValidateCallbackResult::Invalid(
                "MRV evidence entry violates canon invariants (missing provenance, bad confidence, etc.)".into(),
            )
        }
    }
}

// ---------- CLAIM ----------
#[hdk_entry_helper]
#[derive(Clone)]
pub struct ClaimEntry {
    pub id: String,
    pub statement: String,
    pub status: String,
    pub investor_facing: bool,
}

impl ClaimEntry {
    fn to_domain(&self) -> Option<Claim> {
        let status = match self.status.as_str() {
            "Architectural" => ClaimStatus::Architectural,
            "Methodological" => ClaimStatus::Methodological,
            "Hypothesis" => ClaimStatus::Hypothesis,
            "PilotObserved" => ClaimStatus::PilotObserved,
            "ThirdPartyReviewed" => ClaimStatus::ThirdPartyReviewed,
            "LegallyAdmitted" => ClaimStatus::LegallyAdmitted,
            "MarketAdmitted" => ClaimStatus::MarketAdmitted,
            _ => return None,
        };
        Some(Claim {
            id: self.id.clone(),
            statement: self.statement.clone(),
            status,
            investor_facing: self.investor_facing,
        })
    }
    pub fn validate_entry(&self) -> ValidateCallbackResult {
        match self.to_domain() {
            Some(c) if c.is_disciplined() => ValidateCallbackResult::Valid,
            Some(_) => ValidateCallbackResult::Invalid(
                "claim is overclaiming: investor-facing claims must be at least PilotObserved".into(),
            ),
            None => ValidateCallbackResult::Invalid(format!("unknown claim status: {}", self.status)),
        }
    }
}

// ---------- PLACE CONTEXT ----------
#[hdk_entry_helper]
#[derive(Clone)]
pub struct PlaceContextEntry {
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
impl PlaceContextEntry {
    fn to_domain(&self) -> PlaceContext {
        PlaceContext {
            id: self.id.clone(), version: self.version.clone(), site_id: self.site_id.clone(),
            geology_context: self.geology_context.clone(), hydrology_context: self.hydrology_context.clone(),
            climate_context: self.climate_context.clone(), land_use_history: self.land_use_history.clone(),
            stewardship_rights_access: self.stewardship_rights_access.clone(),
            uncertainty_note: self.uncertainty_note.clone(),
        }
    }
    pub fn validate_entry(&self) -> ValidateCallbackResult {
        if self.to_domain().is_valid() { ValidateCallbackResult::Valid }
        else { ValidateCallbackResult::Invalid("PlaceContext is incomplete".into()) }
    }
}

// ---------- SYSTEM BOUNDARY ----------
#[hdk_entry_helper]
#[derive(Clone)]
pub struct SystemBoundaryEntry {
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
impl SystemBoundaryEntry {
    fn to_domain(&self) -> SystemBoundary {
        SystemBoundary {
            id: self.id.clone(), version: self.version.clone(), site_id: self.site_id.clone(),
            effective_from: self.effective_from, spatial_definition: self.spatial_definition.clone(),
            included_processes: self.included_processes.clone(), excluded_processes: self.excluded_processes.clone(),
            external_dependencies: self.external_dependencies.clone(), rationale: self.rationale.clone(),
        }
    }
    pub fn validate_entry(&self) -> ValidateCallbackResult {
        if self.to_domain().is_valid() { ValidateCallbackResult::Valid }
        else { ValidateCallbackResult::Invalid("SystemBoundary is incomplete or invalid".into()) }
    }
}

// ---------- OBSERVATION vNEXT ----------
#[hdk_entry_helper]
#[derive(Clone)]
pub struct ObservationEntry {
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
impl ObservationEntry {
    fn to_domain(&self) -> ObservationVnext {
        ObservationVnext {
            id: self.id.clone(), subject_id: self.subject_id.clone(),
            place_context_version: self.place_context_version.clone(),
            system_boundary_version: self.system_boundary_version.clone(),
            method_id: self.method_id.clone(), observed_at: self.observed_at,
            source_class: self.source_class.clone(),
            observer_or_instrument_id: self.observer_or_instrument_id.clone(),
            management_state_version: self.management_state_version.clone(),
            raw_evidence_refs: self.raw_evidence_refs.clone(),
            disturbance_id: self.disturbance_id.clone(),
            uncertainty_note: self.uncertainty_note.clone(),
        }
    }
    pub fn validate_entry(&self) -> ValidateCallbackResult {
        if self.to_domain().is_valid() { ValidateCallbackResult::Valid }
        else { ValidateCallbackResult::Invalid("Observation lacks required context/provenance".into()) }
    }
}

// ---------- RELATIONSHIP ASSERTION ----------
#[hdk_entry_helper]
#[derive(Clone)]
pub struct RelationshipAssertionEntry {
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
impl RelationshipAssertionEntry {
    fn to_domain(&self) -> RelationshipAssertion {
        RelationshipAssertion {
            id: self.id.clone(), source_actor_or_method: self.source_actor_or_method.clone(),
            subject_id: self.subject_id.clone(), predicate: self.predicate.clone(),
            object_id_or_literal: self.object_id_or_literal.clone(), valid_from: self.valid_from,
            valid_to: self.valid_to, provenance_refs: self.provenance_refs.clone(),
            uncertainty: self.uncertainty.clone(), epistemic_status: self.epistemic_status.clone(),
            zero_weight_for_pru: self.zero_weight_for_pru,
        }
    }
    pub fn validate_entry(&self) -> ValidateCallbackResult {
        if self.to_domain().is_valid() { ValidateCallbackResult::Valid }
        else { ValidateCallbackResult::Invalid("RelationshipAssertion violates zero-value/provenance boundary".into()) }
    }
}

// ---------- DISTURBANCE ----------
#[hdk_entry_helper]
#[derive(Clone)]
pub struct DisturbanceEntry {
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
impl DisturbanceEntry {
    fn to_domain(&self) -> Disturbance {
        Disturbance {
            id: self.id.clone(), disturbance_type: self.disturbance_type.clone(),
            start_time: self.start_time, end_time: self.end_time,
            severity_method_id: self.severity_method_id.clone(), severity_value: self.severity_value,
            spatial_scope: self.spatial_scope.clone(), source_refs: self.source_refs.clone(),
            uncertainty: self.uncertainty.clone(),
        }
    }
    pub fn validate_entry(&self) -> ValidateCallbackResult {
        if self.to_domain().is_valid() { ValidateCallbackResult::Valid }
        else { ValidateCallbackResult::Invalid("Disturbance entry is invalid".into()) }
    }
}

// ---------- REVIEW ATTESTATION ----------
#[hdk_entry_helper]
#[derive(Clone)]
pub struct ReviewAttestationEntry {
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
impl ReviewAttestationEntry {
    fn to_domain(&self) -> ReviewAttestation {
        ReviewAttestation {
            id: self.id.clone(), reviewer_id: self.reviewer_id.clone(), scope: self.scope.clone(),
            coi_status: self.coi_status.clone(), subject_refs: self.subject_refs.clone(),
            decision: self.decision.clone(), limitations: self.limitations.clone(),
            independent_for_scope: self.independent_for_scope,
            reviewed_at: self.reviewed_at,
        }
    }
    pub fn validate_entry(&self) -> ValidateCallbackResult {
        if self.to_domain().is_valid() { ValidateCallbackResult::Valid }
        else { ValidateCallbackResult::Invalid("ReviewAttestation violates COI/scope invariants".into()) }
    }
}

// ---------- EVIDENCE PACKAGE ----------
#[hdk_entry_helper]
#[derive(Clone)]
pub struct EvidencePackageEntry {
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
impl EvidencePackageEntry {
    fn to_domain(&self) -> EvidencePackage {
        EvidencePackage {
            id: self.id.clone(),
            subject_id: self.subject_id.clone(),
            claim_uids: self.claim_uids.clone(),
            place_context_version: self.place_context_version.clone(),
            system_boundary_version: self.system_boundary_version.clone(),
            observation_refs: self.observation_refs.clone(),
            method_refs: self.method_refs.clone(),
            raw_data_hashes: self.raw_data_hashes.clone(),
            transformed_data_hashes: self.transformed_data_hashes.clone(),
            package_hash: self.package_hash.clone(),
            claims_registry_hash: self.claims_registry_hash.clone(),
            missing_data_statement: self.missing_data_statement.clone(),
            adverse_event_statement: self.adverse_event_statement.clone(),
            created_at: self.created_at,
        }
    }
    pub fn validate_entry(&self) -> ValidateCallbackResult {
        if self.to_domain().is_valid() { ValidateCallbackResult::Valid }
        else { ValidateCallbackResult::Invalid("EvidencePackage violates provenance/context invariants".into()) }
    }
}

// ---------- ADMISSIBILITY DECISION ----------
#[hdk_entry_helper]
#[derive(Clone)]
pub struct AdmissibilityDecisionEntry {
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
impl AdmissibilityDecisionEntry {
    fn to_domain(&self) -> AdmissibilityDecision {
        AdmissibilityDecision {
            id: self.id.clone(),
            subject_id: self.subject_id.clone(),
            claim_uid: self.claim_uid.clone(),
            evidence_package_refs: self.evidence_package_refs.clone(),
            review_attestation_refs: self.review_attestation_refs.clone(),
            legal_gate: self.legal_gate,
            mrv_gate: self.mrv_gate,
            confidence: self.confidence,
            decision: self.decision.clone(),
            blockers: self.blockers.clone(),
            decided_at: self.decided_at,
            authority_boundary: self.authority_boundary.clone(),
        }
    }
    pub fn validate_entry(&self) -> ValidateCallbackResult {
        if self.to_domain().is_valid() { ValidateCallbackResult::Valid }
        else { ValidateCallbackResult::Invalid("AdmissibilityDecision violates gate/authority invariants".into()) }
    }
}

// ---------- SEMANTIC CLAIM ID ----------
#[hdk_entry_helper]
#[derive(Clone)]
pub struct SemanticClaimRefEntry {
    pub claim_uid: String,
    pub legacy_claim_id: Option<String>,
    pub lineage: String,
}
impl SemanticClaimRefEntry {
    pub fn validate_entry(&self) -> ValidateCallbackResult {
        if valid_semantic_claim_uid(&self.claim_uid) && !self.lineage.is_empty() {
            ValidateCallbackResult::Valid
        } else {
            ValidateCallbackResult::Invalid("semantic claim identity is invalid or uses a legacy global ID".into())
        }
    }
}

#[hdk_entry_types]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    #[entry_type(visibility = "public")] Ohe(OheEntry),
    #[entry_type(visibility = "public")] Evidence(MrvEvidenceEntry),
    #[entry_type(visibility = "public")] Claim(ClaimEntry),
    #[entry_type(visibility = "public")] PlaceContext(PlaceContextEntry),
    #[entry_type(visibility = "public")] SystemBoundary(SystemBoundaryEntry),
    #[entry_type(visibility = "public")] Observation(ObservationEntry),
    #[entry_type(visibility = "public")] RelationshipAssertion(RelationshipAssertionEntry),
    #[entry_type(visibility = "public")] Disturbance(DisturbanceEntry),
    #[entry_type(visibility = "public")] ReviewAttestation(ReviewAttestationEntry),
    #[entry_type(visibility = "public")] EvidencePackage(EvidencePackageEntry),
    #[entry_type(visibility = "public")] AdmissibilityDecision(AdmissibilityDecisionEntry),
    #[entry_type(visibility = "public")] SemanticClaimRef(SemanticClaimRefEntry),
}

#[hdk_link_types]
pub enum LinkTypes {
    SubjectToEvidence,
    EvidenceIdentity,
    SubjectToEvidencePackage,
    EvidencePackageIdentity,
    EvidencePackageToReview,
    ClaimToAdmissibilityDecision,
}

#[hdk_extern]
pub fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    if let FlatOp::StoreEntry(OpEntry::CreateEntry { app_entry, .. }) =
        op.flattened::<EntryTypes, LinkTypes>()?
    {
        let res = match app_entry {
            EntryTypes::Ohe(e) => e.validate_entry(),
            EntryTypes::Evidence(e) => e.validate_entry(),
            EntryTypes::Claim(e) => e.validate_entry(),
            EntryTypes::PlaceContext(e) => e.validate_entry(),
            EntryTypes::SystemBoundary(e) => e.validate_entry(),
            EntryTypes::Observation(e) => e.validate_entry(),
            EntryTypes::RelationshipAssertion(e) => e.validate_entry(),
            EntryTypes::Disturbance(e) => e.validate_entry(),
            EntryTypes::ReviewAttestation(e) => e.validate_entry(),
            EntryTypes::EvidencePackage(e) => e.validate_entry(),
            EntryTypes::AdmissibilityDecision(e) => e.validate_entry(),
            EntryTypes::SemanticClaimRef(e) => e.validate_entry(),
        };
        return Ok(res);
    }
    Ok(ValidateCallbackResult::Valid)
}
