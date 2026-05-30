use super::ids::AgentId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Agent {
    pub id: AgentId,
    pub name: String,
}

impl Agent {
    pub fn new(id: AgentId, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.name.trim().is_empty()
    }
}
