//! Experimental isolated coordinator; never use on existing signed DNA.
use hdk::prelude::*;
use prometheus_mrf_integrity_lab::{EntryTypes, MrfSyntheticReceipt};

#[hdk_extern]
pub fn create_synthetic_receipt(input: MrfSyntheticReceipt) -> ExternResult<ActionHash> {
    // Integrity zome rejects unpinned hashes, fake outputs and REAL data.
    create_entry(EntryTypes::SyntheticMrfReceipt(input))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AttemptUpdateInput {
    pub original_action_hash: ActionHash,
    pub new_receipt: MrfSyntheticReceipt,
}
#[hdk_extern]
pub fn attempt_update_receipt(input: AttemptUpdateInput) -> ExternResult<ActionHash> {
    // This path is intentionally exposed ONLY in a TEST DNA to prove denial.
    update_entry(
        input.original_action_hash,
        &EntryTypes::SyntheticMrfReceipt(input.new_receipt),
    )
}
#[hdk_extern]
pub fn attempt_delete_receipt(action_hash: ActionHash) -> ExternResult<ActionHash> {
    // This path is intentionally exposed ONLY in a TEST DNA to prove denial.
    delete_entry(action_hash)
}
