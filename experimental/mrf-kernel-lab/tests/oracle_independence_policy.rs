//! Policy-only oracle independence sandbox, SYNTHETIC TEST data.
//! NOTE: This does NOT verify signatures or imply scientific origin/truth.
//! Cryptographic signature validation is a separate mandatory gate.
use std::collections::{BTreeMap, BTreeSet};
#[derive(Debug,PartialEq,Eq)]
enum State { CandidateAgreement, Unknown, Disagreement, Review }
#[derive(Clone)]
struct Attestation {
    operator: &'static str, // independently controlled organisation, not device ID
    device: &'static str,
    sample: &'static str,
    payload_hash: &'static str,
    calibration: bool,
    fresh: bool,
    signature_verified: bool,
    result: i64,
}
fn assess(items:&[Attestation], quorum:usize) -> State {
    if quorum<2 || items.is_empty() { return State::Unknown; }
    if items.iter().any(|a| !a.signature_verified || !a.calibration || !a.fresh) {
        return State::Unknown;
    }
    // A field measurement identified only by a shared sample hash is not
    // independently acquired merely because distinct devices signed it.
    let mut operators=BTreeSet::new();
    let mut devices=BTreeSet::new();
    let mut samples=BTreeSet::new();
    let mut payloads=BTreeSet::new();
    let mut values=BTreeMap::<i64,usize>::new();
    for a in items {
        if !operators.insert(a.operator) || !devices.insert(a.device) ||
           !samples.insert(a.sample) || !payloads.insert(a.payload_hash) {
            return State::Review;
        }
        *values.entry(a.result).or_default() += 1;
    }
    if operators.len()<quorum { return State::Unknown; }
    if values.len()>1 { return State::Disagreement; }
    State::CandidateAgreement
}
fn test_attestations() -> Vec<Attestation> {
    vec![
        Attestation{operator:"A",device:"sensor-A",sample:"sample-a",payload_hash:"hash-a",calibration:true,fresh:true,signature_verified:true,result:7},
        Attestation{operator:"B",device:"sensor-B",sample:"sample-b",payload_hash:"hash-b",calibration:true,fresh:true,signature_verified:true,result:7},
        Attestation{operator:"C",device:"sensor-C",sample:"sample-c",payload_hash:"hash-c",calibration:true,fresh:true,signature_verified:true,result:7},
    ]
}
#[test] fn independent_candidate_only() {assert_eq!(assess(&test_attestations(),3),State::CandidateAgreement);}
#[test] fn same_operator_control_collusion() {
 let mut a=test_attestations();a[1].operator="A";
 assert_eq!(assess(&a,3),State::Review);
}
#[test] fn duplicate_evidence_hash() {
 let mut a=test_attestations();a[1].payload_hash="hash-a";
 assert_eq!(assess(&a,3),State::Review);
}
#[test] fn clone_sample() {
 let mut a=test_attestations();a[1].sample="sample-a";
 assert_eq!(assess(&a,3),State::Review);
}
#[test] fn missing_calibration_abstains() {
 let mut a=test_attestations();a[2].calibration=false;
 assert_eq!(assess(&a,3),State::Unknown);
}
#[test] fn stale_or_revoked_abstains() {
 let mut a=test_attestations();a[2].fresh=false;
 assert_eq!(assess(&a,3),State::Unknown);
 a[2].fresh=true;a[2].signature_verified=false;
 assert_eq!(assess(&a,3),State::Unknown);
}
#[test] fn independently_disagreeing_readings_disagree() {
 let mut a=test_attestations();a[2].result=8;
 assert_eq!(assess(&a,3),State::Disagreement);
}
#[test] fn insufficient_distinct_operators_abstains() {
 let mut a=test_attestations();a.pop();
 assert_eq!(assess(&a,3),State::Unknown);
}
