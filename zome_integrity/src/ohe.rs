// ohe.rs
// One Human Ecosystem (OHE) domain entity.
//
// Canon boundary (WP v1.1.2 §H, Public-Land OHE Protocol): an OHE deployment
// creates NO land, harvesting, governance, data, ecological-credit, PRU, RAP,
// revenue or financial rights. This type records operational status only; it
// never carries value and never asserts admissibility. Rights arise solely from
// valid legal instruments, modelled elsewhere.

use crate::valueflows::ids::AgentId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OheStatus {
    /// Declared, not yet baselined.
    Proposed,
    /// Baseline evidence recorded (precondition for any later indicator work).
    BaselineRecorded,
    /// Active stewardship in progress.
    Active,
    /// Terminated or transitioned per a defined pathway.
    Terminated,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ohe {
    pub id: String,
    /// Assigned steward (required: no OHE without assigned stewardship).
    pub steward: AgentId,
    /// Free-text site / location reference (geo-reference handled at evidence layer).
    pub site: String,
    pub status: OheStatus,
    /// Whether baseline evidence exists. Must be true once status is Active.
    pub baseline_recorded: bool,
}

impl Ohe {
    pub fn is_valid(&self) -> bool {
        if self.id.is_empty() || self.steward.0.is_empty() || self.site.is_empty() {
            return false;
        }
        // Non-bypassable sequence: an Active OHE must have baseline evidence,
        // and a BaselineRecorded status must agree with the flag.
        match self.status {
            OheStatus::Active => self.baseline_recorded,
            OheStatus::BaselineRecorded => self.baseline_recorded,
            OheStatus::Proposed => !self.baseline_recorded,
            OheStatus::Terminated => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn steward() -> AgentId {
        AgentId("agent-steward-1".into())
    }

    #[test]
    fn valid_proposed_ohe() {
        let o = Ohe {
            id: "ohe-1".into(),
            steward: steward(),
            site: "Mediterranean temperate plot A".into(),
            status: OheStatus::Proposed,
            baseline_recorded: false,
        };
        assert!(o.is_valid());
    }

    #[test]
    fn active_requires_baseline() {
        let o = Ohe {
            id: "ohe-2".into(),
            steward: steward(),
            site: "plot B".into(),
            status: OheStatus::Active,
            baseline_recorded: false,
        };
        assert!(!o.is_valid(), "Active OHE without baseline must fail");

        let ok = Ohe { baseline_recorded: true, ..o };
        assert!(ok.is_valid());
    }

    #[test]
    fn missing_steward_or_site_fails() {
        let o = Ohe {
            id: "ohe-3".into(),
            steward: AgentId("".into()),
            site: "plot C".into(),
            status: OheStatus::Proposed,
            baseline_recorded: false,
        };
        assert!(!o.is_valid(), "OHE without steward must fail");

        let o2 = Ohe {
            id: "ohe-4".into(),
            steward: steward(),
            site: "".into(),
            status: OheStatus::Proposed,
            baseline_recorded: false,
        };
        assert!(!o2.is_valid(), "OHE without site must fail");
    }
}
