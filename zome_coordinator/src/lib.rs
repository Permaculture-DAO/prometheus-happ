use hdk::prelude::*;
use zome_integrity::admissibility::admissible;
use zome_integrity::entry_types::{ClaimEntry, EntryTypes, LinkTypes, MrvEvidenceEntry, OheEntry};
use zome_integrity::real_data_control::{
    dna_properties, CalibrationApprovalEntry, RealDataAuthorizationEntry, ReviewAttestationEntry,
};

/// Deterministic anchor for a subject (e.g. an OHE id) to hang evidence links on.
/// Both writer and reader derive the same base from the subject id.
fn subject_base(subject_id: &str) -> ExternResult<AnyLinkableHash> {
    Ok(Path::from(format!("subject:{subject_id}"))
        .path_entry_hash()?
        .into())
}

/// Stable single-agent idempotency key: subject + sensor + indicator + observation time.
fn calibration_sensor_base(sensor_id: &str) -> ExternResult<AnyLinkableHash> {
    Ok(Path::from(format!("calibration_sensor:{sensor_id}"))
        .path_entry_hash()?
        .into())
}

fn evidence_identity_base(evidence: &MrvEvidenceEntry) -> ExternResult<AnyLinkableHash> {
    Ok(Path::from(format!(
        "evidence_identity:{}:{}:{}:{}",
        evidence.subject_id, evidence.sensor_id, evidence.indicator, evidence.observed_at
    ))
    .path_entry_hash()?
    .into())
}

#[hdk_extern]
fn init() -> ExternResult<InitCallbackResult> {
    Ok(InitCallbackResult::Pass)
}

#[hdk_extern]
fn hello_benchmark_layer() -> ExternResult<String> {
    Ok("Prometheus Benchmark Intelligence Layer online: evaluation_not_certification".to_string())
}

// ---------------------------------------------------------------------------
// CRUD over the canon-aligned entries. Integrity validation (ohe/mrv/claim
// invariants + binary admissibility) runs in the integrity zome's validate
// callback, so create_* fails for entries that violate the canon. No PRU value
// is created or returned anywhere here.
// ---------------------------------------------------------------------------

#[hdk_extern]
pub fn create_ohe(ohe: OheEntry) -> ExternResult<ActionHash> {
    create_entry(EntryTypes::Ohe(ohe))
}

#[hdk_extern]
pub fn create_evidence(evidence: MrvEvidenceEntry) -> ExternResult<ActionHash> {
    Ok(create_evidence_idempotent(evidence)?.action_hash)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateEvidenceResult {
    pub action_hash: ActionHash,
    /// False means an existing action was returned for the same identity tuple.
    pub created: bool,
}

/// Durable idempotent create for the local gateway/agent.
///
/// Sequential re-delivery of the same subject+sensor+indicator+observed_at returns
/// the existing action. Concurrent or multi-agent races remain outside this bounded
/// guarantee and must not be described as universally duplicate-proof.
///
/// REAL evidence is deliberately rejected here until a separately reviewed,
/// persistent authorization + calibration registry is implemented. The integrity
/// zome independently rejects REAL entries, including writes from other coordinators.
#[hdk_extern]
pub fn create_evidence_idempotent(
    evidence: MrvEvidenceEntry,
) -> ExternResult<CreateEvidenceResult> {
    if evidence.evidence_class == "REAL" {
        return Err(wasm_error!(WasmErrorInner::Guest(
            "REAL_DATA_PERSISTENCE_GATE_CLOSED: real evidence requires the approved persistent authorization/calibration workflow".into()
        )));
    }
    let identity_base = evidence_identity_base(&evidence)?;
    let existing = get_links(
        LinkQuery::new(
            identity_base.clone(),
            LinkTypes::EvidenceIdentity.try_into_filter()?,
        ),
        GetStrategy::default(),
    )?;
    if let Some(action_hash) = existing
        .into_iter()
        .find_map(|link| link.target.into_action_hash())
    {
        return Ok(CreateEvidenceResult {
            action_hash,
            created: false,
        });
    }

    let subject = evidence.subject_id.clone();
    let action_hash = create_entry(EntryTypes::Evidence(evidence))?;
    create_link(
        subject_base(&subject)?,
        action_hash.clone(),
        LinkTypes::SubjectToEvidence,
        (),
    )?;
    create_link(
        identity_base,
        action_hash.clone(),
        LinkTypes::EvidenceIdentity,
        (),
    )?;
    Ok(CreateEvidenceResult {
        action_hash,
        created: true,
    })
}

#[hdk_extern]
pub fn create_claim(claim: ClaimEntry) -> ExternResult<ActionHash> {
    create_entry(EntryTypes::Claim(claim))
}

#[hdk_extern]
pub fn create_real_data_authorization(
    authorization: RealDataAuthorizationEntry,
) -> ExternResult<ActionHash> {
    create_entry(EntryTypes::RealDataAuthorization(authorization))
}

#[hdk_extern]
pub fn create_calibration_approval(
    calibration: CalibrationApprovalEntry,
) -> ExternResult<ActionHash> {
    let sensor_id = calibration.sensor_id.clone();
    let previous = calibration.previous_calibration.clone();
    let action_hash = create_entry(EntryTypes::CalibrationApproval(calibration))?;
    create_link(
        calibration_sensor_base(&sensor_id)?,
        action_hash.clone(),
        LinkTypes::SensorToCalibration,
        (),
    )?;
    if let Some(previous_action) = previous {
        create_link(
            previous_action,
            action_hash.clone(),
            LinkTypes::CalibrationSuccessor,
            (),
        )?;
    }
    Ok(action_hash)
}

#[hdk_extern]
pub fn create_review_attestation(
    attestation: ReviewAttestationEntry,
) -> ExternResult<ActionHash> {
    let evidence_action = attestation.evidence_action.clone();
    let action_hash = create_entry(EntryTypes::ReviewAttestation(attestation))?;
    create_link(
        evidence_action,
        action_hash.clone(),
        LinkTypes::EvidenceToReview,
        (),
    )?;
    Ok(action_hash)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RealDataGateStatus {
    pub real_data_enabled: bool,
    pub authority_configured: bool,
    pub approved_reviewer_count: usize,
    pub authority_boundary: String,
}

#[hdk_extern]
pub fn get_real_data_gate_status(_: ()) -> ExternResult<RealDataGateStatus> {
    let props = dna_properties()?;
    Ok(RealDataGateStatus {
        real_data_enabled: props.real_data_enabled,
        authority_configured: props.real_data_authority.is_some(),
        approved_reviewer_count: props.approved_reviewers.len(),
        authority_boundary: "persistence_control_plane_only_no_scientific_admission".into(),
    })
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CalibrationResolveInput {
    pub sensor_id: String,
    pub observed_at: i64,
    pub calibration_hash: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CalibrationResolveResult {
    pub status: String,
    pub action_hash: Option<ActionHash>,
    pub calibration_hash: Option<String>,
    pub reason: String,
}

#[hdk_extern]
pub fn resolve_calibration(
    input: CalibrationResolveInput,
) -> ExternResult<CalibrationResolveResult> {
    let links = get_links(
        LinkQuery::new(
            calibration_sensor_base(&input.sensor_id)?,
            LinkTypes::SensorToCalibration.try_into_filter()?,
        ),
        GetStrategy::default(),
    )?;
    let mut records: Vec<(ActionHash, CalibrationApprovalEntry)> = Vec::new();
    for link in links {
        let Some(action_hash) = link.target.into_action_hash() else {
            continue;
        };
        let Some(record) = get(action_hash.clone(), GetOptions::default())? else {
            continue;
        };
        if let Ok(Some(entry)) = record.entry().to_app_option::<CalibrationApprovalEntry>() {
            records.push((action_hash, entry));
        }
    }

    let mut active = Vec::new();
    for (action_hash, entry) in records.iter() {
        if entry.status != "APPROVED"
            || input.observed_at < entry.valid_from
            || input.observed_at > entry.valid_until
        {
            continue;
        }
        if let Some(expected_hash) = input.calibration_hash.as_ref() {
            if &entry.calibration_hash != expected_hash {
                continue;
            }
        }
        let terminated = records.iter().any(|(_, successor)| {
            successor.previous_calibration.as_ref() == Some(action_hash)
                && successor.valid_from <= input.observed_at
                && (successor.status == "APPROVED" || successor.status == "REVOKED")
        });
        if !terminated {
            active.push((action_hash.clone(), entry.clone()));
        }
    }

    match active.as_slice() {
        [] => Ok(CalibrationResolveResult {
            status: "NO_ACTIVE".into(),
            action_hash: None,
            calibration_hash: None,
            reason: "no approved non-terminated calibration matches sensor/time/hash".into(),
        }),
        [(action_hash, entry)] => Ok(CalibrationResolveResult {
            status: "ACTIVE".into(),
            action_hash: Some(action_hash.clone()),
            calibration_hash: Some(entry.calibration_hash.clone()),
            reason: "exactly one approved non-terminated calibration resolved".into(),
        }),
        _ => Ok(CalibrationResolveResult {
            status: "AMBIGUOUS".into(),
            action_hash: None,
            calibration_hash: None,
            reason: "multiple active calibration lineages remain; fail closed".into(),
        }),
    }
}

#[hdk_extern]
pub fn get_record(action_hash: ActionHash) -> ExternResult<Option<Record>> {
    get(action_hash, GetOptions::default())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EvidenceRecordView {
    pub action_hash: ActionHash,
    pub evidence: MrvEvidenceEntry,
}

/// Independently query persisted evidence linked to one subject.
#[hdk_extern]
pub fn get_subject_evidence(subject_id: String) -> ExternResult<Vec<EvidenceRecordView>> {
    let links = get_links(
        LinkQuery::new(
            subject_base(&subject_id)?,
            LinkTypes::SubjectToEvidence.try_into_filter()?,
        ),
        GetStrategy::default(),
    )?;
    let mut records = Vec::new();
    for link in links {
        if let Some(action_hash) = link.target.into_action_hash() {
            if let Some(record) = get(action_hash.clone(), GetOptions::default())? {
                if let Ok(Some(evidence)) = record.entry().to_app_option::<MrvEvidenceEntry>() {
                    records.push(EvidenceRecordView {
                        action_hash,
                        evidence,
                    });
                }
            }
        }
    }
    Ok(records)
}

// ---------------------------------------------------------------------------
// The evidence spine: assess ADMISSIBILITY (binary), never value.
// Given a legal-gate decision and a subject's MRV evidence, returns whether the
// evidence may proceed to off-runtime value-analysis. Returns a bool — there is
// no value field, no PRU, no amount. value stays prose-only, suspended at zero.
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssessInput {
    pub legal_gate: bool,
    pub confidence_threshold: f64,
    pub require_reviewer: bool,
    pub evidence: Vec<MrvEvidenceEntry>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssessResult {
    /// Binary admissibility (LegalGate ∧ MRVGate). NOT a value.
    pub admissible: bool,
    pub authority_boundary: String,
}

#[hdk_extern]
pub fn assess_admissibility(input: AssessInput) -> ExternResult<AssessResult> {
    let domain: Vec<_> = input.evidence.iter().map(|e| e.to_domain()).collect();
    let ok = admissible(
        input.legal_gate,
        &domain,
        input.confidence_threshold,
        input.require_reviewer,
    );
    Ok(AssessResult {
        admissible: ok,
        authority_boundary: "admissibility_only_no_value".to_string(),
    })
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubjectAssessInput {
    pub subject_id: String,
    pub legal_gate: bool,
    pub confidence_threshold: f64,
    pub require_reviewer: bool,
}

/// Source-chain/DHT spine: gather a subject's persisted MRV evidence (via links) and
/// run the binary admissibility gate over it. Returns a bool, never a value.
#[hdk_extern]
pub fn assess_subject_admissibility(input: SubjectAssessInput) -> ExternResult<AssessResult> {
    let evidence: Vec<_> = get_subject_evidence(input.subject_id)?
        .into_iter()
        .map(|record| record.evidence.to_domain())
        .collect();
    let ok = admissible(
        input.legal_gate,
        &evidence,
        input.confidence_threshold,
        input.require_reviewer,
    );
    Ok(AssessResult {
        admissible: ok,
        authority_boundary: "admissibility_only_no_value".to_string(),
    })
}
