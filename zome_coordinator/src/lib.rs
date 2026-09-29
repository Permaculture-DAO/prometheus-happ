use hdk::prelude::*;
use zome_integrity::admissibility::admissible;
use zome_integrity::entry_types::{
    AdmissibilityDecisionEntry, ClaimEntry, DisturbanceEntry, EntryTypes, EvidencePackageEntry,
    LinkTypes, MrvEvidenceEntry, ObservationEntry, OheEntry, PlaceContextEntry,
    RelationshipAssertionEntry, ReviewAttestationEntry, SemanticClaimRefEntry, SystemBoundaryEntry,
};

fn subject_base(subject_id: &str) -> ExternResult<AnyLinkableHash> {
    Ok(Path::from(format!("subject:{subject_id}")).path_entry_hash()?.into())
}

fn evidence_identity_base(evidence: &MrvEvidenceEntry) -> ExternResult<AnyLinkableHash> {
    Ok(Path::from(format!(
        "evidence_identity:{}:{}:{}",
        evidence.subject_id, evidence.indicator, evidence.observed_at
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

// Legacy/current CRUD.
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
    pub created: bool,
}

#[hdk_extern]
pub fn create_evidence_idempotent(
    evidence: MrvEvidenceEntry,
) -> ExternResult<CreateEvidenceResult> {
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
        return Ok(CreateEvidenceResult { action_hash, created: false });
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
    Ok(CreateEvidenceResult { action_hash, created: true })
}

#[hdk_extern]
pub fn create_claim(claim: ClaimEntry) -> ExternResult<ActionHash> {
    create_entry(EntryTypes::Claim(claim))
}

// Candidate vNext CRUD. Integrity zome validates each object. No function below
// evaluates ecological truth, creates PRU value, or certifies a claim.
#[hdk_extern]
pub fn create_place_context(entry: PlaceContextEntry) -> ExternResult<ActionHash> {
    create_entry(EntryTypes::PlaceContext(entry))
}

#[hdk_extern]
pub fn create_system_boundary(entry: SystemBoundaryEntry) -> ExternResult<ActionHash> {
    create_entry(EntryTypes::SystemBoundary(entry))
}

#[hdk_extern]
pub fn create_observation(entry: ObservationEntry) -> ExternResult<ActionHash> {
    create_entry(EntryTypes::Observation(entry))
}

#[hdk_extern]
pub fn create_relationship_assertion(
    entry: RelationshipAssertionEntry,
) -> ExternResult<ActionHash> {
    create_entry(EntryTypes::RelationshipAssertion(entry))
}

#[hdk_extern]
pub fn create_disturbance(entry: DisturbanceEntry) -> ExternResult<ActionHash> {
    create_entry(EntryTypes::Disturbance(entry))
}

#[hdk_extern]
pub fn create_review_attestation(
    entry: ReviewAttestationEntry,
) -> ExternResult<ActionHash> {
    create_entry(EntryTypes::ReviewAttestation(entry))
}

#[hdk_extern]
pub fn create_evidence_package(
    entry: EvidencePackageEntry,
) -> ExternResult<ActionHash> {
    create_entry(EntryTypes::EvidencePackage(entry))
}

#[hdk_extern]
pub fn create_admissibility_decision(
    entry: AdmissibilityDecisionEntry,
) -> ExternResult<ActionHash> {
    create_entry(EntryTypes::AdmissibilityDecision(entry))
}

#[hdk_extern]
pub fn create_semantic_claim_ref(
    entry: SemanticClaimRefEntry,
) -> ExternResult<ActionHash> {
    create_entry(EntryTypes::SemanticClaimRef(entry))
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
                    records.push(EvidenceRecordView { action_hash, evidence });
                }
            }
        }
    }
    Ok(records)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssessInput {
    pub legal_gate: bool,
    pub confidence_threshold: f64,
    pub require_reviewer: bool,
    pub evidence: Vec<MrvEvidenceEntry>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssessResult {
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
