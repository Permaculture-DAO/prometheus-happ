//! Test-only two-conductor read-by-action-hash experiment.
//! The second conductor does not author the first agent's receipt.
use holo_hash::ActionHash;
use holochain::prelude::Record;
use holochain::sweettest::*;
use std::path::PathBuf;
use std::time::Duration;

fn test_receipt() -> serde_json::Value {
    serde_json::json!({
        "subject_id":"TEST-gate5-remote-a",
        "evidence_class":"TEST",
        "model_sha256":"e074715a26129c8f00e828774eadc71ef33f94a620c0dcd0e00216b6156bda5f",
        "input_i64":[250,90],
        "output_i64":[8,-2],
        "authority":"model_conformity_only",
        "financial_admission":false
    })
}

#[tokio::test(flavor="multi_thread")]
async fn test_gate5_remote_get_by_action_hash() {
    let dna_path=PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..").join("experimental_dna")
        .join("prometheus_mrf_synthetic_lab.dna");
    let dna=SweetDnaFile::from_bundle(&dna_path).await.expect("TEST DNA");
    let mut a=SweetConductor::from_standard_config().await;
    let mut b=SweetConductor::from_standard_config().await;
    let app_a=a.setup_app("TEST-g5-a",&[dna.clone()]).await.expect("A setup");
    let app_b=b.setup_app("TEST-g5-b",&[dna]).await.expect("B setup");
    let za=app_a.cells()[0].clone().zome("prometheus_mrf_coordinator_lab");
    let zb=app_b.cells()[0].clone().zome("prometheus_mrf_coordinator_lab");

    let exchanged=tokio::time::timeout(
        Duration::from_secs(90),SweetConductor::exchange_peer_info([&a,&b])
    ).await.expect("G5_DISCOVERY_TIMEOUT");
    assert!(exchanged,"G5_DISCOVERY_NO_PEER_INFO");
    println!("G5_DISCOVERY=PASS");

    println!("G5_AUTHOR_A_BEGIN");
    let ah:ActionHash=tokio::time::timeout(
        Duration::from_secs(120),
        a.call(&za,"create_synthetic_receipt",test_receipt())
    ).await.expect("G5_A_CREATE_TIMEOUT");
    println!("G5_AUTHOR_A=PASS");

    let mut recovered:Option<Record>=None;
    for attempt in 0..5 {
        println!("G5_REMOTE_B_ATTEMPT={}",attempt+1);
        let response:Option<Record>=tokio::time::timeout(
            Duration::from_secs(40),
            b.call(&zb,"get_synthetic_receipt_by_action",ah.clone())
        ).await.expect("G5_B_GET_TIMEOUT");
        if response.is_some() {
            recovered=response;
            break;
        }
        tokio::time::sleep(Duration::from_secs(3)).await;
    }
    let record=recovered.expect("G5_REMOTE_B_GET_EMPTY");
    assert_eq!(record.action_address(),&ah,"G5_ACTION_HASH_MISMATCH");
    println!("G5_REMOTE_ACTION_HASH_MATCH=PASS");
    // A remote get is not an authority-level validation or ecological audit.
}
