//! TEST-only two-conductor reversed write-order diagnosis.
//! Tests B CREATE followed by A CREATE; does not claim DHT replication.
use holo_hash::ActionHash;
use holochain::sweettest::*;
use std::path::PathBuf;
use std::time::Duration;
fn receipt(subject:&str)->serde_json::Value {
    serde_json::json!({
        "subject_id":subject,
        "evidence_class":"TEST",
        "model_sha256":"e074715a26129c8f00e828774eadc71ef33f94a620c0dcd0e00216b6156bda5f",
        "input_i64":[250,90], "output_i64":[8,-2],
        "authority":"model_conformity_only","financial_admission":false
    })
}
#[tokio::test(flavor="multi_thread")]
async fn two_independent_conductors_reversed_write_sequence() {
    let path=PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..").join("experimental_dna")
        .join("prometheus_mrf_synthetic_lab.dna");
    let dna=SweetDnaFile::from_bundle(&path).await.expect("exact TEST DNA");
    println!("REVERSE_STAGE_INIT");
    let mut a=SweetConductor::from_standard_config().await;
    let mut b=SweetConductor::from_standard_config().await;
    let app_a=a.setup_app("TEST-reverse-A",&[dna.clone()]).await.expect("A");
    let app_b=b.setup_app("TEST-reverse-B",&[dna]).await.expect("B");
    let zome_a=app_a.cells()[0].clone().zome("prometheus_mrf_coordinator_lab");
    let zome_b=app_b.cells()[0].clone().zome("prometheus_mrf_coordinator_lab");
    assert!(SweetConductor::exchange_peer_info([&a,&b]).await);
    println!("REVERSE_STAGE_PEER_INFO_PASS");
    println!("REVERSE_STAGE_B_FIRST_BEGIN");
    let _:ActionHash=tokio::time::timeout(
        Duration::from_secs(130),b.call(&zome_b,"create_synthetic_receipt",receipt("TEST-reverse-B"))
    ).await.expect("REVERSE_STAGE_B_FIRST_TIMEOUT");
    println!("REVERSE_STAGE_B_FIRST_PASS");
    println!("REVERSE_STAGE_A_SECOND_BEGIN");
    let _:ActionHash=tokio::time::timeout(
        Duration::from_secs(100),a.call(&zome_a,"create_synthetic_receipt",receipt("TEST-reverse-A"))
    ).await.expect("REVERSE_STAGE_A_SECOND_TIMEOUT");
    println!("REVERSE_STAGE_A_SECOND_PASS");
}
