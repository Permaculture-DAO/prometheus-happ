// Conductor integration test (sweettest, in-process — no hc sandbox TTY/cap dance).
// Asserts the integrity `validate` callback runs live: a valid OHE is created,
// and an invalid one (Active without baseline) is rejected.

use holo_hash::ActionHash;
use holochain::sweettest::*;
use std::path::PathBuf;

async fn setup() -> (SweetConductor, SweetZome) {
    let dna_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("dnas")
        .join("hearth")
        .join("hearth.dna");
    let dna = SweetDnaFile::from_bundle(&dna_path).await.unwrap();
    let mut conductor = SweetConductor::from_standard_config().await;
    let app = conductor.setup_app("prometheus", &[dna]).await.unwrap();
    let cell = app.cells()[0].clone();
    let zome = cell.zome("zome_coordinator");
    (conductor, zome)
}

#[tokio::test(flavor = "multi_thread")]
async fn valid_ohe_is_created() {
    let (conductor, zome) = setup().await;
    let payload = serde_json::json!({
        "id": "ohe-1", "steward": "agent-1", "site": "plot A",
        "status": "Proposed", "baseline_recorded": false
    });
    let _hash: ActionHash = conductor.call(&zome, "create_ohe", payload).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn active_without_baseline_is_rejected() {
    let (conductor, zome) = setup().await;
    let payload = serde_json::json!({
        "id": "ohe-2", "steward": "agent-1", "site": "plot B",
        "status": "Active", "baseline_recorded": false
    });
    let res: Result<ActionHash, _> = conductor.call_fallible(&zome, "create_ohe", payload).await;
    assert!(
        res.is_err(),
        "Active-without-baseline OHE must be rejected by the integrity validate callback"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_evidence_is_persisted_with_sensor_identity() {
    let (conductor, zome) = setup().await;
    let payload = serde_json::json!({
        "id": "test:ohe-1:sensor-a:soil_moisture:1900000000",
        "subject_id": "TEST-ohe-1",
        "sensor_id": "sensor-a",
        "indicator": "soil_moisture",
        "evidence_class": "TEST",
        "calibration_hash": null,
        "method_hash": "test-method-hash",
        "data_hash": "test-data-hash",
        "observed_at": 1900000000_i64,
        "confidence": 0.7,
        "missing_data": false,
        "reviewer": null
    });
    let result: serde_json::Value = conductor
        .call(&zome, "create_evidence_idempotent", payload)
        .await;
    assert_eq!(result.get("created").and_then(|v| v.as_bool()), Some(true));
}

#[tokio::test(flavor = "multi_thread")]
async fn real_evidence_fails_closed_at_persistence_boundary() {
    let (conductor, zome) = setup().await;
    let payload = serde_json::json!({
        "id": "ohe-1:sensor-a:soil_moisture:1900000000",
        "subject_id": "ohe-1",
        "sensor_id": "sensor-a",
        "indicator": "soil_moisture",
        "evidence_class": "REAL",
        "calibration_hash": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "method_hash": "method-hash",
        "data_hash": "data-hash",
        "observed_at": 1900000000_i64,
        "confidence": 0.7,
        "missing_data": false,
        "reviewer": null
    });
    let res: Result<serde_json::Value, _> = conductor
        .call_fallible(&zome, "create_evidence_idempotent", payload)
        .await;
    let error = res.expect_err("REAL evidence must remain persistence-gated");
    assert!(format!("{error:?}").contains("REAL_DATA_PERSISTENCE_GATE_CLOSED"));
}
