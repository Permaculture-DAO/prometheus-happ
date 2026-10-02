// Conductor integration test (sweettest, in-process — no hc sandbox TTY/cap dance).
// Asserts the integrity `validate` callback runs live: a valid OHE is created,
// and an invalid one (Active without baseline) is rejected.

use holo_hash::{ActionHash, AgentPubKey};
use holochain::prelude::DnaModifiersOpt;
use holochain::sweettest::*;
use holochain_serialized_bytes::prelude::*;
use std::path::PathBuf;

// ActionHash uses MessagePack bytes; serde_json::Value cannot decode the binary
// return field. Match the coordinator's typed response rather than losing it.
#[derive(Debug, serde::Deserialize)]
struct CreateEvidenceResult {
    action_hash: ActionHash,
    created: bool,
}

#[derive(Debug, serde::Deserialize)]
struct EvidenceView {
    action_hash: ActionHash,
    evidence: PersistedEvidence,
}

#[derive(Debug, serde::Deserialize)]
struct PersistedEvidence {
    sensor_id: String,
    evidence_class: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, SerializedBytes)]
struct TestDnaProperties {
    real_data_enabled: bool,
    real_data_authority: Option<AgentPubKey>,
    approved_reviewers: Vec<AgentPubKey>,
}

#[derive(Debug, serde::Deserialize)]
struct GateStatus {
    real_data_enabled: bool,
    authority_configured: bool,
    approved_reviewer_count: usize,
}

#[derive(Debug, serde::Deserialize)]
struct CalibrationResolveResult {
    status: String,
    action_hash: Option<ActionHash>,
    calibration_hash: Option<String>,
}

fn dna_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("dnas")
        .join("hearth")
        .join("hearth.dna")
}

async fn setup_control_plane() -> (
    SweetConductor,
    SweetCell,
    SweetCell,
    SweetCell,
    SweetCell,
    AgentPubKey,
    AgentPubKey,
    AgentPubKey,
    AgentPubKey,
) {
    let mut conductor = SweetConductor::from_standard_config().await;
    let agents = SweetAgents::get(conductor.keystore(), 4).await;
    let authority = agents[0].clone();
    let reviewer = agents[1].clone();
    let rogue = agents[2].clone();
    let evidence_author = agents[3].clone();

    let props = TestDnaProperties {
        real_data_enabled: false,
        real_data_authority: Some(authority.clone()),
        // evidence_author is deliberately approved too so the self-review test
        // proves independence, rather than failing only on reviewer allowlisting.
        approved_reviewers: vec![reviewer.clone(), evidence_author.clone()],
    };
    let modifiers = DnaModifiersOpt::none().with_properties(props);
    let dna = SweetDnaFile::from_bundle_with_overrides(&dna_path(), modifiers)
        .await
        .unwrap();

    let authority_app = conductor
        .setup_app_for_agent("authority", authority.clone(), std::slice::from_ref(&dna))
        .await
        .unwrap();
    let reviewer_app = conductor
        .setup_app_for_agent("reviewer", reviewer.clone(), std::slice::from_ref(&dna))
        .await
        .unwrap();
    let rogue_app = conductor
        .setup_app_for_agent("rogue", rogue.clone(), std::slice::from_ref(&dna))
        .await
        .unwrap();
    let evidence_app = conductor
        .setup_app_for_agent("evidence", evidence_author.clone(), std::slice::from_ref(&dna))
        .await
        .unwrap();

    (
        conductor,
        authority_app.cells()[0].clone(),
        reviewer_app.cells()[0].clone(),
        rogue_app.cells()[0].clone(),
        evidence_app.cells()[0].clone(),
        authority,
        reviewer,
        rogue,
        evidence_author,
    )
}

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
    let result: CreateEvidenceResult = conductor
        .call(&zome, "create_evidence_idempotent", payload.clone())
        .await;
    assert!(result.created);
    let records: Vec<EvidenceView> = conductor
        .call(&zome, "get_subject_evidence", "TEST-ohe-1".to_string())
        .await;
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].action_hash, result.action_hash);
    assert_eq!(records[0].evidence.sensor_id, "sensor-a");
    assert_eq!(records[0].evidence.evidence_class, "TEST");
    let repeated: CreateEvidenceResult = conductor
        .call(&zome, "create_evidence_idempotent", payload)
        .await;
    assert!(!repeated.created);
    assert_eq!(repeated.action_hash, result.action_hash);
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
    let res: Result<CreateEvidenceResult, _> = conductor
        .call_fallible(&zome, "create_evidence_idempotent", payload)
        .await;
    let error = res.expect_err("REAL evidence must remain persistence-gated");
    assert!(format!("{error:?}").contains("REAL_DATA_PERSISTENCE_GATE_CLOSED"));
}


#[tokio::test(flavor = "multi_thread")]
async fn test_dna_uses_real_test_agents_for_control_plane_authority() {
    let (conductor, authority_cell, _reviewer_cell, rogue_cell, _evidence_cell, _authority, _reviewer, _rogue, _evidence_author) =
        setup_control_plane().await;
    let authority_zome = authority_cell.zome("zome_coordinator");
    let rogue_zome = rogue_cell.zome("zome_coordinator");

    let status: GateStatus = conductor.call(&authority_zome, "get_real_data_gate_status", ()).await;
    assert!(!status.real_data_enabled);
    assert!(status.authority_configured);
    assert_eq!(status.approved_reviewer_count, 2);

    let authorization = serde_json::json!({
        "id": "auth-test-1",
        "subject_id": "ohe-test-1",
        "sensor_id": "sensor-test-1",
        "indicator": "soil_moisture",
        "valid_from": 100_i64,
        "valid_until": 2000_i64,
        "enabled": true
    });
    let _: ActionHash = conductor
        .call(&authority_zome, "create_real_data_authorization", authorization.clone())
        .await;
    let unauthorized: Result<ActionHash, _> = conductor
        .call_fallible(&rogue_zome, "create_real_data_authorization", authorization)
        .await;
    assert!(unauthorized.is_err(), "non-authority agent must not write real-data authorization");
}

#[tokio::test(flavor = "multi_thread")]
async fn calibration_supersession_and_revocation_resolve_fail_closed() {
    let (conductor, authority_cell, _reviewer_cell, rogue_cell, _evidence_cell, ..) =
        setup_control_plane().await;
    let authority_zome = authority_cell.zome("zome_coordinator");
    let rogue_zome = rogue_cell.zome("zome_coordinator");
    let hash_a = "a".repeat(64);
    let hash_b = "b".repeat(64);

    let root = serde_json::json!({
        "id": "cal-root",
        "sensor_id": "sensor-test-1",
        "calibration_hash": hash_a,
        "valid_from": 100_i64,
        "valid_until": 1000_i64,
        "status": "APPROVED",
        "previous_calibration": null
    });
    let root_action: ActionHash = conductor
        .call(&authority_zome, "create_calibration_approval", root.clone())
        .await;

    let unauthorized: Result<ActionHash, _> = conductor
        .call_fallible(&rogue_zome, "create_calibration_approval", root)
        .await;
    assert!(unauthorized.is_err(), "non-authority calibration write must fail");

    let before: CalibrationResolveResult = conductor
        .call(
            &authority_zome,
            "resolve_calibration",
            serde_json::json!({
                "sensor_id": "sensor-test-1",
                "observed_at": 400_i64,
                "calibration_hash": "a".repeat(64)
            }),
        )
        .await;
    assert_eq!(before.status, "ACTIVE");
    assert_eq!(before.action_hash, Some(root_action.clone()));
    assert_eq!(before.calibration_hash.as_deref(), Some("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"));

    let replacement = serde_json::json!({
        "id": "cal-replacement",
        "sensor_id": "sensor-test-1",
        "calibration_hash": hash_b,
        "valid_from": 500_i64,
        "valid_until": 1500_i64,
        "status": "APPROVED",
        "previous_calibration": root_action
    });
    let replacement_action: ActionHash = conductor
        .call(&authority_zome, "create_calibration_approval", replacement)
        .await;

    let old_after: CalibrationResolveResult = conductor
        .call(
            &authority_zome,
            "resolve_calibration",
            serde_json::json!({
                "sensor_id": "sensor-test-1",
                "observed_at": 600_i64,
                "calibration_hash": "a".repeat(64)
            }),
        )
        .await;
    assert_eq!(old_after.status, "NO_ACTIVE");

    let new_after: CalibrationResolveResult = conductor
        .call(
            &authority_zome,
            "resolve_calibration",
            serde_json::json!({
                "sensor_id": "sensor-test-1",
                "observed_at": 600_i64,
                "calibration_hash": "b".repeat(64)
            }),
        )
        .await;
    assert_eq!(new_after.status, "ACTIVE");
    assert_eq!(new_after.action_hash, Some(replacement_action.clone()));

    let revoke = serde_json::json!({
        "id": "cal-revoke",
        "sensor_id": "sensor-test-1",
        "calibration_hash": "b".repeat(64),
        "valid_from": 800_i64,
        "valid_until": 800_i64,
        "status": "REVOKED",
        "previous_calibration": replacement_action
    });
    let _: ActionHash = conductor
        .call(&authority_zome, "create_calibration_approval", revoke)
        .await;

    let revoked: CalibrationResolveResult = conductor
        .call(
            &authority_zome,
            "resolve_calibration",
            serde_json::json!({
                "sensor_id": "sensor-test-1",
                "observed_at": 900_i64,
                "calibration_hash": "b".repeat(64)
            }),
        )
        .await;
    assert_eq!(revoked.status, "NO_ACTIVE");
}

#[tokio::test(flavor = "multi_thread")]
async fn reviewer_attestation_requires_approved_independent_real_test_agent() {
    let (conductor, _authority_cell, reviewer_cell, rogue_cell, evidence_cell, _authority, reviewer, rogue, evidence_author) =
        setup_control_plane().await;
    let reviewer_zome = reviewer_cell.zome("zome_coordinator");
    let rogue_zome = rogue_cell.zome("zome_coordinator");
    let evidence_zome = evidence_cell.zome("zome_coordinator");

    let evidence_payload = serde_json::json!({
        "id": "test:ohe-review:sensor-review:soil_moisture:1900000100",
        "subject_id": "TEST-ohe-review",
        "sensor_id": "sensor-review",
        "indicator": "soil_moisture",
        "evidence_class": "TEST",
        "calibration_hash": null,
        "method_hash": "test-method-hash",
        "data_hash": "test-data-hash",
        "observed_at": 1900000100_i64,
        "confidence": 0.9,
        "missing_data": false,
        "reviewer": null
    });
    let evidence: CreateEvidenceResult = conductor
        .call(&evidence_zome, "create_evidence_idempotent", evidence_payload)
        .await;

    await_consistency([&evidence_cell, &reviewer_cell, &rogue_cell])
        .await
        .unwrap();

    let valid_review = serde_json::json!({
        "id": "review-valid",
        "evidence_action": evidence.action_hash,
        "reviewer": reviewer,
        "decision": "ACCEPT",
        "reviewed_at": 1900000200_i64,
        "lineage_hash": "c".repeat(64)
    });
    let _: ActionHash = conductor
        .call(&reviewer_zome, "create_review_attestation", valid_review)
        .await;

    let self_review = serde_json::json!({
        "id": "review-self",
        "evidence_action": evidence.action_hash,
        "reviewer": evidence_author,
        "decision": "ACCEPT",
        "reviewed_at": 1900000201_i64,
        "lineage_hash": "d".repeat(64)
    });
    let self_result: Result<ActionHash, _> = conductor
        .call_fallible(&evidence_zome, "create_review_attestation", self_review)
        .await;
    assert!(self_result.is_err(), "approved agent must still be unable to self-review own evidence");

    let rogue_review = serde_json::json!({
        "id": "review-rogue",
        "evidence_action": evidence.action_hash,
        "reviewer": rogue,
        "decision": "ACCEPT",
        "reviewed_at": 1900000202_i64,
        "lineage_hash": "e".repeat(64)
    });
    let rogue_result: Result<ActionHash, _> = conductor
        .call_fallible(&rogue_zome, "create_review_attestation", rogue_review)
        .await;
    assert!(rogue_result.is_err(), "unapproved reviewer must fail");
}
