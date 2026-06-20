// claim.rs
// Claim + claim-status + binary admissibility gate.
//
// Canon (WP v1.1.2 §X.W Claims Register, Controlling Firewall, Methodology):
//   - Every material claim carries a status.
//   - THE LOWEST UNRESOLVED GATE CONTROLS THE PERMITTED CLAIM.
//   - Gates are BINARY. No PRU value is ever computed in the runtime: this module
//     deliberately has no value field and no valuation function. Value lives only
//     in the prose canon's bounded expression and is suspended at zero for capital
//     purposes until governance-versioned gates clear. The runtime decides only
//     admissibility (yes/no), never amount.

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ClaimStatus {
    /// Design-level only.
    Architectural,
    /// Method defined, not yet observed.
    Methodological,
    /// Stated as a hypothesis, pilot-required.
    Hypothesis,
    /// Observed in a pilot (not yet externally reviewed).
    PilotObserved,
    /// Reviewed by an independent third party.
    ThirdPartyReviewed,
    /// Admitted under a legal instrument/opinion.
    LegallyAdmitted,
    /// Admitted by an actual market transaction.
    MarketAdmitted,
}

/// Binary admissibility gates (canon: gates ∈ {0,1}, never a soft score).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmissibilityGate {
    pub legal_gate: bool,
    pub mrv_gate: bool,
}

impl AdmissibilityGate {
    /// The lowest unresolved gate controls: admissible only if ALL gates pass.
    pub fn admissible(&self) -> bool {
        self.legal_gate && self.mrv_gate
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Claim {
    pub id: String,
    pub statement: String,
    pub status: ClaimStatus,
    /// Whether this claim is presented to investors/market.
    pub investor_facing: bool,
}

impl Claim {
    pub fn is_valid(&self) -> bool {
        !self.id.is_empty() && !self.statement.is_empty()
    }

    /// Anti-overclaiming discipline: an investor-facing claim may not rest on
    /// merely architectural, methodological or hypothesis status — it must be at
    /// least pilot-observed. Returns false if the claim overstates its evidence.
    pub fn is_disciplined(&self) -> bool {
        if !self.is_valid() {
            return false;
        }
        if self.investor_facing {
            return self.status >= ClaimStatus::PilotObserved;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gate_is_binary_and_lowest_controls() {
        assert!(AdmissibilityGate { legal_gate: true, mrv_gate: true }.admissible());
        assert!(!AdmissibilityGate { legal_gate: true, mrv_gate: false }.admissible());
        assert!(!AdmissibilityGate { legal_gate: false, mrv_gate: true }.admissible());
        assert!(!AdmissibilityGate { legal_gate: false, mrv_gate: false }.admissible());
    }

    #[test]
    fn investor_claim_cannot_rest_on_hypothesis() {
        let c = Claim {
            id: "c-1".into(),
            statement: "Regeneration validated at site".into(),
            status: ClaimStatus::Hypothesis,
            investor_facing: true,
        };
        assert!(!c.is_disciplined(), "investor-facing hypothesis is overclaiming");

        let ok = Claim { status: ClaimStatus::PilotObserved, ..c.clone() };
        assert!(ok.is_disciplined());

        // Non-investor-facing hypothesis is fine.
        let internal = Claim { investor_facing: false, ..c };
        assert!(internal.is_disciplined());
    }

    #[test]
    fn status_ordering_holds() {
        assert!(ClaimStatus::Architectural < ClaimStatus::PilotObserved);
        assert!(ClaimStatus::PilotObserved < ClaimStatus::MarketAdmitted);
    }
}
