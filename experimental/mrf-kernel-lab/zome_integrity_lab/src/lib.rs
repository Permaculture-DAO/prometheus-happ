//! Experimental TEST-only Holochain integrity zome (no production DNA link).
//! Model arithmetic is shared with the verified native/WASM prototype.
//! Model hash is pinned to 115-byte synthetic PMRF-BIN-v1 artifact.
//! IMPORTANT: This callback proves deterministic model conformity, not truth.
use hdk::prelude::*;
use hdi::prelude::{FlatOp, OpEntry, OpRecord, OpUpdate};
use prometheus_mrf_kernel_lab::synthetic_forest;
use sha2::{Digest, Sha256};

pub const FROZEN_TOY_MODEL_SHA256: &str =
    "e074715a26129c8f00e828774eadc71ef33f94a620c0dcd0e00216b6156bda5f";
pub const TEST_AUTHORITY: &str = "model_conformity_only";

#[hdk_entry_helper]
#[derive(Clone)]
pub struct MrfSyntheticReceipt {
    pub subject_id: String,
    pub evidence_class: String,
    pub model_sha256: String,
    pub input_i64: Vec<i64>,
    pub output_i64: Vec<i64>,
    pub authority: String,
    pub financial_admission: bool,
}

impl MrfSyntheticReceipt {
    pub fn validate_entry(&self) -> ValidateCallbackResult {
        // Fail closed: never allow a REAL entry through this laboratory DNA.
        if self.evidence_class != "TEST" {
            return ValidateCallbackResult::Invalid(
                "REAL_DATA_PERSISTENCE_GATE_CLOSED: MRF receipts are TEST-only".into());
        }
        if !self.subject_id.starts_with("TEST-") || self.subject_id.len() < 6 {
            return ValidateCallbackResult::Invalid(
                "synthetic receipt requires TEST-namespaced subject".into());
        }
        // Bind the admitted digest to the exact canonical bytes actually used
        // by the deterministic interpreter; do not trust receipt metadata alone.
        let model_bytes = match synthetic_forest().to_bytes() {
            Ok(v) => v,
            Err(_) => return ValidateCallbackResult::Invalid("unserializable model".into()),
        };
        let actual_digest = format!("{:x}", Sha256::digest(&model_bytes));
        if actual_digest != FROZEN_TOY_MODEL_SHA256 {
            return ValidateCallbackResult::Invalid("embedded model hash mismatch".into());
        }
        if self.model_sha256 != actual_digest {
            return ValidateCallbackResult::Invalid(
                "model artifact hash is not admitted by experimental DNA".into());
        }
        if self.authority != TEST_AUTHORITY || self.financial_admission {
            return ValidateCallbackResult::Invalid(
                "inference has model-conformity authority only; no financial admission".into());
        }
        let expected = match synthetic_forest().infer(&self.input_i64) {
            Ok(y) => y,
            Err(_) => return ValidateCallbackResult::Invalid(
                "bad synthetic input shape or deterministic inference error".into()),
        };
        if self.output_i64 != expected {
            return ValidateCallbackResult::Invalid("synthetic MRF result mismatch".into());
        }
        ValidateCallbackResult::Valid
    }
}
#[hdk_entry_types]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    #[entry_type(visibility = "public")]
    SyntheticMrfReceipt(MrfSyntheticReceipt),
}
#[hdk_link_types]
pub enum LinkTypes { SyntheticReceipt }

#[hdk_extern]
pub fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    // Synthetic receipts are append-only. Reject all attempts to mutate or
    // remove past decisions and all link operations in this experimental DNA.
    // Other host/system operations, including agent activity, remain governed
    // by Holochain's standard validation constraints.
    let result = match op.flattened::<EntryTypes, LinkTypes>()? {
        FlatOp::StoreRecord(OpRecord::CreateEntry { app_entry, .. })
        | FlatOp::StoreEntry(OpEntry::CreateEntry { app_entry, .. }) => {
            match app_entry {
                EntryTypes::SyntheticMrfReceipt(e) => e.validate_entry(),
            }
        }
        FlatOp::StoreRecord(OpRecord::UpdateEntry { .. })
        | FlatOp::StoreRecord(OpRecord::UpdatePrivateEntry { .. })
        | FlatOp::StoreRecord(OpRecord::DeleteEntry { .. })
        | FlatOp::StoreEntry(OpEntry::UpdateEntry { .. })
        | FlatOp::RegisterUpdate(OpUpdate::Entry { .. })
        | FlatOp::RegisterUpdate(OpUpdate::PrivateEntry { .. })
        | FlatOp::RegisterDelete(_) => ValidateCallbackResult::Invalid(
            "MRF_SYNTHETIC_RECEIPTS_APPEND_ONLY: updates and deletes forbidden".into(),
        ),
        FlatOp::StoreRecord(OpRecord::CreatePrivateEntry { .. }) => {
            ValidateCallbackResult::Invalid(
                "MRF_SYNTHETIC_RECEIPTS_PUBLIC_TEST_ONLY".into(),
            )
        }
        FlatOp::StoreRecord(OpRecord::CreateLink { .. })
        | FlatOp::StoreRecord(OpRecord::DeleteLink { .. })
        | FlatOp::RegisterCreateLink { .. }
        | FlatOp::RegisterDeleteLink { .. } => ValidateCallbackResult::Invalid(
            "MRF_SYNTHETIC_NO_LINK_MUTATION: use separate governed provenance".into(),
        ),
        _ => ValidateCallbackResult::Valid,
    };
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn good() -> MrfSyntheticReceipt {
        MrfSyntheticReceipt {
            subject_id:"TEST-site-01".into(),
            evidence_class:"TEST".into(),
            model_sha256:FROZEN_TOY_MODEL_SHA256.into(),
            input_i64:vec![250,90],
            output_i64:vec![8,-2],
            authority:TEST_AUTHORITY.into(),
            financial_admission:false,
        }
    }
    fn invalid(e:&MrfSyntheticReceipt, needle:&str) {
        match e.validate_entry() {
            ValidateCallbackResult::Invalid(reason) =>
                assert!(reason.contains(needle),"actual: {reason}"),
            other => panic!("expected Invalid, got {other:?}")
        }
    }
    #[test] fn synthetic_gold_valid() {
        assert!(matches!(good().validate_entry(),ValidateCallbackResult::Valid));
    }
    #[test] fn synthetic_threshold_valid() {
        let mut e=good();e.input_i64=vec![300,100];e.output_i64=vec![3,0];
        assert!(matches!(e.validate_entry(),ValidateCallbackResult::Valid));
    }
    #[test] fn real_gate_fail_closed() {
        let mut e=good();e.evidence_class="REAL".into();
        invalid(&e,"REAL_DATA_PERSISTENCE_GATE_CLOSED");
    }
    #[test] fn namespace_required() {
        let mut e=good();e.subject_id="site-01".into();
        invalid(&e,"TEST-namespaced");
    }
    #[test] fn actual_baked_model_digest_test() {
        let bytes=synthetic_forest().to_bytes().unwrap();
        assert_eq!(bytes.len(),115);
        assert_eq!(format!("{:x}", Sha256::digest(&bytes)), FROZEN_TOY_MODEL_SHA256);
    }
    #[test] fn mismatched_model_rejected() {
        let mut e=good();e.model_sha256="bad".into();
        invalid(&e,"model artifact hash");
    }
    #[test] fn forged_result_rejected() {
        let mut e=good();e.output_i64=vec![8,0];
        invalid(&e,"result mismatch");
    }
    #[test] fn wrong_input_dimension_rejected() {
        let mut e=good();e.input_i64=vec![250];
        invalid(&e,"input shape");
    }
    #[test] fn reject_financial_admission() {
        let mut e=good();e.financial_admission=true;
        invalid(&e,"model-conformity authority");
    }
    #[test] fn reject_certifying_authority() {
        let mut e=good();e.authority="certified_regeneration".into();
        invalid(&e,"model-conformity authority");
    }
}