use hdk::prelude::*;
use zome_integrity::entry_types::{ClaimEntry, EntryTypes, LinkTypes, MrvEvidenceEntry, OheEntry};
use zome_integrity::admissibility::admissible;

/// Deterministic anchor for a subject (e.g. an OHE id) to hang evidence links on.
/// Both writer and reader derive the same base from the subject id.
fn subject_base(subject_id: &str) -> ExternResult<AnyLinkableHash> {
    Ok(Path::from(format!("subject:{subject_id}")).path_entry_hash()?.into())
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
    let subject = evidence.subject_id.clone();
    let action_hash = create_entry(EntryTypes::Evidence(evidence))?;
    // Link the subject anchor -> this evidence, so it is queryable by subject.
    create_link(
        subject_base(&subject)?,
        action_hash.clone(),
        LinkTypes::SubjectToEvidence,
        (),
    )?;
    Ok(action_hash)
}

#[hdk_extern]
pub fn create_claim(claim: ClaimEntry) -> ExternResult<ActionHash> {
    create_entry(EntryTypes::Claim(claim))
}

#[hdk_extern]
pub fn get_record(action_hash: ActionHash) -> ExternResult<Option<Record>> {
    get(action_hash, GetOptions::default())
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

/// On-chain spine: gather a subject's persisted MRV evidence (via links) and
/// run the binary admissibility gate over it. Returns a bool, never a value.
#[hdk_extern]
pub fn assess_subject_admissibility(input: SubjectAssessInput) -> ExternResult<AssessResult> {
    let links = get_links(
        LinkQuery::new(
            subject_base(&input.subject_id)?,
            LinkTypes::SubjectToEvidence.try_into_filter()?,
        ),
        GetStrategy::default(),
    )?;
    let mut evidence = Vec::new();
    for link in links {
        if let Some(ah) = link.target.into_action_hash() {
            if let Some(record) = get(ah, GetOptions::default())? {
                if let Ok(Some(entry)) = record.entry().to_app_option::<MrvEvidenceEntry>() {
                    evidence.push(entry.to_domain());
                }
            }
        }
    }
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
