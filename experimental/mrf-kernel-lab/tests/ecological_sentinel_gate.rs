//! TEST-only ecological sentinel: anti-compensatory and provenance checks.
//! No ecological measurements or site claims are represented. This is a
//! falsification fixture, NOT a production MRV/admissibility methodology.
#[derive(Clone)]
struct Sentinel {
    site: &'static str,
    baseline_method: &'static str,
    followup_method: &'static str,
    baseline: [Option<i64>;5],  // soil, water, biodiversity, yield, external inputs
    followup: [Option<i64>;5],
    independently_observed: bool,
    comparator_available: bool,
    duplicate_evidence_hash: bool,
}
#[derive(Debug,PartialEq,Eq)]
enum Decision { ReviewCandidate, Hold(&'static str), EcologicalRegression(&'static str) }

fn evaluate(s:&Sentinel)->Decision {
    if !s.site.starts_with("TEST-") || !s.independently_observed || !s.comparator_available {
        return Decision::Hold("site/provenance/comparator missing");
    }
    if s.duplicate_evidence_hash {
        return Decision::Hold("replayed evidence");
    }
    if s.baseline_method != s.followup_method {
        return Decision::Hold("unreviewed method change");
    }
    let (Some(soil0), Some(water0), Some(bio0), Some(yield0), Some(inputs0)) =
        (s.baseline[0],s.baseline[1],s.baseline[2],s.baseline[3],s.baseline[4])
    else {return Decision::Hold("incomplete baseline")};
    let (Some(soil1), Some(water1), Some(bio1), Some(yield1), Some(inputs1)) =
        (s.followup[0],s.followup[1],s.followup[2],s.followup[3],s.followup[4])
    else {return Decision::Hold("incomplete followup")};
    if soil1<soil0 {return Decision::EcologicalRegression("soil");}
    if water1<water0 {return Decision::EcologicalRegression("water");}
    if bio1<bio0 {return Decision::EcologicalRegression("biodiversity");}
    if yield1<yield0 {return Decision::EcologicalRegression("yield");}
    if inputs1>inputs0 {return Decision::EcologicalRegression("external inputs");}
    // Even passing the synthetic sentinel cannot establish causal improvement.
    Decision::ReviewCandidate
}
fn trial()->Sentinel {
    Sentinel {
        site:"TEST-SICILY",baseline_method:"TEST-method-v1",followup_method:"TEST-method-v1",
        baseline:[Some(100),Some(100),Some(100),Some(100),Some(100)],
        followup:[Some(110),Some(110),Some(110),Some(110),Some(90)],
        independently_observed:true,comparator_available:true,
        duplicate_evidence_hash:false
    }
}
#[test] fn all_positive_is_still_only_candidate() {
    assert_eq!(evaluate(&trial()),Decision::ReviewCandidate);
}
#[test] fn soil_gain_does_not_offset_water_loss() {
    let mut x=trial();x.followup[1]=Some(95);
    assert_eq!(evaluate(&x),Decision::EcologicalRegression("water"));
}
#[test] fn yield_gain_does_not_offset_biodiversity_loss() {
    let mut x=trial();x.followup[2]=Some(99);
    assert_eq!(evaluate(&x),Decision::EcologicalRegression("biodiversity"));
}
#[test] fn increasing_external_inputs_trigger_regression() {
    let mut x=trial();x.followup[4]=Some(101);
    assert_eq!(evaluate(&x),Decision::EcologicalRegression("external inputs"));
}
#[test] fn model_predictions_without_independent_observation_hold() {
    let mut x=trial();x.independently_observed=false;
    assert_eq!(evaluate(&x),Decision::Hold("site/provenance/comparator missing"));
}
#[test] fn missing_baseline_never_interpreted_as_zero() {
    let mut x=trial();x.baseline[3]=None;
    assert_eq!(evaluate(&x),Decision::Hold("incomplete baseline"));
}
#[test] fn missing_comparator_requires_review() {
    let mut x=trial();x.comparator_available=false;
    assert_eq!(evaluate(&x),Decision::Hold("site/provenance/comparator missing"));
}
#[test] fn method_drift_and_duplicate_evidence_fail_closed() {
    let mut x=trial();x.followup_method="TEST-method-v2";
    assert_eq!(evaluate(&x),Decision::Hold("unreviewed method change"));
    x.followup_method="TEST-method-v1";x.duplicate_evidence_hash=true;
    assert_eq!(evaluate(&x),Decision::Hold("replayed evidence"));
}
