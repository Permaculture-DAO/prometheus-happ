use hdi::prelude::*;

#[hdk_entry_helper]
#[derive(Clone)]
pub struct MessageEntry {
    pub content: String,
}

#[hdk_entry_types]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    #[entry_type(visibility = "public")]
    Message(MessageEntry),
}
