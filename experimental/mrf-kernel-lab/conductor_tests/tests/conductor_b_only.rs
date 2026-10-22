//! Gate-5 diagnostic: two separate TEST conductor apps, call B only.
//! This isolates whether the second conductor's CREATE works *without* A CREATE.
//! Does NOT prove cross-peer DHT fetch, network agreement or physical evidence.
use holo_hash::ActionHash;
use holochain::sweettest::*;
use std::path::PathBuf;
use std::time::Duration;

fn receipt() -> serde_json::Value {
    serde_json::json!({
        "subject_id": "TEST-peer-B-only",
        "evidence_class": "TEST",
        "model_sha256": "e074715a26129c8f00e828774eadc71ef33f94a620c0dcd0e00216b6156bda5f",
        "input_i64": [250,90], "output_i64":[8,-2],
        "authority":"model_conformity_only", "financial_admission": false
    })
}
#[tokio::test(flavor="multi_thread")]
async fn second_conductor_can_create_without_first_conductor_write() {
    let path=PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..").join("experimental_dna")
        .join("prometheus_mrf_synthetic_lab.dna");
    let dna=SweetDnaFile::from_bundle(&path).await.expect("load exact TEST DNA");
    println!("B_ONLY_STAGE_INIT_BEGIN");
    let mut a=SweetConductor::from_standard_config().await;
    let mut b=SweetConductor::from_standard_config().await;
    let _app_a=a.setup_app("TEST-peer-A-idle",&[dna.clone()]).await.expect("A");
    let app_b=b.setup_app("TEST-peer-B-active",&[dna]).await.expect("B");
    let zome_b=app_b.cells()[0].clone().zome("prometheus_mrf_coordinator_lab");
    assert!(SweetConductor::exchange_peer_info([&a,&b]).await, "B peer discovery");
    println!("B_ONLY_STAGE_DISCOVERY_PASS");
    println!("B_ONLY_STAGE_B_CREATE_BEGIN");
    let _:ActionHash=tokio::time::timeout(
        Duration::from_secs(85),
        b.call(&zome_b,"create_synthetic_receipt",receipt())
    ).await.expect("B_ONLY_STAGE_CREATE_TIMEOUT");
    println!("B_ONLY_STAGE_B_CREATE_PASS");
}
