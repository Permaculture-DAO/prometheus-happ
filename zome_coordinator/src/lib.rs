use hdk::prelude::*;
use zome_integrity::entry_types::{ClaimEntry, EntryTypes, MrvEvidenceEntry, OheEntry};

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
    create_entry(EntryTypes::Evidence(evidence))
}

#[hdk_extern]
pub fn create_claim(claim: ClaimEntry) -> ExternResult<ActionHash> {
    create_entry(EntryTypes::Claim(claim))
}

#[hdk_extern]
pub fn get_record(action_hash: ActionHash) -> ExternResult<Option<Record>> {
    get(action_hash, GetOptions::default())
}
