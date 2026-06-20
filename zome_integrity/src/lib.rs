use hdk::prelude::*;

// Domain layer (pure Rust, canon-aligned). These compile into the integrity zome
// and are unit-tested on the host. They model structure and validation only —
// never PRU value (value lives in the prose canon, suspended at zero until gates
// clear). See claim.rs for the binary admissibility gate.
pub mod valueflows;
pub mod holo_entries;
pub mod entries;
pub mod ohe;
pub mod mrv;
pub mod claim;
pub mod no_double_counting;

#[hdk_extern]
fn init() -> ExternResult<InitCallbackResult> {
    Ok(InitCallbackResult::Pass)
}
