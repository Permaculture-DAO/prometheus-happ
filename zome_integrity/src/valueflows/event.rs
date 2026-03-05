use super::agent::Agent;
use super::process::Process;
use super::resource::ResourceSpec;
use super::ids::EventId;

#[derive(Debug, Clone, PartialEq)]
pub enum EconomicEventAction {
    Consume,
    Produce,
    Use,
    Work,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EconomicEvent {
    pub id: EventId,
    pub action: EconomicEventAction,
    pub provider: Agent,
    pub receiver: Option<Agent>,
    pub process: Option<Process>,
    pub resource_spec: ResourceSpec,
    pub quantity: f64,
}

impl EconomicEvent {
    pub fn is_valid(&self) -> bool {
        !self.id.0.is_empty()
            && self.quantity > 0.0
            && self.provider.is_valid()
            && self.resource_spec.is_valid()
    }
}
