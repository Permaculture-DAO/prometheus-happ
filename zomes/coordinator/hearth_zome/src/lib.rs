use hdk::prelude::*;

#[hdk_extern]
fn init() -> ExternResult<InitCallbackResult> {
    Ok(InitCallbackResult::Pass)
}

#[hdk_extern]
fn hello() -> ExternResult<String> {
    Ok("Hello from hearth!".to_string())
}
