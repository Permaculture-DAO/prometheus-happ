use hdk::prelude::*;
use std::collections::HashSet;
use zome_integrity::{EntryTypes, MessageEntry};

#[hdk_extern]
fn init() -> ExternResult<InitCallbackResult> {
    let zome_name = zome_info()?.name;
let granted_functions = GrantedFunctions::Listed(HashSet::from([
    (zome_name.clone(), FunctionName::from("hello")),
    (zome_name.clone(), FunctionName::from("ping")),
    (zome_name.clone(), FunctionName::from("get_status")),
    (zome_name.clone(), FunctionName::from("list_messages")),
    (zome_name.clone(), FunctionName::from("set_message")),
    (zome_name, FunctionName::from("get_message")),
    ]));

    let grant = ZomeCallCapGrant {
        tag: "prometheus-local-dev-bridge".into(),
        access: CapAccess::Unrestricted,
        functions: granted_functions,
    };

    create_cap_grant(grant)?;

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

#[hdk_extern]
fn list_messages(_: ()) -> ExternResult<Vec<String>> {
    let records = query(ChainQueryFilter::new().include_entries(true))?;

    let mut messages = Vec::new();

    for record in records {
        let maybe_message: Option<MessageEntry> = record
            .entry()
            .to_app_option()
            .map_err(|e| wasm_error!(WasmErrorInner::Guest(e.to_string())))?;

        if let Some(message) = maybe_message {
            messages.push(message.content);
        }
    }

    Ok(messages)
}
