//! Isolated candidate conductor proofs, using synthetic inputs only.
//! Persistence is not calibration, independent review or underwriting admission.
use holo_hash::ActionHash;
use holochain::{prelude::Record, sweettest::*};
use serde::Deserialize;
use serde_json::{json, Value};
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct AssessmentView {
    action_hash: ActionHash,
    assessment: Value,
}

#[derive(Debug, Deserialize)]
struct BearerView {
    action_hash: ActionHash,
    bearer: Value,
}

async fn setup() -> (SweetConductor, SweetZome) {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../dnas/hearth/hearth.dna");
    let dna = SweetDnaFile::from_bundle(&path).await.unwrap();
    let mut conductor = SweetConductor::from_standard_config().await;
    let app = conductor.setup_app("ravel-test", &[dna]).await.unwrap();
    let zome = app.cells()[0].zome("zome_coordinator");
    (conductor, zome)
}

fn assessment() -> Value {
    json!({
        "id": "TEST-ravel-1", "subject_id": "TEST-subject-1", "model_version": "v0.1-synthetic",
        "evidence_refs": ["TEST-fixture-not-field-evidence"],
        "expected_loss": 17.0, "es95": 80.0, "es99": 80.0,
        "ppci": 0.1, "rr_delta": 0.2, "urbc": 0.625, "confidence": 0.0,
        "vrrc": 0.0, "vrrc_status": "not_admitted", "mode": "shadow_underwriting",
        "authority_boundary": "evaluation_not_certification", "created_at": 1900000000
    })
}

#[tokio::test(flavor = "multi_thread")]
async fn ravel_records_round_trip_without_financial_authority() {
    let (conductor, zome) = setup().await;
    let payload = assessment();
    let hash: ActionHash = conductor
        .call(&zome, "create_ravel_assessment", payload.clone())
        .await;
    let rows: Vec<AssessmentView> = conductor
        .call(
            &zome,
            "get_subject_ravel_assessments",
            "TEST-subject-1".to_string(),
        )
        .await;
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].action_hash, hash);
    assert_eq!(rows[0].assessment, payload);
    let bearer = json!({"assessment_id": "TEST-ravel-1", "scenario_id": "TEST-tail", "bearer_id": "TEST-insurer", "economic_group_id": "TEST-group", "retained_loss": 16.0, "model_version": "v0.1-synthetic"});
    let bearer_hash: ActionHash = conductor
        .call(&zome, "create_ultimate_risk_bearer", bearer.clone())
        .await;
    let bearers: Vec<BearerView> = conductor
        .call(
            &zome,
            "get_assessment_ultimate_risk_bearers",
            "TEST-ravel-1".to_string(),
        )
        .await;
    assert_eq!(bearers.len(), 1);
    assert_eq!(bearers[0].action_hash, bearer_hash);
    assert_eq!(bearers[0].bearer, bearer);
    let brake = json!({"assessment_id": "TEST-ravel-1", "severity": "Red", "reasons": ["synthetic review test"], "review_required": true, "autonomous_enforcement": false, "created_at": 1900000000});
    let brake_hash: ActionHash = conductor
        .call(&zome, "create_ravel_brake_signal", brake)
        .await;
    let record: Option<Record> = conductor
        .call(&zome, "get_record", brake_hash.clone())
        .await;
    let stored = record.expect("created brake record must be readable");
    assert_eq!(stored.action_address(), &brake_hash);
    assert!(stored.entry().as_option().is_some());
}

#[tokio::test(flavor = "multi_thread")]
async fn ravel_nonzero_credit_and_autonomous_brake_are_rejected() {
    let (conductor, zome) = setup().await;
    let mut payload = assessment();
    payload["vrrc"] = json!(1.0);
    let result: Result<ActionHash, _> = conductor
        .call_fallible(&zome, "create_ravel_assessment", payload)
        .await;
    let error = result.expect_err("VRRC must remain zero");
    assert!(format!("{error:?}").contains("shadow-underwriting/non-authority"));
    let mut payload = assessment();
    payload["mode"] = json!("underwriting_approved");
    let result: Result<ActionHash, _> = conductor
        .call_fallible(&zome, "create_ravel_assessment", payload)
        .await;
    let error = result.expect_err("Non-shadow mode must remain forbidden");
    assert!(format!("{error:?}").contains("shadow-underwriting/non-authority"));
    let brake = json!({"assessment_id": "TEST-ravel-1", "severity": "Red", "reasons": ["synthetic"], "review_required": true, "autonomous_enforcement": true, "created_at": 1900000000});
    let result: Result<ActionHash, _> = conductor
        .call_fallible(&zome, "create_ravel_brake_signal", brake)
        .await;
    let error = result.expect_err("Autonomous enforcement must remain disabled");
    assert!(format!("{error:?}").contains("review/non-enforcement"));
    let brake = json!({"assessment_id": "TEST-ravel-1", "severity": "Red", "reasons": ["synthetic"], "review_required": false, "autonomous_enforcement": false, "created_at": 1900000000});
    let result: Result<ActionHash, _> = conductor
        .call_fallible(&zome, "create_ravel_brake_signal", brake)
        .await;
    let error = result.expect_err("Red signal must require human review");
    assert!(format!("{error:?}").contains("review/non-enforcement"));
    let rows: Vec<AssessmentView> = conductor
        .call(
            &zome,
            "get_subject_ravel_assessments",
            "TEST-subject-1".to_string(),
        )
        .await;
    assert!(rows.is_empty());
}
