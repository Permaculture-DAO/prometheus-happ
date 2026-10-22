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


/// EXPERIMENTAL TEST-only read endpoint: a remote agent may request the record
/// by the author-provided immutable ActionHash. This is not certification.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SyntheticReceiptLookup {
    pub action_hash: ActionHash,
    pub receipt: MrfSyntheticReceipt,
}

#[hdk_extern]
pub fn get_synthetic_receipt_for_audit(
    requested_action_hash: ActionHash,
) -> ExternResult<Option<SyntheticReceiptLookup>> {
    let Some(record) = get(requested_action_hash.clone(), GetOptions::network())? else {
        return Ok(None);
    };
    if record.action_address() != &requested_action_hash {
        return Err(wasm_error!(WasmErrorInner::Guest(
            "MRF_TEST_ACTION_HASH_MISMATCH".into(),
        )));
    }
    let Some(receipt) = record.entry().to_app_option::<MrfSyntheticReceipt>()
        .map_err(|_| wasm_error!(WasmErrorInner::Guest(
            "MRF_TEST_RECEIPT_DECODE_FAILURE".into(),
        )))? else {
        return Err(wasm_error!(WasmErrorInner::Guest(
            "MRF_TEST_UNEXPECTED_ENTRY_TYPE".into(),
        )));
    };
    // The integrity zome is authoritative for TEST entry validity. This
    // coordinator only narrows the typed query and returns evidence for audit.
    Ok(Some(SyntheticReceiptLookup {
        action_hash: requested_action_hash,
        receipt,
    }))
}
