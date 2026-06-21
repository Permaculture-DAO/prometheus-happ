// admissibility.rs
// The spine: the binary gate that stands BETWEEN evidence and value.
//
// Canon (WP v1.1.2 — bounded valuation, controlling firewall):
//   V_PRU_adj = V_base · LegalGate · MRVGate · (1 + U_max·g(Z)) · Conf_total
//   value = 0 if any of V_base, LegalGate, MRVGate, Conf_total = 0.
// This module computes ONLY the admissibility precondition (LegalGate ∧ MRVGate,
// with a conservative confidence floor). It DELIBERATELY computes NO value: there
// is no V_base, no U_max, no g(Z), no PRU here. Value stays prose-only and
// suspended at zero until governance-approved off-runtime analysis. The runtime
// answers one question: is this evidence admissible? — never "how much".

use crate::mrv::MrvEvidence;
use crate::no_double_counting::check_no_double_counting;

/// Conservative confidence aggregation: the weakest evidence caps the set
/// (min), so one low-confidence record holds the whole subject back.
pub fn conservative_confidence(evidence: &[MrvEvidence]) -> f64 {
    if evidence.is_empty() {
        return 0.0;
    }
    evidence.iter().map(|e| e.confidence).fold(1.0_f64, f64::min)
}

/// MRV gate (binary). True only if there is evidence, every record is valid,
/// there is no double counting, reviewers are present when required, and the
/// conservative confidence clears the governance-versioned threshold. A single
/// number is one input, never the gate by itself — it is one of several ANDed
/// conditions here.
pub fn mrv_gate(evidence: &[MrvEvidence], confidence_threshold: f64, require_reviewer: bool) -> bool {
    if evidence.is_empty() {
        return false;
    }
    if !evidence.iter().all(|e| e.is_valid()) {
        return false;
    }
    if check_no_double_counting(evidence).is_err() {
        return false;
    }
    if require_reviewer && !evidence.iter().all(|e| e.reviewer.is_some()) {
        return false;
    }
    conservative_confidence(evidence) >= confidence_threshold
}

/// Admissibility = the lowest gate controls. LegalGate ∧ MRVGate, both binary.
/// Returns whether evidence may PROCEED to off-runtime value-analysis — not a value.
pub fn admissible(
    legal_gate: bool,
    evidence: &[MrvEvidence],
    confidence_threshold: f64,
    require_reviewer: bool,
) -> bool {
    legal_gate && mrv_gate(evidence, confidence_threshold, require_reviewer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mrv::{EvidenceSource, MrvEvidence};

    fn ev(id: &str, indicator: &str, t: i64, conf: f64, reviewer: Option<&str>) -> MrvEvidence {
        MrvEvidence {
            id: id.into(),
            subject_id: "ohe-1".into(),
            indicator: indicator.into(),
            source: EvidenceSource::SoilTest,
            method_hash: "m".into(),
            data_hash: "d".into(),
            observed_at: t,
            confidence: conf,
            missing_data: false,
            reviewer: reviewer.map(|s| s.into()),
        }
    }

    fn good_set() -> Vec<MrvEvidence> {
        vec![
            ev("e1", "soil_organic_matter", 100, 0.9, Some("r1")),
            ev("e2", "water_infiltration", 100, 0.88, Some("r1")),
        ]
    }

    #[test]
    fn admissible_only_when_all_gates_pass() {
        assert!(admissible(true, &good_set(), 0.85, true), "all gates pass → admissible");
        assert!(!admissible(false, &good_set(), 0.85, true), "legal gate false → not admissible");
    }

    #[test]
    fn low_confidence_blocks() {
        let mut s = good_set();
        s.push(ev("e3", "biomass", 100, 0.5, Some("r1"))); // weakest caps the set
        assert!(!admissible(true, &s, 0.85, true), "conservative confidence below threshold → blocked");
    }

    #[test]
    fn double_counting_blocks() {
        let dup = vec![
            ev("e1", "soil_organic_matter", 100, 0.9, Some("r1")),
            ev("e2", "soil_organic_matter", 100, 0.9, Some("r1")), // same subject+indicator+time
        ];
        assert!(!admissible(true, &dup, 0.85, true), "double counting → blocked");
    }

    #[test]
    fn missing_reviewer_blocks_when_required() {
        let s = vec![ev("e1", "soil_organic_matter", 100, 0.9, None)];
        assert!(!admissible(true, &s, 0.85, true), "no reviewer (required) → blocked");
        assert!(admissible(true, &s, 0.85, false), "reviewer not required → may pass");
    }

    #[test]
    fn empty_evidence_is_never_admissible() {
        assert!(!admissible(true, &[], 0.85, true));
        assert_eq!(conservative_confidence(&[]), 0.0);
    }

    #[test]
    fn no_value_is_produced() {
        // The spine returns only booleans. There is no value-bearing function to call.
        let a: bool = admissible(true, &good_set(), 0.85, true);
        assert!(a == true || a == false);
    }
}
