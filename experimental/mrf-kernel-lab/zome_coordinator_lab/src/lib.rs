//! Experimental isolated coordinator; never use on existing signed DNA.
use hdk::prelude::*;
use prometheus_mrf_integrity_lab::{EntryTypes, LinkTypes, MrfSyntheticReceipt, FROZEN_TOY_MODEL_SHA256, TEST_AUTHORITY};

#[hdk_extern]
pub fn create_synthetic_receipt(input: MrfSyntheticReceipt) -> ExternResult<ActionHash> {
    // Integrity zome rejects unpinned hashes, fake outputs and REAL data.
    create_entry(EntryTypes::SyntheticMrfReceipt(input))
}

// TEST-only negative pathways. A correct DNA must reject ALL these actions.
#[hdk_extern]
pub fn try_update_synthetic_receipt(original: ActionHash) -> ExternResult<ActionHash> {
    let new_entry = MrfSyntheticReceipt {
        subject_id: "TEST-site-01".into(),
        evidence_class: "TEST".into(),
        model_sha256: FROZEN_TOY_MODEL_SHA256.into(),
        input_i64: vec![250,90],
        output_i64: vec![8,-2],
        authority: TEST_AUTHORITY.into(),
        financial_admission: false,
    };
    update_entry(original, EntryTypes::SyntheticMrfReceipt(new_entry))
}

#[hdk_extern]
pub fn try_delete_synthetic_receipt(original: ActionHash) -> ExternResult<ActionHash> {
    delete_entry(original)
}

#[hdk_extern]
pub fn try_link_synthetic_receipt(original: ActionHash) -> ExternResult<ActionHash> {
    create_link(original.clone(), original, LinkTypes::SyntheticReceipt, ())
}
