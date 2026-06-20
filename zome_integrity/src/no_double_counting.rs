// no_double_counting.rs
// No-double-counting invariant (canon WP v1.1.2 §S MRV Controls):
// "unique event-identity tuples and duplicate-claim detection".
//
// Pure function over a set of MRV evidence: rejects duplicate identity tuples
// (subject_id, indicator, observed_at). Used by the integrity validation layer
// before any evidence is admitted.

use crate::mrv::MrvEvidence;
// BTreeSet (not HashSet): no RandomState/getrandom seed, so it is safe inside the
// zome wasm where host entropy is unsupported.
use std::collections::BTreeSet;

#[derive(Debug, PartialEq, Eq)]
pub enum DoubleCountError {
    Duplicate { subject_id: String, indicator: String, observed_at: i64 },
}

/// Returns Ok(()) if every evidence identity tuple is unique; otherwise the first
/// duplicate found. O(n) with a hash set.
pub fn check_no_double_counting(evidence: &[MrvEvidence]) -> Result<(), DoubleCountError> {
    let mut seen: BTreeSet<(String, String, i64)> = BTreeSet::new();
    for e in evidence {
        let id = e.identity();
        if !seen.insert(id.clone()) {
            return Err(DoubleCountError::Duplicate {
                subject_id: id.0,
                indicator: id.1,
                observed_at: id.2,
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mrv::{EvidenceSource, MrvEvidence};

    fn ev(id: &str, subject: &str, indicator: &str, t: i64) -> MrvEvidence {
        MrvEvidence {
            id: id.into(),
            subject_id: subject.into(),
            indicator: indicator.into(),
            source: EvidenceSource::SoilTest,
            method_hash: "m".into(),
            data_hash: "d".into(),
            observed_at: t,
            confidence: 0.6,
            missing_data: false,
            reviewer: None,
        }
    }

    #[test]
    fn unique_evidence_passes() {
        let set = vec![
            ev("e1", "ohe-1", "soil_organic_matter", 100),
            ev("e2", "ohe-1", "soil_organic_matter", 200), // different time
            ev("e3", "ohe-1", "water_infiltration", 100),  // different indicator
            ev("e4", "ohe-2", "soil_organic_matter", 100),  // different subject
        ];
        assert_eq!(check_no_double_counting(&set), Ok(()));
    }

    #[test]
    fn duplicate_identity_is_rejected() {
        // Same subject+indicator+time but different evidence id = double counting.
        let set = vec![
            ev("e1", "ohe-1", "soil_organic_matter", 100),
            ev("e2", "ohe-1", "soil_organic_matter", 100),
        ];
        assert_eq!(
            check_no_double_counting(&set),
            Err(DoubleCountError::Duplicate {
                subject_id: "ohe-1".into(),
                indicator: "soil_organic_matter".into(),
                observed_at: 100,
            })
        );
    }
}
