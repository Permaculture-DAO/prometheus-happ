//! Gate-5 network-read API sanity on the author conductor only.
//! Failure/success here must NOT be misrepresented as remote DHT proof.
use holo_hash::ActionHash;
use holochain::sweettest::*;
use serde::Deserialize;
use std::{path::PathBuf,time::Duration};
#[derive(Debug,Deserialize)]
struct AuditReceipt {
  subject_id:String,evidence_class:String,model_sha256:String,
  input_i64:Vec<i64>,output_i64:Vec<i64>,financial_admission:bool,
}
#[derive(Debug,Deserialize)]
struct AuditLookup {action_hash:ActionHash,receipt:AuditReceipt}
#[tokio::test(flavor="multi_thread")]
async fn author_can_read_exact_original_receipt() {
    let path=PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..").join("experimental_dna")
        .join("prometheus_mrf_synthetic_lab.dna");
    let dna=SweetDnaFile::from_bundle(&path).await.expect("exact TEST DNA");
    let mut a=SweetConductor::from_standard_config().await;
    let app=a.setup_app("TEST-author-local-read",&[dna]).await.expect("A");
    let zome=app.cells()[0].clone().zome("prometheus_mrf_coordinator_lab");
    let sample=serde_json::json!({
      "subject_id":"TEST-author-local-read",
      "evidence_class":"TEST",
      "model_sha256":"e074715a26129c8f00e828774eadc71ef33f94a620c0dcd0e00216b6156bda5f",
      "input_i64":[250,90],"output_i64":[8,-2],
      "authority":"model_conformity_only","financial_admission":false
    });
    println!("LOCAL_A_CREATE_BEGIN");
    let action:ActionHash=tokio::time::timeout(
      Duration::from_secs(90),a.call(&zome,"create_synthetic_receipt",sample)
    ).await.expect("LOCAL_A_CREATE_TIMEOUT");
    println!("LOCAL_A_CREATE_PASS");
    println!("LOCAL_A_GET_BEGIN");
    let found:Option<AuditLookup>=tokio::time::timeout(
      Duration::from_secs(65),a.call(&zome,"get_synthetic_receipt_for_audit",action.clone())
    ).await.expect("LOCAL_A_GET_TIMEOUT");
    let found=found.expect("LOCAL_A_GET_MISSING");
    assert_eq!(found.action_hash,action);
    assert_eq!(found.receipt.subject_id,"TEST-author-local-read");
    assert_eq!(found.receipt.evidence_class,"TEST");
    assert_eq!(found.receipt.input_i64,vec![250,90]);
    assert_eq!(found.receipt.output_i64,vec![8,-2]);
    assert_eq!(found.receipt.model_sha256,
      "e074715a26129c8f00e828774eadc71ef33f94a620c0dcd0e00216b6156bda5f");
    assert!(!found.receipt.financial_admission);
    println!("LOCAL_A_GET_EXACT_ACTION_PASS");
}
