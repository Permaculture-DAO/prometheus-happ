//! Isolated two-conductor DHT propagation test: still not ecological truth.
//! Explicitly uses separate SweetConductor instances and agents, SAME TEST DNA.
use holo_hash::ActionHash;
use holochain::sweettest::{await_consistency, SweetConductorBatch, SweetDnaFile};
use std::path::PathBuf;
fn fixture() -> serde_json::Value {
    serde_json::json!({
        "subject_id":"TEST-two-peer-site", "evidence_class":"TEST",
        "model_sha256":"e074715a26129c8f00e828774eadc71ef33f94a620c0dcd0e00216b6156bda5f",
        "input_i64":[250,90], "output_i64":[8,-2],
        "authority":"model_conformity_only", "financial_admission":false
    })
}
#[tokio::test(flavor="multi_thread")]
async fn separate_conductors_replicate_only_valid_test_receipts() {
    let dna_path=PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..").join("experimental_dna")
        .join("prometheus_mrf_synthetic_lab.dna");
    let dna=SweetDnaFile::from_bundle(&dna_path).await.expect("exact TEST DNA");
    let mut conductors=SweetConductorBatch::from_standard_config_rendezvous(2).await;
    let ((alice,), (bob,))=conductors
        .setup_app("test-mrf-multi-peer",&[dna]).await
        .expect("two isolated conductor apps").into_tuples();
    println!("GATE_TWO_CONDUCTORS_STARTED=PASS");
    await_consistency(&[alice.clone(),bob.clone()]).await.expect("initial consistency");
    let _:ActionHash=conductors[0].call(&alice.zome("prometheus_mrf_coordinator_lab"),
        "create_synthetic_receipt",fixture()).await;
    let _:ActionHash=conductors[1].call(&bob.zome("prometheus_mrf_coordinator_lab"),
        "create_synthetic_receipt",fixture()).await;
    await_consistency(&[alice.clone(),bob.clone()]).await.expect("DHT consistency");
    println!("GATE_TWO_CONDUCTORS_CONSISTENCY=PASS");
    let mut forged=fixture();forged["output_i64"]=serde_json::json!([123,456]);
    let result:Result<ActionHash,_>=conductors[1].call_fallible(
        &bob.zome("prometheus_mrf_coordinator_lab"),
        "create_synthetic_receipt",forged).await;
    assert!(result.is_err(),"forged peer entry must be rejected");
    println!("GATE_TWO_CONDUCTORS_FORGERY_REJECTED=PASS");
}
