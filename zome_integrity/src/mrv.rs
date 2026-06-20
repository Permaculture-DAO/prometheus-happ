// mrv.rs
// MRV evidence record (Measurement, Reporting, Verification).
//
// Canon (WP v1.1.2 §I, §S MRV Controls): verification is infrastructure, not
// paperwork. No single source is absolute; institutional credibility comes from
// provenance, method hash, data hash, timestamp, confidence, anomaly disclosure
// and disciplined treatment of missing data. This record carries evidence and
// its provenance ONLY — never value, never admissibility (see claim.rs).

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvidenceSource {
    FieldSensor,
    Laboratory,
    SoilTest,
    WaterTest,
    Satellite,
    Drone,
    GeoPhoto,
    BiodiversityRecord,
    OperationalLog,
    ThirdPartyAudit,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MrvEvidence {
    pub id: String,
    /// The OHE / subject this evidence pertains to.
    pub subject_id: String,
    /// What is being measured (e.g. "soil_organic_matter").
    pub indicator: String,
    pub source: EvidenceSource,
    /// Provenance: hash of the method/protocol used.
    pub method_hash: String,
    /// Provenance: hash of the raw data.
    pub data_hash: String,
    /// Observation time (unix seconds).
    pub observed_at: i64,
    /// Conservative confidence in [0.0, 1.0]. Single input, never a gate by itself.
    pub confidence: f64,
    /// Disciplined treatment of missing data: explicit, never silently imputed.
    pub missing_data: bool,
    /// Reviewer trail (optional at capture; required before admissibility).
    pub reviewer: Option<String>,
}

impl MrvEvidence {
    pub fn is_valid(&self) -> bool {
        !self.id.is_empty()
            && !self.subject_id.is_empty()
            && !self.indicator.is_empty()
            && !self.method_hash.is_empty()
            && !self.data_hash.is_empty()
            && self.observed_at > 0
            && (0.0..=1.0).contains(&self.confidence)
    }

    /// Unique event-identity tuple for no-double-counting (canon §S).
    pub fn identity(&self) -> (String, String, i64) {
        (self.subject_id.clone(), self.indicator.clone(), self.observed_at)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base() -> MrvEvidence {
        MrvEvidence {
            id: "ev-1".into(),
            subject_id: "ohe-1".into(),
            indicator: "soil_organic_matter".into(),
            source: EvidenceSource::SoilTest,
            method_hash: "m-hash-abc".into(),
            data_hash: "d-hash-xyz".into(),
            observed_at: 1_900_000_000,
            confidence: 0.7,
            missing_data: false,
            reviewer: Some("reviewer-1".into()),
        }
    }

    #[test]
    fn valid_evidence() {
        assert!(base().is_valid());
    }

    #[test]
    fn missing_provenance_fails() {
        let e = MrvEvidence { method_hash: "".into(), ..base() };
        assert!(!e.is_valid(), "evidence without method hash must fail");
        let e2 = MrvEvidence { data_hash: "".into(), ..base() };
        assert!(!e2.is_valid(), "evidence without data hash must fail");
    }

    #[test]
    fn confidence_must_be_in_unit_interval() {
        assert!(!MrvEvidence { confidence: 1.5, ..base() }.is_valid());
        assert!(!MrvEvidence { confidence: -0.1, ..base() }.is_valid());
        assert!(MrvEvidence { confidence: 0.0, ..base() }.is_valid());
        assert!(MrvEvidence { confidence: 1.0, ..base() }.is_valid());
    }

    #[test]
    fn identity_tuple_is_subject_indicator_time() {
        let e = base();
        assert_eq!(
            e.identity(),
            ("ohe-1".to_string(), "soil_organic_matter".to_string(), 1_900_000_000)
        );
    }
}
