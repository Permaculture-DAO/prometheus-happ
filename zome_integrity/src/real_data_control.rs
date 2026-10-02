use hdk::prelude::*;
use crate::entry_types::MrvEvidenceEntry;

#[derive(Clone, Debug, Serialize, Deserialize, SerializedBytes, Default)]
pub struct PrometheusDnaProperties {
    pub real_data_enabled: bool,
    pub real_data_authority: Option<AgentPubKey>,
    pub approved_reviewers: Vec<AgentPubKey>,
}

pub fn dna_properties() -> ExternResult<PrometheusDnaProperties> {
    let raw = dna_info()?.modifiers.properties;
    Ok(raw.try_into().unwrap_or_default())
}

fn valid_window(valid_from: i64, valid_until: i64) -> bool {
    valid_from > 0 && valid_until >= valid_from
}

fn valid_sha256_hex(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|b| b.is_ascii_hexdigit())
}

#[hdk_entry_helper]
#[derive(Clone)]
pub struct RealDataAuthorizationEntry {
    pub id: String,
    pub subject_id: String,
    pub sensor_id: String,
    pub indicator: String,
    pub valid_from: i64,
    pub valid_until: i64,
    pub enabled: bool,
}

impl RealDataAuthorizationEntry {
    pub fn validate_shape(&self) -> ValidateCallbackResult {
        if self.id.is_empty() || self.subject_id.is_empty() || self.sensor_id.is_empty() || self.indicator.is_empty() {
            return ValidateCallbackResult::Invalid("real-data authorization requires non-empty id/subject/sensor/indicator".into());
        }
        if !valid_window(self.valid_from, self.valid_until) {
            return ValidateCallbackResult::Invalid("real-data authorization has invalid validity window".into());
        }
        ValidateCallbackResult::Valid
    }
}

#[hdk_entry_helper]
#[derive(Clone)]
pub struct CalibrationApprovalEntry {
    pub id: String,
    pub sensor_id: String,
    pub calibration_hash: String,
    pub valid_from: i64,
    pub valid_until: i64,
    /// APPROVED creates/continues a calibration lineage. REVOKED terminates one.
    pub status: String,
    /// None only for the first approval in a sensor lineage.
    /// A later APPROVED entry supersedes this predecessor from valid_from onward.
    /// A REVOKED entry terminates this predecessor from valid_from onward.
    pub previous_calibration: Option<ActionHash>,
}

impl CalibrationApprovalEntry {
    pub fn validate_shape(&self) -> ValidateCallbackResult {
        if self.id.is_empty() || self.sensor_id.is_empty() {
            return ValidateCallbackResult::Invalid("calibration approval requires id and sensor_id".into());
        }
        if !valid_sha256_hex(&self.calibration_hash) {
            return ValidateCallbackResult::Invalid("calibration approval requires a 64-hex SHA-256 hash".into());
        }
        if !valid_window(self.valid_from, self.valid_until) {
            return ValidateCallbackResult::Invalid("calibration approval has invalid validity window".into());
        }
        if self.status != "APPROVED" && self.status != "REVOKED" {
            return ValidateCallbackResult::Invalid("calibration status must be APPROVED or REVOKED".into());
        }
        if self.status == "REVOKED" && self.previous_calibration.is_none() {
            return ValidateCallbackResult::Invalid("calibration revocation must reference the approved calibration being revoked".into());
        }
        if self.status == "REVOKED" && self.valid_from != self.valid_until {
            return ValidateCallbackResult::Invalid("calibration revocation is an effective-time event and requires valid_from == valid_until".into());
        }
        ValidateCallbackResult::Valid
    }
}

#[hdk_entry_helper]
#[derive(Clone)]
pub struct ReviewAttestationEntry {
    pub id: String,
    pub evidence_action: ActionHash,
    pub reviewer: AgentPubKey,
    pub decision: String,
    pub reviewed_at: i64,
    pub lineage_hash: String,
}

impl ReviewAttestationEntry {
    pub fn validate_shape(&self) -> ValidateCallbackResult {
        if self.id.is_empty() || self.reviewed_at <= 0 {
            return ValidateCallbackResult::Invalid("review attestation requires id and positive reviewed_at".into());
        }
        if self.decision != "ACCEPT" && self.decision != "REJECT" {
            return ValidateCallbackResult::Invalid("review decision must be ACCEPT or REJECT".into());
        }
        if !valid_sha256_hex(&self.lineage_hash) {
            return ValidateCallbackResult::Invalid("review attestation requires a 64-hex lineage_hash".into());
        }
        ValidateCallbackResult::Valid
    }
}

fn authority_result(author: &AgentPubKey, props: &PrometheusDnaProperties) -> ValidateCallbackResult {
    match &props.real_data_authority {
        Some(authority) if authority == author => ValidateCallbackResult::Valid,
        Some(_) => ValidateCallbackResult::Invalid("control-plane entry author is not the configured real-data authority".into()),
        None => ValidateCallbackResult::Invalid("REAL_DATA_CONTROL_PLANE_CLOSED: no real-data authority configured in DNA properties".into()),
    }
}

pub fn validate_authorization(
    entry: &RealDataAuthorizationEntry,
    author: &AgentPubKey,
    props: &PrometheusDnaProperties,
) -> ValidateCallbackResult {
    match entry.validate_shape() {
        ValidateCallbackResult::Valid => authority_result(author, props),
        other => other,
    }
}

pub fn validate_calibration(
    entry: &CalibrationApprovalEntry,
    author: &AgentPubKey,
    props: &PrometheusDnaProperties,
) -> ExternResult<ValidateCallbackResult> {
    if let other @ ValidateCallbackResult::Invalid(_) = entry.validate_shape() {
        return Ok(other);
    }
    if let other @ ValidateCallbackResult::Invalid(_) = authority_result(author, props) {
        return Ok(other);
    }
    let Some(previous_action) = entry.previous_calibration.clone() else {
        if entry.status == "REVOKED" {
            return Ok(ValidateCallbackResult::Invalid(
                "calibration revocation cannot be a lineage root".into(),
            ));
        }
        return Ok(ValidateCallbackResult::Valid);
    };

    let record = must_get_valid_record(previous_action)?;
    let previous: Option<CalibrationApprovalEntry> = record
        .entry()
        .to_app_option()
        .map_err(|err| wasm_error!(WasmErrorInner::Guest(format!(
            "previous calibration decode failed: {err}"
        ))))?;
    let Some(previous) = previous else {
        return Ok(ValidateCallbackResult::Invalid(
            "previous_calibration must target a calibration entry".into(),
        ));
    };
    if previous.status != "APPROVED" {
        return Ok(ValidateCallbackResult::Invalid(
            "calibration lineage may only advance from an APPROVED predecessor".into(),
        ));
    }
    if previous.sensor_id != entry.sensor_id {
        return Ok(ValidateCallbackResult::Invalid(
            "calibration lineage cannot cross sensor_id".into(),
        ));
    }
    if entry.valid_from <= previous.valid_from {
        return Ok(ValidateCallbackResult::Invalid(
            "calibration successor/revocation must become effective after predecessor valid_from".into(),
        ));
    }
    match entry.status.as_str() {
        "APPROVED" if previous.calibration_hash == entry.calibration_hash => Ok(
            ValidateCallbackResult::Invalid(
                "superseding calibration must use a different calibration_hash".into(),
            ),
        ),
        "REVOKED" if previous.calibration_hash != entry.calibration_hash => Ok(
            ValidateCallbackResult::Invalid(
                "revocation must carry the same calibration_hash as the calibration being revoked".into(),
            ),
        ),
        _ => Ok(ValidateCallbackResult::Valid),
    }
}

pub fn validate_review(
    entry: &ReviewAttestationEntry,
    author: &AgentPubKey,
    props: &PrometheusDnaProperties,
) -> ExternResult<ValidateCallbackResult> {
    if let other @ ValidateCallbackResult::Invalid(_) = entry.validate_shape() {
        return Ok(other);
    }
    if &entry.reviewer != author {
        return Ok(ValidateCallbackResult::Invalid("reviewer identity must equal the action author".into()));
    }
    if !props.approved_reviewers.iter().any(|reviewer| reviewer == author) {
        return Ok(ValidateCallbackResult::Invalid("review author is not listed in DNA approved_reviewers".into()));
    }
    let record = must_get_valid_record(entry.evidence_action.clone())?;
    let evidence: Option<MrvEvidenceEntry> = record
        .entry()
        .to_app_option()
        .map_err(|err| wasm_error!(WasmErrorInner::Guest(format!(
            "review target decode failed: {err}"
        ))))?;
    if evidence.is_none() {
        return Ok(ValidateCallbackResult::Invalid("review attestation must target an MRV evidence entry".into()));
    }
    if record.action().author() == author {
        return Ok(ValidateCallbackResult::Invalid("reviewer must be independent from the evidence author".into()));
    }
    Ok(ValidateCallbackResult::Valid)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn agent(byte: u8) -> AgentPubKey {
        AgentPubKey::from_raw_36(vec![byte; 36])
    }

    #[test]
    fn authority_is_fail_closed_when_unconfigured() {
        let props = PrometheusDnaProperties::default();
        let entry = RealDataAuthorizationEntry {
            id: "auth-1".into(), subject_id: "ohe-1".into(), sensor_id: "s-1".into(),
            indicator: "soil_moisture".into(), valid_from: 1, valid_until: 2, enabled: true,
        };
        assert!(matches!(validate_authorization(&entry, &agent(1), &props), ValidateCallbackResult::Invalid(_)));
    }

    #[test]
    fn only_configured_authority_can_write_control_entries() {
        let authority = agent(1);
        let other = agent(2);
        let props = PrometheusDnaProperties {
            real_data_enabled: false,
            real_data_authority: Some(authority.clone()),
            approved_reviewers: vec![],
        };
        let auth = RealDataAuthorizationEntry {
            id: "auth-1".into(), subject_id: "ohe-1".into(), sensor_id: "s-1".into(),
            indicator: "soil_moisture".into(), valid_from: 1, valid_until: 2, enabled: true,
        };
        assert!(matches!(validate_authorization(&auth, &authority, &props), ValidateCallbackResult::Valid));
        assert!(matches!(validate_authorization(&auth, &other, &props), ValidateCallbackResult::Invalid(_)));
    }

    #[test]
    fn calibration_shape_is_strict() {
        let entry = CalibrationApprovalEntry {
            id: "cal-1".into(), sensor_id: "s-1".into(), calibration_hash: "a".repeat(64),
            valid_from: 1, valid_until: 2, status: "APPROVED".into(), previous_calibration: None,
        };
        assert!(matches!(entry.validate_shape(), ValidateCallbackResult::Valid));
        let mut bad = entry.clone();
        bad.calibration_hash = "g".repeat(64);
        assert!(matches!(bad.validate_shape(), ValidateCallbackResult::Invalid(_)));
    }

    #[test]
    fn review_shape_is_strict() {
        let entry = ReviewAttestationEntry {
            id: "review-1".into(), evidence_action: ActionHash::from_raw_36(vec![3; 36]),
            reviewer: agent(4), decision: "ACCEPT".into(), reviewed_at: 1, lineage_hash: "b".repeat(64),
        };
        assert!(matches!(entry.validate_shape(), ValidateCallbackResult::Valid));
    }
}
