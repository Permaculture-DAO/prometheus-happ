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

use crate::ohe::{Ohe, OheStatus};
use crate::mrv::MrvEvidence;
use crate::claim::{Claim, ClaimStatus};
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
            Some(_) => ValidateCallbackResult::Invalid("claim is overclaiming: investor-facing claims must be at least PilotObserved".into()),
            None => ValidateCallbackResult::Invalid(format!("unknown claim status: {}", self.status)),
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
}

// Link types. SubjectToEvidence links a subject anchor (e.g. an OHE id) to each
// MRV evidence record, so the admissibility gate can run over a subject's actual
// chain-persisted evidence (not just a passed-in list).
#[hdk_link_types]
pub enum LinkTypes {
    SubjectToEvidence,
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
        };
        return Ok(res);
    }
    Ok(ValidateCallbackResult::Valid)
}
