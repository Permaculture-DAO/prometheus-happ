use crate::valueflows::{
    Agent,
    ResourceSpec,
    Process,
    EconomicEvent,
    EconomicEventAction,
};

use crate::valueflows::ids::{
    AgentId,
    ResourceSpecId,
    ProcessId,
};

//
// AGENT
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentEntry {
    pub id: String,
    pub name: String,
}

impl From<Agent> for AgentEntry {
    fn from(a: Agent) -> Self {
        Self {
            id: a.id.0,
            name: a.name,
        }
    }
}

impl From<AgentEntry> for Agent {
    fn from(e: AgentEntry) -> Self {
        Self {
            id: AgentId(e.id),
            name: e.name,
        }
    }
}

//
// RESOURCE SPEC
//

#[derive(Debug, Clone, PartialEq)]
pub struct ResourceSpecEntry {
    pub id: String,
    pub name: String,
    pub unit: String,
}

impl From<ResourceSpec> for ResourceSpecEntry {
    fn from(r: ResourceSpec) -> Self {
        Self {
            id: r.id.0,
            name: r.name,
            unit: r.unit,
        }
    }
}

impl From<ResourceSpecEntry> for ResourceSpec {
    fn from(e: ResourceSpecEntry) -> Self {
        Self {
            id: ResourceSpecId(e.id),
            name: e.name,
            unit: e.unit,
        }
    }
}

//
// PROCESS
//

#[derive(Debug, Clone, PartialEq)]
pub struct ProcessEntry {
    pub id: String,
    pub name: String,
}

impl From<Process> for ProcessEntry {
    fn from(p: Process) -> Self {
        Self {
            id: p.id.0,
            name: p.name,
        }
    }
}

impl From<ProcessEntry> for Process {
    fn from(e: ProcessEntry) -> Self {
        Self {
            id: ProcessId(e.id),
            name: e.name,
        }
    }
}

//
// EVENT ACTION
//

#[derive(Debug, Clone, PartialEq)]
pub enum EconomicEventActionEntry {
    Consume,
    Produce,
    Use,
    Work,
}

impl From<EconomicEventAction> for EconomicEventActionEntry {
    fn from(a: EconomicEventAction) -> Self {
        match a {
            EconomicEventAction::Consume => Self::Consume,
            EconomicEventAction::Produce => Self::Produce,
            EconomicEventAction::Use => Self::Use,
            EconomicEventAction::Work => Self::Work,
        }
    }
}

//
// ECONOMIC EVENT
//

#[derive(Debug, Clone, PartialEq)]
pub struct EconomicEventEntry {
    pub id: String,
    pub action: EconomicEventActionEntry,
    pub provider: AgentEntry,
    pub receiver: Option<AgentEntry>,
    pub process: Option<ProcessEntry>,
    pub resource_spec: ResourceSpecEntry,
    pub quantity: f64,
}

impl From<EconomicEvent> for EconomicEventEntry {
    fn from(e: EconomicEvent) -> Self {
        Self {
            id: e.id.0,
            action: e.action.into(),
            provider: e.provider.into(),
            receiver: e.receiver.map(Into::into),
            process: e.process.map(Into::into),
            resource_spec: e.resource_spec.into(),
            quantity: e.quantity,
        }
    }
}
