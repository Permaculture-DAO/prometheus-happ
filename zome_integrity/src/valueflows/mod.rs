pub mod ids;
pub use ids::*;
pub mod agent;
pub mod resource;
pub mod process;
pub mod event;

pub use agent::Agent;
pub use resource::ResourceSpec;
pub use process::Process;
pub use event::{EconomicEvent, EconomicEventAction};
#[cfg(test)]
mod tests {
    use super::*;
    use super::ids::{
        AgentId,
        ResourceSpecId,
        ProcessId,
        EventId,
    };

    //
    // AGENT
    //

    #[test]
    fn simple_agent_validation() {
        let a = Agent {
            id: AgentId("agent-1".into()),
            name: "Uwohali".into(),
        };
        assert!(a.is_valid());

        let invalid = Agent {
            id: AgentId("agent-2".into()),
            name: "".into(),
        };
        assert!(!invalid.is_valid());
    }

    //
    // RESOURCE SPEC
    //

    #[test]
    fn resource_spec_validation() {
        let rs = ResourceSpec {
            id: ResourceSpecId("resource-spec-1".into()),
            name: "Soil Organic Matter".into(),
            unit: "percent".into(),
        };

        assert!(rs.is_valid(), "ResourceSpec valido deve passare is_valid");

        let rs_no_name = ResourceSpec {
            id: ResourceSpecId("resource-spec-2".into()),
            name: "".into(),
            unit: "percent".into(),
        };
        assert!(!rs_no_name.is_valid(), "ResourceSpec senza nome deve fallire");

        let rs_no_unit = ResourceSpec {
            id: ResourceSpecId("resource-spec-3".into()),
            name: "Soil Carbon".into(),
            unit: "".into(),
        };
        assert!(!rs_no_unit.is_valid(), "ResourceSpec senza unità deve fallire");
    }

    //
    // PROCESS
    //

    #[test]
    fn process_validation() {
        let p = Process {
            id: ProcessId("process-1".into()),
            name: "Regenerative grazing rotation".into(),
        };

        assert!(p.is_valid(), "Process valido deve passare is_valid");

        let p_empty_name = Process {
            id: ProcessId("process-2".into()),
            name: "".into(),
        };

        assert!(!p_empty_name.is_valid(), "Process senza nome deve fallire");
    }

    //
    // ECONOMIC EVENT
    //

    #[test]
    fn economic_event_validation() {
        let provider = Agent {
            id: AgentId("agent-1".into()),
            name: "Uwohali".into(),
        };

        let receiver = Agent {
            id: AgentId("agent-2".into()),
            name: "Community DAO".into(),
        };

        let process = Process {
            id: ProcessId("process-1".into()),
            name: "Tree planting campaign".into(),
        };

        let resource_spec = ResourceSpec {
            id: ResourceSpecId("resource-1".into()),
            name: "Native trees planted".into(),
            unit: "trees".into(),
        };

        let event = EconomicEvent {
            id: EventId("event-1".into()),
            action: EconomicEventAction::Produce,
            provider: provider.clone(),
            receiver: Some(receiver.clone()),
            process: Some(process.clone()),
            resource_spec: resource_spec.clone(),
            quantity: 100.0,
        };

        assert!(event.is_valid(), "Event valido deve passare is_valid");

        // ❌ quantita' zero
        let zero_qty = EconomicEvent {
            quantity: 0.0,
            ..event.clone()
        };
        assert!(!zero_qty.is_valid(), "Event con quantity = 0 deve fallire");

        // ❌ ID vuoto
        let empty_id = EconomicEvent {
            id: EventId("".into()),
            ..event.clone()
        };
        assert!(!empty_id.is_valid(), "Event con id vuoto deve fallire");

        // ❌ provider non valido
        let bad_provider = EconomicEvent {
            provider: Agent {
                id: AgentId("bad-agent".into()),
                name: "".into(),
            },
            ..event.clone()
        };
        assert!(!bad_provider.is_valid(), "Event con provider non valido deve fallire");

        // ❌ resource_spec non valido
        let bad_resource = EconomicEvent {
            resource_spec: ResourceSpec {
                id: ResourceSpecId("bad-resource".into()),
                name: "".into(),
                unit: "".into(),
            },
            ..event
        };
        assert!(!bad_resource.is_valid(), "Event con resource_spec non valido deve fallire");
    }
}
