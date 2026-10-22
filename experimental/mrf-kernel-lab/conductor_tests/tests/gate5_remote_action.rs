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
    // Holochain v0.6.1 upstream Sweettest's verified multi-node pattern:
    // a shared rendezvous/bootstrap configuration, but distinct conductor
    // state stores and agent identities. Do not use this with any live app.
    let mut batch=SweetConductorBatch::from_standard_config_rendezvous(2).await;
    let ((alice,), (bob,))=batch
        .setup_app("TEST-g5-isolated",&[dna])
        .await.expect("two isolated apps")
        .into_tuples();
    assert_ne!(alice.agent_pubkey(), bob.agent_pubkey(), "G5_DISTINCT_AGENTS_REQUIRED");
    println!("G5_DISTINCT_AGENTS=PASS");
    let za=alice.zome("prometheus_mrf_coordinator_lab");
    let zb=bob.zome("prometheus_mrf_coordinator_lab");

    println!("G5_GENESIS_CONSISTENCY_BEGIN");
    tokio::time::timeout(
        Duration::from_secs(135),
        await_consistency_s(120,&[alice.clone(),bob.clone()])
    ).await.expect("G5_GENESIS_CONSISTENCY_TIMEOUT")
     .expect("G5_GENESIS_OPS_NOT_INTEGRATED");
    println!("G5_GENESIS_CONSISTENCY=PASS");

    println!("G5_AUTHOR_A_BEGIN");
    let ah:ActionHash=tokio::time::timeout(
        Duration::from_secs(120),
        batch[0].call(&za,"create_synthetic_receipt",test_receipt())
    ).await.expect("G5_A_CREATE_TIMEOUT");
    println!("G5_AUTHOR_A=PASS");

    // Source-verified Holochain consistency helper checks DHT op integration
    // across the two conductor databases, stronger than peer-info exchange.
    println!("G5_DHT_CONSISTENCY_BEGIN");
    tokio::time::timeout(
        Duration::from_secs(135),
        await_consistency_s(120,&[alice.clone(),bob.clone()])
    ).await.expect("G5_DHT_CONSISTENCY_TIMEOUT")
     .expect("G5_DHT_OPS_NOT_INTEGRATED");
    println!("G5_DHT_CONSISTENCY=PASS");

    let mut recovered:Option<Record>=None;
    for attempt in 0..5 {
        println!("G5_REMOTE_B_ATTEMPT={}",attempt+1);
        let response:Option<Record>=tokio::time::timeout(
            Duration::from_secs(75),
            batch[1].call(&zb,"get_synthetic_receipt_by_action",ah.clone())
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
    assert_eq!(record.action().author(), alice.agent_pubkey(), "G5_ACTION_AUTHOR_MISMATCH");
    println!("G5_AUTHOR_PROVENANCE=PASS");

    // An independently constructed, validly typed but unissued action hash
    // must not resolve to an existing receipt on B. Keep the negative probe
    // bounded and fail closed; do not turn timeout into success.
    let unknown_hash=ActionHash::from_raw_36(vec![0xa5;36]);
    assert_ne!(unknown_hash,ah,"G5_NEGATIVE_HASH_COLLISION");
    let unknown:Option<Record>=tokio::time::timeout(
        Duration::from_secs(75),
        batch[1].call(&zb,"get_synthetic_receipt_by_action",unknown_hash)
    ).await.expect("G5_UNKNOWN_HASH_LOOKUP_TIMEOUT");
    assert!(unknown.is_none(),"G5_UNKNOWN_HASH_RETURNED_RECORD");
    println!("G5_UNKNOWN_ACTION_HASH_REJECTED=PASS");
    // A remote get is not an authority-level validation or ecological audit.
}
