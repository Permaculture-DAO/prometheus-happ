//! Experimental isolated coordinator; never use on existing signed DNA.
use hdk::prelude::*;
use prometheus_mrf_integrity_lab::{EntryTypes, MrfSyntheticReceipt};

#[hdk_extern]
pub fn create_synthetic_receipt(input: MrfSyntheticReceipt) -> ExternResult<ActionHash> {
    // Integrity zome rejects unpinned hashes, fake outputs and REAL data.
    create_entry(EntryTypes::SyntheticMrfReceipt(input))
}

// Negative-only experimental attack surface. Never enable on production DNA.
// The integrity zome MUST reject these operations on the append-only MRF receipt.
#[hdk_extern]
pub fn update_synthetic_receipt(input: (ActionHash, MrfSyntheticReceipt)) -> ExternResult<ActionHash> {
    update_entry(input.0, &EntryTypes::SyntheticMrfReceipt(input.1))
}

#[hdk_extern]
pub fn delete_synthetic_receipt(original: ActionHash) -> ExternResult<ActionHash> {
    delete_entry(original)
}

#[hdk_extern]
pub fn link_synthetic_receipts(input: (ActionHash, ActionHash)) -> ExternResult<ActionHash> {
    create_link(
        input.0,
        input.1,
        prometheus_mrf_integrity_lab::LinkTypes::SyntheticReceipt,
        LinkTag::new(vec![]),
    )
}
