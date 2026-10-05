// entry_types.rs
// Real Holochain entry types for the integrity zome, with a validation callback
// that enforces the canon-aligned domain invariants (ohe.rs / mrv.rs / claim.rs).
//
// Entry structs are flat (primitive fields) so they need no serde on the domain
// enums; each carries a `validate_entry()` mirroring the domain rules, and the
// `validate(op)` callback runs it on create/update. No PRU value anywhere.

use hdk::prelude::*;
// Validation Op flattening types live in hdi.
use hdi::prelude::{FlatOp, OpEntry};

use crate::claim::{Claim, ClaimStatus};
use crate::mrv::MrvEvidence;
use crate::ohe::{Ohe, OheStatus};
use crate::valueflows::ids::AgentId;

// ---------- OHE ----------
#[hdk_entry_helper]
#[derive(Clone)]
pub struct OheEntry {
    pub id: String,
    pub steward: String,
    pub site: String,
    /// "Proposed" | "BaselineRecorded" | "Active" | "Terminated"
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
            Some(_) => ValidateCallbackResult::Invalid("OHE entry violates canon invariants (e.g. Active without baseline, missing steward/site)".into()),
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
            // source not persisted at entry level in this increment; default for validation
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
            ValidateCallbackResult::Invalid("MRV evidence entry violates canon invariants (missing provenance, bad confidence, etc.)".into())
        }
    }
}

// ---------- CLAIM ----------
#[hdk_entry_helper]
#[derive(Clone)]
pub struct ClaimEntry {
    pub id: String,
    pub statement: String,
    /// "Architectural" | "Methodological" | "Hypothesis" | "PilotObserved"
    /// | "ThirdPartyReviewed" | "LegallyAdmitted" | "MarketAdmitted"
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
                "claim is overclaiming: investor-facing claims must be at least PilotObserved"
                    .into(),
            ),
            None => {
                ValidateCallbackResult::Invalid(format!("unknown claim status: {}", self.status))
            }
        }
    }
}

// ---------- RAVEL SHADOW RISK RECORDS ----------
// RAVEL records provenance-bound diagnostic outputs only. They never certify,
// price insurance, approve credit, or create PRU/RAP/capital consequences.

fn finite_nonnegative(value: f64) -> bool {
    value.is_finite() && value >= 0.0
}

fn probability_like(value: f64) -> bool {
    value.is_finite() && (0.0..=1.0).contains(&value)
}

#[hdk_entry_helper]
#[derive(Clone)]
pub struct RavelAssessmentEntry {
    pub id: String,
    pub subject_id: String,
    pub model_version: String,
    pub evidence_refs: Vec<String>,
    pub expected_loss: f64,
    pub es95: f64,
    pub es99: f64,
    pub ppci: Option<f64>,
    /// Regenerative Risk Delta. May be negative when the candidate scenario is worse.
    pub rr_delta: Option<f64>,
    pub urbc: Option<f64>,
    pub confidence: f64,
    /// Candidate v0.1 invariant: always 0.0 for capital-facing purposes.
    pub vrrc: f64,
    /// Must be "not_admitted" in candidate v0.1.
    pub vrrc_status: String,
    /// Must be "shadow_underwriting".
    pub mode: String,
    /// Must be "evaluation_not_certification".
    pub authority_boundary: String,
    pub created_at: i64,
}

impl RavelAssessmentEntry {
    pub fn validate_entry(&self) -> ValidateCallbackResult {
        let rr_ok = self
            .rr_delta
            .map(|v| v.is_finite() && v <= 1.0)
            .unwrap_or(true);
        let ppci_ok = self.ppci.map(probability_like).unwrap_or(true);
        let urbc_ok = self.urbc.map(probability_like).unwrap_or(true);
        let ok = !self.id.is_empty()
            && !self.subject_id.is_empty()
            && !self.model_version.is_empty()
            && finite_nonnegative(self.expected_loss)
            && finite_nonnegative(self.es95)
            && finite_nonnegative(self.es99)
            && self.es95 + 1e-9 >= self.expected_loss
            && self.es99 + 1e-9 >= self.es95
            && ppci_ok
            && rr_ok
            && urbc_ok
            && probability_like(self.confidence)
            && self.vrrc == 0.0
            && self.vrrc_status == "not_admitted"
            && self.mode == "shadow_underwriting"
            && self.authority_boundary == "evaluation_not_certification";
        if ok {
            ValidateCallbackResult::Valid
        } else {
            ValidateCallbackResult::Invalid(
                "RAVEL assessment violates shadow-underwriting/non-authority invariants".into(),
            )
        }
    }
}

#[hdk_entry_helper]
#[derive(Clone)]
pub struct UltimateRiskBearerEntry {
    pub assessment_id: String,
    pub scenario_id: String,
    pub bearer_id: String,
    pub economic_group_id: String,
    pub retained_loss: f64,
    pub model_version: String,
}

impl UltimateRiskBearerEntry {
    pub fn validate_entry(&self) -> ValidateCallbackResult {
        if !self.assessment_id.is_empty()
            && !self.scenario_id.is_empty()
            && !self.bearer_id.is_empty()
            && !self.economic_group_id.is_empty()
            && !self.model_version.is_empty()
            && finite_nonnegative(self.retained_loss)
        {
            ValidateCallbackResult::Valid
        } else {
            ValidateCallbackResult::Invalid("invalid ultimate-risk-bearer record".into())
        }
    }
}

#[hdk_entry_helper]
#[derive(Clone)]
pub struct RavelBrakeSignalEntry {
    pub assessment_id: String,
    /// "Green" | "Yellow" | "Orange" | "Red"
    pub severity: String,
    pub reasons: Vec<String>,
    pub review_required: bool,
    /// Must remain false: RAVEL never autonomously enforces financial consequences.
    pub autonomous_enforcement: bool,
    pub created_at: i64,
}

impl RavelBrakeSignalEntry {
    pub fn validate_entry(&self) -> ValidateCallbackResult {
        let severity_ok = matches!(
            self.severity.as_str(),
            "Green" | "Yellow" | "Orange" | "Red"
        );
        let review_ok = match self.severity.as_str() {
            "Orange" | "Red" => self.review_required,
            _ => true,
        };
        if !self.assessment_id.is_empty()
            && severity_ok
            && review_ok
            && !self.autonomous_enforcement
        {
            ValidateCallbackResult::Valid
        } else {
            ValidateCallbackResult::Invalid(
                "RAVEL brake signal violates review/non-enforcement boundary".into(),
            )
        }
    }
}

// ---------- ENTRY TYPES + VALIDATION ----------
#[hdk_entry_types]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    #[entry_type(visibility = "public")]
    Ohe(OheEntry),
    #[entry_type(visibility = "public")]
    Evidence(MrvEvidenceEntry),
    #[entry_type(visibility = "public")]
    Claim(ClaimEntry),
    #[entry_type(visibility = "public")]
    RavelAssessment(RavelAssessmentEntry),
    #[entry_type(visibility = "public")]
    UltimateRiskBearer(UltimateRiskBearerEntry),
    #[entry_type(visibility = "public")]
    RavelBrakeSignal(RavelBrakeSignalEntry),
}

// Link types. SubjectToEvidence supports subject queries. EvidenceIdentity links a
// deterministic event-identity anchor to the first committed evidence action, which
// gives a single gateway/agent durable idempotency boundary across process restarts.
// It is not a universal cross-agent duplicate-proof claim.
#[hdk_link_types]
pub enum LinkTypes {
    SubjectToEvidence,
    EvidenceIdentity,
    SubjectToRavelAssessment,
    AssessmentToUltimateRiskBearer,
    AssessmentToBrakeSignal,
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
            EntryTypes::RavelAssessment(e) => e.validate_entry(),
            EntryTypes::UltimateRiskBearer(e) => e.validate_entry(),
            EntryTypes::RavelBrakeSignal(e) => e.validate_entry(),
        };
        return Ok(res);
    }
    Ok(ValidateCallbackResult::Valid)
}
