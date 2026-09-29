// P2.2B conductor integration tests.
// Uses an in-process Holochain SweetConductor with the exact wasm artifacts built
// from this repository branch. Synthetic TEST data only.

use holochain::conductor::{api::error::ConductorApiError, CellError};
use holochain::core::workflow::WorkflowError;
use holochain::sweettest::*;
use holo_hash::ActionHash;
use holochain_types::prelude::*;
use std::path::PathBuf;

// Query rows contain MessagePack binary hashes, so the outer response must be typed.
#[derive(Debug, serde::Deserialize)]
struct PackageRecord {
    action_hash: ActionHash,
    package: serde_json::Value,
}

#[derive(Debug, serde::Deserialize)]
struct ReviewRecord {
    action_hash: ActionHash,
    review: serde_json::Value,
}

#[derive(Debug, serde::Deserialize)]
struct DecisionRecord {
    action_hash: ActionHash,
    decision: serde_json::Value,
}

async fn dna_from_local_wasm() -> DnaFile {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
    let target = root.join("target").join("wasm32-unknown-unknown").join("debug");

    let integrity_wasm = DnaWasm::from(
        std::fs::read(target.join("zome_integrity.wasm"))
            .expect("zome_integrity.wasm must be built before integration tests"),
    );
    let coordinator_wasm = DnaWasm::from(
        std::fs::read(target.join("zome_coordinator.wasm"))
            .expect("zome_coordinator.wasm must be built before integration tests"),
    );

    let integrity_name = ZomeName::from("zome_integrity");
    let coordinator_name = ZomeName::from("zome_coordinator");

    let integrity_hash = WasmHash::with_data(&integrity_wasm).await;
    let coordinator_hash = WasmHash::with_data(&coordinator_wasm).await;

    let integrity = IntegrityZome::new(
        integrity_name.clone(),
        ZomeDef::Wasm(WasmZome {
            wasm_hash: integrity_hash,
            dependencies: vec![],
        })
        .into(),
    );

    let coordinator = CoordinatorZome::new(
        coordinator_name,
        ZomeDef::Wasm(WasmZome {
            wasm_hash: coordinator_hash,
            dependencies: vec![integrity_name],
        })
        .into(),
    );

    SweetDnaFile::unique_from_zomes(
        vec![integrity],
        vec![coordinator],
        vec![integrity_wasm, coordinator_wasm],
    )
    .await
    .0
}

async fn setup() -> (SweetConductor, SweetZome) {
    let dna = dna_from_local_wasm().await;
    let mut conductor = SweetConductor::from_standard_config().await;
    let app = conductor.setup_app("prometheus-p2-2b", &[dna]).await.unwrap();
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
    assert!(res.is_err(), "Active-without-baseline OHE must be rejected");
}

#[tokio::test(flavor = "multi_thread")]
async fn p2_2b_evidence_spine_reconstructs_on_live_conductor() {
    let (conductor, zome) = setup().await;
    let now = 1_900_000_000_i64;
    let subject = "TEST-ohe-p2-2b";
    let claim_uid = "prometheus.runtime.evaluation_not_certification";
    let package_id = "ep-TEST-p2-2b";
    let review_id = "rev-TEST-p2-2b";
    let decision_id = "adm-TEST-p2-2b";

    let place: ActionHash = conductor.call(&zome, "create_place_context", serde_json::json!({
        "id":"pc-entry-TEST","version":"pc-TEST-v0.1","site_id":subject,
        "geology_context":"TEST","hydrology_context":"TEST","climate_context":"TEST",
        "land_use_history":"TEST","stewardship_rights_access":"TEST",
        "uncertainty_note":"synthetic TEST only"
    })).await;
    assert!(!place.to_string().is_empty());

    let boundary: ActionHash = conductor.call(&zome, "create_system_boundary", serde_json::json!({
        "id":"sb-entry-TEST","version":"sb-TEST-v0.1","site_id":subject,
        "effective_from":now,"spatial_definition":"TEST polygon",
        "included_processes":["TEST-flow"],"excluded_processes":["real-field-performance"],
        "external_dependencies":["TEST-conductor"],"rationale":"synthetic lineage proof"
    })).await;
    assert!(!boundary.to_string().is_empty());

    let observation: ActionHash = conductor.call(&zome, "create_observation", serde_json::json!({
        "id":"obs-TEST-p2-2b","subject_id":subject,
        "place_context_version":"pc-TEST-v0.1","system_boundary_version":"sb-TEST-v0.1",
        "method_id":"method-TEST-v0.1","observed_at":now,"source_class":"operational_log",
        "observer_or_instrument_id":"instrument-TEST","management_state_version":"ms-TEST-v0.1",
        "raw_evidence_refs":["sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"],
        "disturbance_id":null,"uncertainty_note":"synthetic TEST"
    })).await;
    assert!(!observation.to_string().is_empty());

    let package: ActionHash = conductor.call(&zome, "create_evidence_package", serde_json::json!({
        "id":package_id,"subject_id":subject,"claim_uids":[claim_uid],
        "place_context_version":"pc-TEST-v0.1","system_boundary_version":"sb-TEST-v0.1",
        "observation_refs":["obs-TEST-p2-2b"],"method_refs":["method-TEST-v0.1"],
        "raw_data_hashes":["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"],
        "transformed_data_hashes":[],
        "package_hash":"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
        "claims_registry_hash":"cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
        "missing_data_statement":"none in synthetic TEST",
        "adverse_event_statement":"none in synthetic TEST","created_at":now
    })).await;
    assert!(!package.to_string().is_empty());

    let packages: Vec<PackageRecord> = conductor.call(&zome, "get_subject_evidence_packages", subject).await;
    assert!(packages.iter().any(|r| r.action_hash == package && r.package["id"] == package_id));

    let review: ActionHash = conductor.call(&zome, "create_review_attestation", serde_json::json!({
        "id":review_id,"reviewer_id":"reviewer-TEST","scope":"synthetic digital lineage only",
        "coi_status":"independent","subject_refs":[package_id],"decision":"pass",
        "limitations":"not ecological/scientific/legal assurance","independent_for_scope":true,
        "reviewed_at":now
    })).await;
    assert!(!review.to_string().is_empty());

    let reviews: Vec<ReviewRecord> = conductor.call(&zome, "get_package_reviews", package_id).await;
    assert!(reviews.iter().any(|r| r.action_hash == review && r.review["id"] == review_id));

    let decision: ActionHash = conductor.call(&zome, "create_admissibility_decision", serde_json::json!({
        "id":decision_id,"subject_id":subject,"claim_uid":claim_uid,
        "evidence_package_refs":[package_id],"review_attestation_refs":[review_id],
        "legal_gate":false,"mrv_gate":true,"confidence":0.90,"decision":"blocked",
        "blockers":["legal gate not established"],"decided_at":now,
        "authority_boundary":"admissibility_only_no_value"
    })).await;
    assert!(!decision.to_string().is_empty());

    let decisions: Vec<DecisionRecord> = conductor.call(&zome, "get_claim_admissibility_decisions", claim_uid).await;
    assert!(decisions.iter().any(|r|
        r.action_hash == decision &&
        r.decision["id"] == decision_id &&
        r.decision["decision"] == "blocked" &&
        r.decision["authority_boundary"] == "admissibility_only_no_value"
    ));

    let bad_review: Result<ActionHash, _> = conductor.call_fallible(&zome, "create_review_attestation", serde_json::json!({
        "id":"rev-TEST-conflicted","reviewer_id":"reviewer-conflicted",
        "scope":"synthetic digital lineage only","coi_status":"conflicted","subject_refs":[package_id],
        "decision":"pass","limitations":"must be rejected","independent_for_scope":false,
        "reviewed_at":now
    })).await;
    let error = bad_review.expect_err("conflicted reviewer pass must be rejected");
    assert!(
        matches!(
            &error,
            ConductorApiError::CellError(CellError::WorkflowError(workflow))
                if matches!(workflow.as_ref(), WorkflowError::SourceChainError(_))
        ),
        "expected a source-chain validation failure, got: {error:?}"
    );
    assert!(
        format!("{error:?}").contains("ReviewAttestation violates COI/scope invariants"),
        "expected the review validation rejection, got: {error:?}"
    );
    let reviews: Vec<ReviewRecord> = conductor.call(&zome, "get_package_reviews", package_id).await;
    assert!(reviews.iter().all(|r| r.review["id"] != "rev-TEST-conflicted"));

    println!("{}", serde_json::json!({
        "schema_version":"p2.2b-sweetconductor-0.1",
        "data_class":"synthetic_TEST",
        "subject_id":subject,
        "claim_uid":claim_uid,
        "action_hashes":{
            "place_context":place.to_string(),
            "system_boundary":boundary.to_string(),
            "observation":observation.to_string(),
            "evidence_package":package.to_string(),
            "review_attestation":review.to_string(),
            "admissibility_decision":decision.to_string()
        },
        "reconstruction":{
            "subject_to_package":true,
            "package_to_review":true,
            "claim_to_decision":true
        },
        "negative_test":{
            "conflicted_reviewer_pass_rejected":true
        },
        "decision":"blocked",
        "authority_boundary":"admissibility_only_no_value",
        "ecological_truth":false,
        "scientific_validation":false,
        "legal_admission":false,
        "certification":false,
        "pru_value":false
    }));
}
