//! Experimental in-process Holochain conductor sweettests. TEST only.
//! Bundle is a separate lab DNA; production hearth DNA is never installed.
use holo_hash::ActionHash;
use holochain::sweettest::*;
use std::path::PathBuf;

async fn setup() -> (SweetConductor, SweetZome) {
    let dna_path=PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..").join("experimental_dna")
        .join("prometheus_mrf_synthetic_lab.dna");
    let dna=SweetDnaFile::from_bundle(&dna_path).await.expect("lab DNA");
    let mut conductor=SweetConductor::from_standard_config().await;
    let app=conductor.setup_app("synthetic-mrf", &[dna]).await.expect("lab app");
    let cell=app.cells()[0].clone();
    let zome=cell.zome("prometheus_mrf_coordinator_lab");
    (conductor,zome)
}

fn toy() -> serde_json::Value {
    serde_json::json!({
        "subject_id":"TEST-site-01", "evidence_class":"TEST",
        "model_sha256":"e074715a26129c8f00e828774eadc71ef33f94a620c0dcd0e00216b6156bda5f",
        "input_i64":[250,90], "output_i64":[8,-2],
        "authority":"model_conformity_only", "financial_admission":false
    })
}

#[tokio::test(flavor="multi_thread")]
async fn test_receipt_accepted() {
    let (conductor,zome)=setup().await;
    let _:ActionHash=conductor.call(&zome,"create_synthetic_receipt",toy()).await;
}
#[tokio::test(flavor="multi_thread")]
async fn corrupt_output_rejected_by_integrity() {
    let (conductor,zome)=setup().await;
    let mut x=toy();x["output_i64"]=serde_json::json!([8,17]);
    let outcome:Result<ActionHash,_>=conductor.call_fallible(&zome,"create_synthetic_receipt",x).await;
    assert!(outcome.is_err(),"invalid model result must be rejected");
}
#[tokio::test(flavor="multi_thread")]
async fn real_receipt_rejected_by_integrity() {
    let (conductor,zome)=setup().await;
    let mut x=toy();x["evidence_class"]=serde_json::json!("REAL");
    let outcome:Result<ActionHash,_>=conductor.call_fallible(&zome,"create_synthetic_receipt",x).await;
    assert!(outcome.is_err(),"REAL persistence must remain closed");
}
#[tokio::test(flavor="multi_thread")]
async fn invalid_model_hash_rejected() {
    let (conductor,zome)=setup().await;
    let mut x=toy();x["model_sha256"]=serde_json::json!("falsified");
    let outcome:Result<ActionHash,_>=conductor.call_fallible(&zome,"create_synthetic_receipt",x).await;
    assert!(outcome.is_err(),"unregistered model must be rejected");
}
#[tokio::test(flavor="multi_thread")]
async fn financing_claim_rejected() {
    let (conductor,zome)=setup().await;
    let mut x=toy();x["financial_admission"]=serde_json::json!(true);
    let outcome:Result<ActionHash,_>=conductor.call_fallible(&zome,"create_synthetic_receipt",x).await;
    assert!(outcome.is_err(),"synthetic evidence cannot promote financial rights");
}