//! Minimal networking smoke: two distinct SweetConductors, same TEST DNA.
//! This tests peer-info discovery only. No DHT replication is claimed.
use holochain::sweettest::*;
use std::path::PathBuf;

#[tokio::test(flavor="multi_thread")]
async fn two_test_conductors_exchange_peer_info() {
    let path=PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..").join("experimental_dna")
        .join("prometheus_mrf_synthetic_lab.dna");
    let dna=SweetDnaFile::from_bundle(&path).await.expect("exact TEST DNA");
    let mut one=SweetConductor::from_standard_config().await;
    let mut two=SweetConductor::from_standard_config().await;
    let _app_one=one.setup_app("TEST-discovery-one",&[dna.clone()]).await.expect("A");
    let _app_two=two.setup_app("TEST-discovery-two",&[dna]).await.expect("B");
    let exchanged=SweetConductor::exchange_peer_info([&one,&two]).await;
    assert!(exchanged, "both TEST conductors must discover peer info");
    println!("TWO_CONDUCTORS_PEER_INFO_DISCOVERY=PASS");
}
