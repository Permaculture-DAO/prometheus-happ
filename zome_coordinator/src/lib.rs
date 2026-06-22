use hdk::prelude::*;
use zome_integrity::admissibility::admissible;
use zome_integrity::entry_types::{ClaimEntry, EntryTypes, LinkTypes, MrvEvidenceEntry, OheEntry};

/// Deterministic anchor for a subject (e.g. an OHE id) to hang evidence links on.
/// Both writer and reader derive the same base from the subject id.
fn subject_base(subject_id: &str) -> ExternResult<AnyLinkableHash> {
    Ok(Path::from(format!("subject:{subject_id}"))
        .path_entry_hash()?
        .into())
}

/// Stable single-agent idempotency key: subject + indicator + observation time.
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
/// Sequential re-delivery of the same subject+indicator+observed_at returns the
/// existing action. Concurrent or multi-agent races remain outside this bounded
/// guarantee and must not be described as universally duplicate-proof.
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
