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
use crate::real_data_control::{
    dna_properties, validate_authorization, validate_calibration, validate_review,
    CalibrationApprovalEntry, RealDataAuthorizationEntry, ReviewAttestationEntry,
};
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
    pub sensor_id: String,
    pub indicator: String,
    /// "TEST" | "REAL". TEST and REAL evidence are never silently interchangeable.
    pub evidence_class: String,
    /// Frozen sensor calibration artifact hash for REAL evidence; optional for TEST.
    pub calibration_hash: Option<String>,
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
        if self.sensor_id.is_empty() {
            return ValidateCallbackResult::Invalid("MRV evidence requires sensor_id".into());
        }
        match self.evidence_class.as_str() {
            "TEST" => {
                if !self.subject_id.starts_with("TEST-") {
                    return ValidateCallbackResult::Invalid(
                        "TEST evidence must remain TEST-namespaced".into(),
                    );
                }
            }
            "REAL" => {
                if self.subject_id.starts_with("TEST-") {
                    return ValidateCallbackResult::Invalid(
                        "REAL evidence cannot use a TEST subject namespace".into(),
                    );
                }
                let hash = self.calibration_hash.as_deref().unwrap_or("");
                if hash.len() != 64 || !hash.bytes().all(|byte| byte.is_ascii_hexdigit()) {
                    return ValidateCallbackResult::Invalid(
                        "REAL evidence requires a 64-hex calibration_hash".into(),
                    );
                }
                // This rule belongs to the DNA, not a replaceable coordinator.
                // A correctly shaped hash is not proof of registry membership.
                return ValidateCallbackResult::Invalid(
                    "REAL_DATA_PERSISTENCE_GATE_CLOSED: approved persistent authorization/calibration workflow required".into(),
                );
            }
            other => {
                return ValidateCallbackResult::Invalid(format!("unknown evidence_class: {other}"))
            }
        }
        if self.reviewer.is_some() {
            return ValidateCallbackResult::Invalid(
                "capture evidence cannot self-declare reviewer; use separate attestation workflow"
                    .into(),
            );
        }
        if self.to_domain().is_valid() {
            ValidateCallbackResult::Valid
        } else {
            ValidateCallbackResult::Invalid("MRV evidence entry violates canon invariants (missing provenance, bad confidence, etc.)".into())
        }
    }
}

#[cfg(test)]
mod persistence_gate_tests {
    use super::*;

    fn test_evidence() -> MrvEvidenceEntry {
        MrvEvidenceEntry {
            id: "test:ev-1".into(),
            subject_id: "TEST-ohe-1".into(),
            sensor_id: "sensor-a".into(),
            indicator: "soil_moisture".into(),
            evidence_class: "TEST".into(),
            calibration_hash: None,
            method_hash: "test-method".into(),
            data_hash: "test-data".into(),
            observed_at: 1_900_000_000,
            confidence: 0.7,
            missing_data: false,
            reviewer: None,
        }
    }

    fn rejection(entry: &MrvEvidenceEntry) -> String {
        match entry.validate_entry() {
            ValidateCallbackResult::Invalid(reason) => reason,
            other => panic!("expected rejection, got {other:?}"),
        }
    }

    #[test]
    fn namespaced_test_evidence_is_valid() {
        assert!(matches!(
            test_evidence().validate_entry(),
            ValidateCallbackResult::Valid
        ));
    }

    #[test]
    fn real_evidence_with_well_formed_hash_still_fails_closed() {
        let mut entry = test_evidence();
        entry.evidence_class = "REAL".into();
        entry.subject_id = "ohe-1".into();
        entry.calibration_hash = Some("a".repeat(64));
        assert!(rejection(&entry).contains("REAL_DATA_PERSISTENCE_GATE_CLOSED"));
    }

    #[test]
    fn real_calibration_hash_requires_exact_hex_format() {
        for hash in [
            None,
            Some(String::new()),
            Some("a".repeat(63)),
            Some("g".repeat(64)),
        ] {
            let mut entry = test_evidence();
            entry.evidence_class = "REAL".into();
            entry.subject_id = "ohe-1".into();
            entry.calibration_hash = hash;
            assert!(rejection(&entry).contains("64-hex calibration_hash"));
        }
    }

    #[test]
    fn capture_reviewer_and_missing_sensor_are_rejected() {
        let mut entry = test_evidence();
        entry.reviewer = Some("self".into());
        assert!(rejection(&entry).contains("cannot self-declare reviewer"));
        entry.reviewer = None;
        entry.sensor_id.clear();
        assert!(rejection(&entry).contains("sensor_id"));
    }

    #[test]
    fn evidence_classes_and_namespaces_cannot_be_interchanged() {
        let mut entry = test_evidence();
        entry.subject_id = "ohe-1".into();
        assert!(rejection(&entry).contains("TEST-namespaced"));
        entry.evidence_class = "UNKNOWN".into();
        assert!(rejection(&entry).contains("unknown evidence_class"));
        entry.evidence_class = "REAL".into();
        entry.subject_id = "TEST-ohe-1".into();
        assert!(rejection(&entry).contains("REAL evidence cannot use a TEST"));
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
    RealDataAuthorization(RealDataAuthorizationEntry),
    #[entry_type(visibility = "public")]
    CalibrationApproval(CalibrationApprovalEntry),
    #[entry_type(visibility = "public")]
    ReviewAttestation(ReviewAttestationEntry),
}

// Link types. SubjectToEvidence supports subject queries. EvidenceIdentity links a
// deterministic event-identity anchor to the first committed evidence action, which
// gives a single gateway/agent durable idempotency boundary across process restarts.
// It is not a universal cross-agent duplicate-proof claim.
#[hdk_link_types]
pub enum LinkTypes {
    SubjectToEvidence,
    EvidenceIdentity,
}

#[hdk_extern]
pub fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    match op.flattened::<EntryTypes, LinkTypes>()? {
        FlatOp::StoreEntry(OpEntry::CreateEntry { app_entry, action }) => {
            let author = action.author;
            let props = dna_properties()?;
            match app_entry {
                EntryTypes::Ohe(e) => Ok(e.validate_entry()),
                EntryTypes::Evidence(e) => Ok(e.validate_entry()),
                EntryTypes::Claim(e) => Ok(e.validate_entry()),
                EntryTypes::RealDataAuthorization(e) => Ok(validate_authorization(&e, &author, &props)),
                EntryTypes::CalibrationApproval(e) => Ok(validate_calibration(&e, &author, &props)),
                EntryTypes::ReviewAttestation(e) => validate_review(&e, &author, &props),
            }
        }
        FlatOp::StoreEntry(OpEntry::UpdateEntry { app_entry, action, .. }) => {
            let author = action.author;
            let props = dna_properties()?;
            match app_entry {
                EntryTypes::Ohe(e) => Ok(e.validate_entry()),
                EntryTypes::Evidence(e) => Ok(e.validate_entry()),
                EntryTypes::Claim(e) => Ok(e.validate_entry()),
                EntryTypes::RealDataAuthorization(e) => Ok(validate_authorization(&e, &author, &props)),
                EntryTypes::CalibrationApproval(e) => Ok(validate_calibration(&e, &author, &props)),
                EntryTypes::ReviewAttestation(e) => validate_review(&e, &author, &props),
            }
        }
        _ => Ok(ValidateCallbackResult::Valid),
    }
}
