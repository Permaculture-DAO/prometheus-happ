use hdk::prelude::*;
use zome_integrity::admissibility::admissible;
use zome_integrity::entry_types::{
    ClaimEntry, EntryTypes, LinkTypes, MrvEvidenceEntry, OheEntry,
    RavelAssessmentEntry, RavelBrakeSignalEntry, UltimateRiskBearerEntry,
};

/// Deterministic anchor for a subject (e.g. an OHE id) to hang evidence links on.
/// Both writer and reader derive the same base from the subject id.
fn subject_base(subject_id: &str) -> ExternResult<AnyLinkableHash> {
    Ok(Path::from(format!("subject:{subject_id}"))
        .path_entry_hash()?
        .into())
}

fn ravel_assessment_base(assessment_id: &str) -> ExternResult<AnyLinkableHash> {
    Ok(Path::from(format!("ravel_assessment:{assessment_id}"))
        .path_entry_hash()?
        .into())
}

/// Stable single-agent idempotency key: subject + sensor + indicator + observation time.
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


/// Persist a shadow-underwriting assessment after integrity validation.
/// This records a diagnostic result; it does not approve underwriting or capital use.
#[hdk_extern]
pub fn create_ravel_assessment(assessment: RavelAssessmentEntry) -> ExternResult<ActionHash> {
    let subject = assessment.subject_id.clone();
    let id = assessment.id.clone();
    let action_hash = create_entry(EntryTypes::RavelAssessment(assessment))?;
    create_link(
        subject_base(&subject)?,
        action_hash.clone(),
        LinkTypes::SubjectToRavelAssessment,
        (),
    )?;
    // Anchor exists only to group subordinate bearer/brake records.
    let _ = ravel_assessment_base(&id)?;
    Ok(action_hash)
}

#[hdk_extern]
pub fn create_ultimate_risk_bearer(entry: UltimateRiskBearerEntry) -> ExternResult<ActionHash> {
    let assessment_id = entry.assessment_id.clone();
    let action_hash = create_entry(EntryTypes::UltimateRiskBearer(entry))?;
    create_link(
        ravel_assessment_base(&assessment_id)?,
        action_hash.clone(),
        LinkTypes::AssessmentToUltimateRiskBearer,
        (),
    )?;
    Ok(action_hash)
}

#[hdk_extern]
pub fn create_ravel_brake_signal(signal: RavelBrakeSignalEntry) -> ExternResult<ActionHash> {
    let assessment_id = signal.assessment_id.clone();
    let action_hash = create_entry(EntryTypes::RavelBrakeSignal(signal))?;
    create_link(
        ravel_assessment_base(&assessment_id)?,
        action_hash.clone(),
        LinkTypes::AssessmentToBrakeSignal,
        (),
    )?;
    Ok(action_hash)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RavelAssessmentRecordView {
    pub action_hash: ActionHash,
    pub assessment: RavelAssessmentEntry,
}

#[hdk_extern]
pub fn get_subject_ravel_assessments(
    subject_id: String,
) -> ExternResult<Vec<RavelAssessmentRecordView>> {
    let links = get_links(
        LinkQuery::new(
            subject_base(&subject_id)?,
            LinkTypes::SubjectToRavelAssessment.try_into_filter()?,
        ),
        GetStrategy::default(),
    )?;
    let mut records = Vec::new();
    for link in links {
        if let Some(action_hash) = link.target.into_action_hash() {
            if let Some(record) = get(action_hash.clone(), GetOptions::default())? {
                if let Ok(Some(assessment)) =
                    record.entry().to_app_option::<RavelAssessmentEntry>()
                {
                    records.push(RavelAssessmentRecordView {
                        action_hash,
                        assessment,
                    });
                }
            }
        }
    }
    Ok(records)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UltimateRiskBearerRecordView {
    pub action_hash: ActionHash,
    pub bearer: UltimateRiskBearerEntry,
}

#[hdk_extern]
pub fn get_assessment_ultimate_risk_bearers(
    assessment_id: String,
) -> ExternResult<Vec<UltimateRiskBearerRecordView>> {
    let links = get_links(
        LinkQuery::new(
            ravel_assessment_base(&assessment_id)?,
            LinkTypes::AssessmentToUltimateRiskBearer.try_into_filter()?,
        ),
        GetStrategy::default(),
    )?;
    let mut records = Vec::new();
    for link in links {
        if let Some(action_hash) = link.target.into_action_hash() {
            if let Some(record) = get(action_hash.clone(), GetOptions::default())? {
                if let Ok(Some(bearer)) =
                    record.entry().to_app_option::<UltimateRiskBearerEntry>()
                {
                    records.push(UltimateRiskBearerRecordView { action_hash, bearer });
                }
            }
        }
    }
    Ok(records)
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
