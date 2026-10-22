//! Gate 5 proposed cross-peer TEST receipt lookup; not yet verified.
//! A authors one immutable TEST receipt. B, on a separate SweetConductor,
//! tries a network get by *A's* exact ActionHash without authoring anything.
//! This does not establish DHT authority consensus or source authenticity.
use holo_hash::ActionHash;
use holochain::sweettest::*;
use serde::Deserialize;
use std::{path::PathBuf, time::Duration};

#[derive(Clone,Debug,Deserialize)]
struct AuditReceipt {
    subject_id:String,
    evidence_class:String,
    model_sha256:String,
    input_i64:Vec<i64>,
    output_i64:Vec<i64>,
    authority:String,
    financial_admission:bool,
}
#[derive(Clone,Debug,Deserialize)]
struct AuditLookup { action_hash:ActionHash, receipt:AuditReceipt }

fn evidence() -> serde_json::Value {
    serde_json::json!({
        "subject_id":"TEST-A-remote-proof-01",
        "evidence_class":"TEST",
        "model_sha256":"e074715a26129c8f00e828774eadc71ef33f94a620c0dcd0e00216b6156bda5f",
        "input_i64":[250,90], "output_i64":[8,-2],
        "authority":"model_conformity_only", "financial_admission":false
    })
}
#[tokio::test(flavor="multi_thread")]
async fn second_conductor_gets_original_receipt_without_own_write() {
    let path=PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..").join("experimental_dna")
        .join("prometheus_mrf_synthetic_lab.dna");
    let dna=SweetDnaFile::from_bundle(&path).await.expect("exact TEST DNA");
    println!("CROSSPEER_STAGE_INIT_BEGIN");
    let mut a=SweetConductor::from_standard_config().await;
    let mut b=SweetConductor::from_standard_config().await;
    let app_a=a.setup_app("TEST-crosspeer-author",&[dna.clone()]).await.expect("A");
    let app_b=b.setup_app("TEST-crosspeer-reader",&[dna]).await.expect("B");
    let zome_a=app_a.cells()[0].clone().zome("prometheus_mrf_coordinator_lab");
    let zome_b=app_b.cells()[0].clone().zome("prometheus_mrf_coordinator_lab");
    assert!(SweetConductor::exchange_peer_info([&a,&b]).await);
    println!("CROSSPEER_PEER_INFO_PASS");

    println!("CROSSPEER_A_CREATE_BEGIN");
    let original:ActionHash=tokio::time::timeout(
        Duration::from_secs(100),
        a.call(&zome_a,"create_synthetic_receipt",evidence())
    ).await.expect("CROSSPEER_A_CREATE_TIMEOUT");
    println!("CROSSPEER_A_CREATE_PASS");

    // A single successful remote read would demonstrate this narrow hypothesis.
    // Absence/errors/timeouts must remain explicit FAIL/UNKNOWN, never a proof.
    for attempt in 0..4 {
        println!("CROSSPEER_B_NETWORK_GET_ATTEMPT_{}",attempt+1);
        let got:Result<Option<AuditLookup>,_>=tokio::time::timeout(
            Duration::from_secs(40),
            b.call_fallible(&zome_b,"get_synthetic_receipt_for_audit",original.clone())
        ).await.expect("CROSSPEER_B_GET_TIMEOUT");
        match got {
            Ok(Some(lookup)) => {
                assert_eq!(lookup.action_hash,original,"remote record action hash differs");
                assert_eq!(lookup.receipt.subject_id,"TEST-A-remote-proof-01");
                assert_eq!(lookup.receipt.evidence_class,"TEST");
                assert_eq!(lookup.receipt.input_i64,vec![250,90]);
                assert_eq!(lookup.receipt.output_i64,vec![8,-2]);
                assert_eq!(lookup.receipt.model_sha256,
                    "e074715a26129c8f00e828774eadc71ef33f94a620c0dcd0e00216b6156bda5f");
                assert_eq!(lookup.receipt.authority,"model_conformity_only");
                assert!(!lookup.receipt.financial_admission);
                println!("CROSSPEER_B_GET_EXACT_A_ACTION_PASS");
                return;
            }
            Ok(None) => println!("CROSSPEER_B_GET_NONE"),
            Err(err) => panic!("CROSSPEER_B_GET_CALL_ERROR: {err:?}"),
        }
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
    panic!("CROSSPEER_NO_REMOTE_RECEIPT_WITHIN_BOUNDED_WINDOW");
}
