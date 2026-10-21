//! Two separately initialized TEST-only Holochain conductors.
//! Peer-info exchange does NOT alone prove DHT propagation or partition tolerance.
use holo_hash::ActionHash;
use holochain::sweettest::*;
use std::path::PathBuf;
use std::time::Duration;

fn receipt() -> serde_json::Value {
    serde_json::json!({
        "subject_id":"TEST-two-conductors",
        "evidence_class":"TEST",
        "model_sha256":"e074715a26129c8f00e828774eadc71ef33f94a620c0dcd0e00216b6156bda5f",
        "input_i64":[250,90], "output_i64":[8,-2],
        "authority":"model_conformity_only","financial_admission":false
    })
}
#[tokio::test(flavor="multi_thread")]
async fn two_independent_conductors_peer_info_and_local_validation() {
    let dna_path=PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..").join("experimental_dna")
        .join("prometheus_mrf_synthetic_lab.dna");
    let dna=SweetDnaFile::from_bundle(&dna_path).await.expect("TEST DNA");
    let mut a=SweetConductor::from_standard_config().await;
    let mut b=SweetConductor::from_standard_config().await;
    let app_a=a.setup_app("TEST-mrf-a",&[dna.clone()]).await.expect("agent A");
    let app_b=b.setup_app("TEST-mrf-b",&[dna]).await.expect("agent B");
    let zome_a=app_a.cells()[0].clone().zome("prometheus_mrf_coordinator_lab");
    let zome_b=app_b.cells()[0].clone().zome("prometheus_mrf_coordinator_lab");
    assert!(SweetConductor::exchange_peer_info([&a,&b]).await,
        "two independently initialized conductor peer records must be exchangeable");
    println!("PEER_INFO_EXCHANGE=PASS");
    println!("STAGE_A_CREATE_BEGIN");
    let _:ActionHash=tokio::time::timeout(
        Duration::from_secs(75),
        a.call(&zome_a,"create_synthetic_receipt",receipt()),
    ).await.expect("STAGE_A_CREATE_TIMEOUT");
    println!("STAGE_A_CREATE_PASS");
    println!("STAGE_B_CREATE_BEGIN");
    let _:ActionHash=tokio::time::timeout(
        Duration::from_secs(75),
        b.call(&zome_b,"create_synthetic_receipt",receipt()),
    ).await.expect("STAGE_B_CREATE_TIMEOUT");
    println!("STAGE_B_CREATE_PASS");
    println!("TWO_CONDUCTORS_LOCAL_TEST_CONFORMITY=PASS");
    let mut bad=receipt();bad["evidence_class"]=serde_json::json!("REAL");
    println!("STAGE_B_REAL_REJECTION_BEGIN");
    let denied:Result<ActionHash,_>=tokio::time::timeout(
        Duration::from_secs(45),
        b.call_fallible(&zome_b,"create_synthetic_receipt",bad),
    ).await.expect("STAGE_B_REAL_REJECTION_TIMEOUT");
    assert!(denied.is_err(),"independent conductor must reject REAL");
    println!("SECOND_CONDUCTOR_REAL_REJECTION=PASS");
}
