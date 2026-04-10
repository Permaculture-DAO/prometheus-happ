use hdk::prelude::*;
use zome_integrity::{EntryTypes, MessageEntry};

#[hdk_extern]
fn init() -> ExternResult<InitCallbackResult> {
    Ok(InitCallbackResult::Pass)
}

#[hdk_extern]
fn hello() -> ExternResult<String> {
    Ok("Hello from zome_coordinator!".to_string())
}

#[hdk_extern]
fn ping() -> ExternResult<String> {
    Ok("pong".to_string())
}

#[hdk_extern]
fn get_status() -> ExternResult<String> {
    Ok("prometheus runtime ok".to_string())
}

#[hdk_extern]
fn set_message(content: String) -> ExternResult<ActionHash> {
    let entry = MessageEntry { content };
    create_entry(&EntryTypes::Message(entry))
}

#[hdk_extern]
fn get_message(action_hash: ActionHash) -> ExternResult<Option<String>> {
    let Some(record) = get(action_hash, GetOptions::default())? else {
        return Ok(None);
    };

    let Some(message): Option<MessageEntry> = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(WasmErrorInner::Guest(e.to_string())))?
    else {
        return Ok(None);
    };

    Ok(Some(message.content))
}
