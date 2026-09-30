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


fn package_base(package_id: &str) -> ExternResult<AnyLinkableHash> {
    Ok(Path::from(format!("evidence_package:{package_id}")).path_entry_hash()?.into())
}

fn claim_base(claim_uid: &str) -> ExternResult<AnyLinkableHash> {
    Ok(Path::from(format!("claim_uid:{claim_uid}")).path_entry_hash()?.into())
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
    let package_refs = entry.subject_refs.clone();
    let action_hash = create_entry(EntryTypes::ReviewAttestation(entry))?;
    for package_id in package_refs {
        create_link(
            package_base(&package_id)?,
            action_hash.clone(),
            LinkTypes::EvidencePackageToReview,
            (),
        )?;
    }
    Ok(action_hash)
}

#[hdk_extern]
pub fn create_evidence_package(
    entry: EvidencePackageEntry,
) -> ExternResult<ActionHash> {
    let subject_id = entry.subject_id.clone();
    let package_id = entry.id.clone();
    let action_hash = create_entry(EntryTypes::EvidencePackage(entry))?;
    create_link(
        subject_base(&subject_id)?,
        action_hash.clone(),
        LinkTypes::SubjectToEvidencePackage,
        (),
    )?;
    create_link(
        package_base(&package_id)?,
        action_hash.clone(),
        LinkTypes::EvidencePackageIdentity,
        (),
    )?;
    Ok(action_hash)
}

#[hdk_extern]
pub fn create_admissibility_decision(
    entry: AdmissibilityDecisionEntry,
) -> ExternResult<ActionHash> {
    let claim_uid = entry.claim_uid.clone();
    let action_hash = create_entry(EntryTypes::AdmissibilityDecision(entry))?;
    create_link(
        claim_base(&claim_uid)?,
        action_hash.clone(),
        LinkTypes::ClaimToAdmissibilityDecision,
        (),
    )?;
    Ok(action_hash)
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
pub struct EvidencePackageRecordView {
    pub action_hash: ActionHash,
    pub package: EvidencePackageEntry,
}

#[hdk_extern]
pub fn get_subject_evidence_packages(subject_id: String) -> ExternResult<Vec<EvidencePackageRecordView>> {
    let links = get_links(
        LinkQuery::new(
            subject_base(&subject_id)?,
            LinkTypes::SubjectToEvidencePackage.try_into_filter()?,
        ),
        GetStrategy::default(),
    )?;
    let mut records = Vec::new();
    for link in links {
        if let Some(action_hash) = link.target.into_action_hash() {
            if let Some(record) = get(action_hash.clone(), GetOptions::default())? {
                if let Ok(Some(package)) = record.entry().to_app_option::<EvidencePackageEntry>() {
                    records.push(EvidencePackageRecordView { action_hash, package });
                }
            }
        }
    }
    Ok(records)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReviewRecordView {
    pub action_hash: ActionHash,
    pub review: ReviewAttestationEntry,
}

#[hdk_extern]
pub fn get_package_reviews(package_id: String) -> ExternResult<Vec<ReviewRecordView>> {
    let links = get_links(
        LinkQuery::new(
            package_base(&package_id)?,
            LinkTypes::EvidencePackageToReview.try_into_filter()?,
        ),
        GetStrategy::default(),
    )?;
    let mut records = Vec::new();
    for link in links {
        if let Some(action_hash) = link.target.into_action_hash() {
            if let Some(record) = get(action_hash.clone(), GetOptions::default())? {
                if let Ok(Some(review)) = record.entry().to_app_option::<ReviewAttestationEntry>() {
                    records.push(ReviewRecordView { action_hash, review });
                }
            }
        }
    }
    Ok(records)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdmissibilityDecisionRecordView {
    pub action_hash: ActionHash,
    pub decision: AdmissibilityDecisionEntry,
}

#[hdk_extern]
pub fn get_claim_admissibility_decisions(
    claim_uid: String,
) -> ExternResult<Vec<AdmissibilityDecisionRecordView>> {
    let links = get_links(
        LinkQuery::new(
            claim_base(&claim_uid)?,
            LinkTypes::ClaimToAdmissibilityDecision.try_into_filter()?,
        ),
        GetStrategy::default(),
    )?;
    let mut records = Vec::new();
    for link in links {
        if let Some(action_hash) = link.target.into_action_hash() {
            if let Some(record) = get(action_hash.clone(), GetOptions::default())? {
                if let Ok(Some(decision)) = record.entry().to_app_option::<AdmissibilityDecisionEntry>() {
                    records.push(AdmissibilityDecisionRecordView { action_hash, decision });
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
