//! Experimental isolated coordinator; never use on existing signed DNA.
use hdk::prelude::*;
use prometheus_mrf_integrity_lab::{EntryTypes, MrfSyntheticReceipt};

#[hdk_extern]
pub fn create_synthetic_receipt(input: MrfSyntheticReceipt) -> ExternResult<ActionHash> {
    // Integrity zome rejects unpinned hashes, fake outputs and REAL data.
    create_entry(EntryTypes::SyntheticMrfReceipt(input))
}