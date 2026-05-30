// validation.rs
// Qui definiremo le regole di validazione delle entries.

use crate::entries::ExampleEntry;

pub fn validate_example(entry: &ExampleEntry) -> bool {
    !entry.id.is_empty()
}
