use super::ids::ResourceSpecId;

#[derive(Debug, Clone, PartialEq)]
pub struct ResourceSpec {
    pub id: ResourceSpecId,
    pub name: String,
    pub unit: String,
}

impl ResourceSpec {
    pub fn is_valid(&self) -> bool {
        !self.name.is_empty() && !self.unit.is_empty()
    }
}
