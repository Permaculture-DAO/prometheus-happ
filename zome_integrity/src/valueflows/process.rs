use super::ids::ProcessId;

#[derive(Debug, Clone, PartialEq)]
pub struct Process {
    pub id: ProcessId,
    pub name: String,
}

impl Process {
    pub fn is_valid(&self) -> bool {
        !self.name.is_empty()
    }
}
