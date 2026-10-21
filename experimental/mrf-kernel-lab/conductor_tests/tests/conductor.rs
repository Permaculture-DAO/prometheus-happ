//! One in-process TEST-only conductor instance; five gate assertions.
//! Isolated packaged DNA, independent of signed "hearth" production DNA.
//! Keep the costly SweetConductor initialization to ONE invocation.
use holo_hash::ActionHash;
use holochain::sweettest::*;
use std::path::PathBuf;

async fn setup() -> (SweetConductor, SweetZome) {
    let dna_path=PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..").join("experimental_dna")
        .join("prometheus_mrf_synthetic_lab.dna");
    let dna=SweetDnaFile::from_bundle(&dna_path).await.expect("load exact LAB DNA");
    let mut conductor=SweetConductor::from_standard_config().await;
    let app=conductor.setup_app("synthetic-mrf", &[dna]).await.expect("setup LAB app");
    let cell=app.cells()[0].clone();
    (conductor,cell.zome("prometheus_mrf_coordinator_lab"))
}
fn toy() -> serde_json::Value {
    serde_json::json!({
        "subject_id":"TEST-site-01", "evidence_class":"TEST",
        "model_sha256":"e074715a26129c8f00e828774eadc71ef33f94a620c0dcd0e00216b6156bda5f",
        "input_i64":[250,90], "output_i64":[8,-2],
        "authority":"model_conformity_only", "financial_admission":false
    })
}
#[tokio::test(flavor="multi_thread")]
async fn synthetic_conductor_valid_and_adversarial_gates() {
    let (conductor,zome)=setup().await;
    // Gate 1: valid TEST synthetic receipt passes.
    let _:ActionHash=conductor.call(&zome,"create_synthetic_receipt",toy()).await;
    println!("GATE_1_TEST_VALID=PASS");
    // Gate 2: forged output rejected by the integrity callback.
    let mut x=toy(); x["output_i64"]=serde_json::json!([8,17]);
    let res:Result<ActionHash,_>=conductor.call_fallible(&zome,"create_synthetic_receipt",x).await;
    assert!(res.is_err(),"GATE_2 forged result must fail");
    println!("GATE_2_OUTPUT_FORGERY_REJECTED=PASS");
    // Gate 3: REAL persistence is prohibited even for plausible input.
    let mut x=toy(); x["evidence_class"]=serde_json::json!("REAL");
    let res:Result<ActionHash,_>=conductor.call_fallible(&zome,"create_synthetic_receipt",x).await;
    assert!(res.is_err(),"GATE_3 REAL must fail");
    println!("GATE_3_REAL_REJECTED=PASS");
    // Gate 4: claims under unapproved model digest are rejected.
    let mut x=toy(); x["model_sha256"]=serde_json::json!("bad-digest");
    let res:Result<ActionHash,_>=conductor.call_fallible(&zome,"create_synthetic_receipt",x).await;
    assert!(res.is_err(),"GATE_4 model hash mismatch must fail");
    println!("GATE_4_MODEL_SUBSTITUTION_REJECTED=PASS");
    // Gate 5: a model receipt cannot authorize finance/rights.
    let mut x=toy(); x["financial_admission"]=serde_json::json!(true);
    let res:Result<ActionHash,_>=conductor.call_fallible(&zome,"create_synthetic_receipt",x).await;
    assert!(res.is_err(),"GATE_5 financial authority must fail");
    println!("GATE_5_FINANCIAL_PROMOTION_REJECTED=PASS");
}