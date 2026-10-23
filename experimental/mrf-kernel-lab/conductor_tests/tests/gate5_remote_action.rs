//! Test-only two-conductor read-by-action-hash experiment.
//! The second conductor does not author the first agent's receipt.
use holo_hash::ActionHash;
use holochain::prelude::Record;
use prometheus_mrf_integrity_lab::MrfSyntheticReceipt;
use holochain::sweettest::*;
use std::path::PathBuf;
use std::time::{Duration, Instant};

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
    let suite_started = Instant::now();
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

    let forward_read_started = Instant::now();
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
    assert_eq!(record.action().author(), alice.agent_pubkey(), "G5_A_AUTHOR_PROVENANCE_MISMATCH");
    println!("G5_A_AUTHOR_PROVENANCE=PASS");
    // A hash-address match alone is insufficient: deserialize the app entry
    // on conductor B and assert the exact frozen model/input/output contract.
    let typed: MrfSyntheticReceipt = record.entry()
        .to_app_option()
        .expect("G5_REMOTE_RECEIPT_DECODE_FAILED")
        .expect("G5_REMOTE_RECEIPT_ENTRY_MISSING");
    assert_eq!(typed.subject_id, "TEST-gate5-remote-a", "G5_SUBJECT_MISMATCH");
    assert_eq!(typed.evidence_class, "TEST", "G5_REAL_GATE_BYPASS");
    assert_eq!(typed.model_sha256,
        "e074715a26129c8f00e828774eadc71ef33f94a620c0dcd0e00216b6156bda5f",
        "G5_MODEL_HASH_MISMATCH");
    assert_eq!(typed.input_i64, vec![250,90], "G5_INPUT_MISMATCH");
    assert_eq!(typed.output_i64, vec![8,-2], "G5_OUTPUT_MISMATCH");
    assert_eq!(typed.authority, "model_conformity_only", "G5_AUTHORITY_ESCALATION");
    assert!(!typed.financial_admission, "G5_FINANCIAL_ADMISSION_BYPASS");
    println!("G5_REMOTE_TYPED_PAYLOAD_MATCH=PASS");
    println!("G7_CI_HOST_READ_A_TO_B_MS={}", forward_read_started.elapsed().as_millis());

    // Gate 5.2: reverse direction, same immutable TEST DNA and two conductors.
    // The second author produces DIFFERENT canonical inputs and output vectors.
    // A must retrieve B's exact action and decode its typed entry independently.
    let mut b_input = test_receipt();
    b_input["subject_id"] = serde_json::json!("TEST-gate5-remote-b");
    b_input["input_i64"] = serde_json::json!([320,120]);
    b_input["output_i64"] = serde_json::json!([3,0]);
    println!("G5_B_AUTHOR_BEGIN");
    let bh: ActionHash = tokio::time::timeout(
        Duration::from_secs(120),
        batch[1].call(&zb, "create_synthetic_receipt", b_input),
    ).await.expect("G5_B_AUTHOR_TIMEOUT");
    assert_ne!(ah, bh, "G5_DISTINCT_ACTION_HASH_EXPECTED");
    println!("G5_B_AUTHOR=PASS");

    println!("G5_REVERSE_DHT_CONSISTENCY_BEGIN");
    tokio::time::timeout(
        Duration::from_secs(135),
        await_consistency_s(120, &[alice.clone(), bob.clone()]),
    ).await.expect("G5_REVERSE_CONSISTENCY_TIMEOUT")
     .expect("G5_REVERSE_OPS_NOT_INTEGRATED");
    println!("G5_REVERSE_DHT_CONSISTENCY=PASS");

    let reverse_read_started = Instant::now();
    let mut a_recovered: Option<Record> = None;
    for attempt in 0..5 {
        println!("G5_REMOTE_A_ATTEMPT={}", attempt + 1);
        let response: Option<Record> = tokio::time::timeout(
            Duration::from_secs(75),
            batch[0].call(&za, "get_synthetic_receipt_by_action", bh.clone()),
        ).await.expect("G5_A_GET_TIMEOUT");
        if response.is_some() {
            a_recovered = response;
            break;
        }
        tokio::time::sleep(Duration::from_secs(3)).await;
    }
    let a_record = a_recovered.expect("G5_REMOTE_A_GET_EMPTY");
    assert_eq!(a_record.action_address(), &bh, "G5_REVERSE_ACTION_HASH_MISMATCH");
    assert_eq!(a_record.action().author(), bob.agent_pubkey(), "G5_B_AUTHOR_PROVENANCE_MISMATCH");
    println!("G5_B_AUTHOR_PROVENANCE=PASS");
    let b_typed: MrfSyntheticReceipt = a_record.entry()
        .to_app_option()
        .expect("G5_REVERSE_RECEIPT_DECODE_FAILED")
        .expect("G5_REVERSE_RECEIPT_ENTRY_MISSING");
    assert_eq!(b_typed.subject_id, "TEST-gate5-remote-b");
    assert_eq!(b_typed.evidence_class, "TEST", "G5_REVERSE_REAL_GATE_BYPASS");
    assert_eq!(b_typed.model_sha256,
        "e074715a26129c8f00e828774eadc71ef33f94a620c0dcd0e00216b6156bda5f",
        "G5_REVERSE_MODEL_HASH_MISMATCH");
    assert_eq!(b_typed.input_i64, vec![320,120], "G5_REVERSE_INPUT_MISMATCH");
    assert_eq!(b_typed.output_i64, vec![3,0], "G5_REVERSE_OUTPUT_MISMATCH");
    assert_eq!(b_typed.authority, "model_conformity_only", "G5_REVERSE_AUTHORITY_ESCALATION");
    assert!(!b_typed.financial_admission, "G5_REVERSE_FINANCIAL_ADMISSION_BYPASS");
    println!("G5_BIDIRECTIONAL_TYPED_REPLICATION=PASS");
    println!("G7_CI_HOST_READ_B_TO_A_MS={}", reverse_read_started.elapsed().as_millis());
    // Independently sourced from PR #55 (which already passed on isolated CI):
    // a well-typed but never issued action hash must not resolve on B.
    // Timeout is a hard failure and must never be mistaken for a clean miss.
    let unknown_hash=ActionHash::from_raw_36(vec![0xa5;36]);
    assert_ne!(unknown_hash,ah,"G5_NEGATIVE_HASH_A_COLLISION");
    assert_ne!(unknown_hash,bh,"G5_NEGATIVE_HASH_B_COLLISION");
    let absent:Option<Record>=tokio::time::timeout(
        Duration::from_secs(75),
        batch[1].call(&zb,"get_synthetic_receipt_by_action",unknown_hash),
    ).await.expect("G5_UNKNOWN_HASH_LOOKUP_TIMEOUT");
    assert!(absent.is_none(),"G5_UNKNOWN_HASH_RETURNED_RECORD");
    println!("G5_UNKNOWN_ACTION_HASH_REJECTED=PASS");

    // G7.2: five successful repeated reads per direction AFTER propagation.
    // Each sample checks ActionHash, author and TEST payload invariants.
    // Warm reads may be served by local caches; these are NOT peer-to-peer RTTs.
    for sample in 1..=5 {
        let started = Instant::now();
        let found: Option<Record> = tokio::time::timeout(
            Duration::from_secs(75),
            batch[1].call(&zb, "get_synthetic_receipt_by_action", ah.clone()),
        ).await.expect("G7_REPEAT_A_TO_B_TIMEOUT");
        let found = found.expect("G7_REPEAT_A_TO_B_EMPTY");
        assert_eq!(found.action_address(), &ah, "G7_REPEAT_A_TO_B_ACTION_MISMATCH");
        assert_eq!(found.action().author(), alice.agent_pubkey(), "G7_REPEAT_A_TO_B_AUTHOR_MISMATCH");
        let entry: MrfSyntheticReceipt = found.entry().to_app_option()
            .expect("G7_REPEAT_A_TO_B_DECODE_FAILED")
            .expect("G7_REPEAT_A_TO_B_ENTRY_MISSING");
        assert_eq!(entry.subject_id, "TEST-gate5-remote-a");
        assert_eq!(entry.evidence_class, "TEST");
        assert_eq!(entry.input_i64, vec![250, 90]);
        assert_eq!(entry.output_i64, vec![8, -2]);
        assert_eq!(entry.authority, "model_conformity_only");
        assert!(!entry.financial_admission);
        println!("G7_CI_HOST_WARM_A_TO_B_{}_MS={}", sample, started.elapsed().as_millis());
    }
    for sample in 1..=5 {
        let started = Instant::now();
        let found: Option<Record> = tokio::time::timeout(
            Duration::from_secs(75),
            batch[0].call(&za, "get_synthetic_receipt_by_action", bh.clone()),
        ).await.expect("G7_REPEAT_B_TO_A_TIMEOUT");
        let found = found.expect("G7_REPEAT_B_TO_A_EMPTY");
        assert_eq!(found.action_address(), &bh, "G7_REPEAT_B_TO_A_ACTION_MISMATCH");
        assert_eq!(found.action().author(), bob.agent_pubkey(), "G7_REPEAT_B_TO_A_AUTHOR_MISMATCH");
        let entry: MrfSyntheticReceipt = found.entry().to_app_option()
            .expect("G7_REPEAT_B_TO_A_DECODE_FAILED")
            .expect("G7_REPEAT_B_TO_A_ENTRY_MISSING");
        assert_eq!(entry.subject_id, "TEST-gate5-remote-b");
        assert_eq!(entry.evidence_class, "TEST");
        assert_eq!(entry.input_i64, vec![320, 120]);
        assert_eq!(entry.output_i64, vec![3, 0]);
        assert_eq!(entry.authority, "model_conformity_only");
        assert!(!entry.financial_admission);
        println!("G7_CI_HOST_WARM_B_TO_A_{}_MS={}", sample, started.elapsed().as_millis());
    }
    println!("G7_CI_HOST_WARM_SAMPLE_COUNT=10");

    // Fail-closed adversarial preflight on the receiving/second conductor.
    // Check the *reported validator reason*, not merely is_err(), which may
    // be caused by a transport failure, timeout or unrelated Wasm panic.
    for (field, tampered, required_reason) in [
        ("output_i64", serde_json::json!([8,17]), "synthetic MRF result mismatch"),
        ("evidence_class", serde_json::json!("REAL"), "REAL_DATA_PERSISTENCE_GATE_CLOSED"),
        ("financial_admission", serde_json::json!(true), "model-conformity authority"),
    ] {
        let mut bad = test_receipt();
        bad[field] = tampered;
        let rejected: Result<ActionHash, _> = tokio::time::timeout(
            Duration::from_secs(45),
            batch[1].call_fallible(&zb, "create_synthetic_receipt", bad),
        ).await.expect("G5_ADVERSARIAL_CALL_TIMEOUT");
        let err = rejected.expect_err("G5_INVALID_RECEIPT_WAS_ACCEPTED");
        let trace = format!("{err:?}");
        assert!(
            trace.contains(required_reason),
            "G5_REJECTION_PROVENANCE_MISMATCH: field={field}, actual={trace}"
        );
        println!("G5_ADVERSARIAL_{field}_REJECTED_WITH_VALIDATOR_REASON=PASS");
    }

    // A's original valid immutable receipt must remain addressable by B
    // after B has attempted the three tampered synthetic writes.
    let after: Option<Record> = tokio::time::timeout(
        Duration::from_secs(75),
        batch[1].call(&zb, "get_synthetic_receipt_by_action", ah.clone()),
    ).await.expect("G5_POST_ATTACK_READ_TIMEOUT");
    let after = after.expect("G5_POST_ATTACK_VALID_RECORD_UNAVAILABLE");
    assert_eq!(after.action_address(), &ah, "G5_POST_ATTACK_HASH_MISMATCH");
    let after_typed: MrfSyntheticReceipt = after.entry()
        .to_app_option()
        .expect("G5_POST_ATTACK_DECODE_FAILED")
        .expect("G5_POST_ATTACK_ENTRY_MISSING");
    assert_eq!(after_typed.input_i64, vec![250,90]);
    assert_eq!(after_typed.output_i64, vec![8,-2]);
    assert_eq!(after_typed.evidence_class, "TEST");
    assert!(!after_typed.financial_admission);
    println!("G5_POST_ATTACK_VALID_RECORD_INTEGRITY=PASS");
    println!("G7_CI_HOST_TEST_WALL_MS={}", suite_started.elapsed().as_millis());

    // Both directions use one local CI runner. These assertions do NOT prove
    // cross-host resilience, partitions/rejoin, scientific truth or oracle quorum.
    // A remote get is not an authority-level validation or ecological audit.
}
