// Conductor integration test (sweettest, in-process — no hc sandbox TTY/cap dance).
// Asserts the integrity `validate` callback runs live: a valid OHE is created,
// and an invalid one (Active without baseline) is rejected.

use holochain::sweettest::*;
use holo_hash::ActionHash;
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
    let res: Result<ActionHash, _> = conductor
        .call_fallible(&zome, "create_ohe", payload)
        .await;
    assert!(
        res.is_err(),
        "Active-without-baseline OHE must be rejected by the integrity validate callback"
    );
}
