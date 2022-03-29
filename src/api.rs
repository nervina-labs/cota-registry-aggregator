use crate::db::{check_lock_hashes_registered, get_block_number};
use crate::smt::entry::generate_registry_smt;
use crate::utils::parse_request_param;
use jsonrpc_http_server::jsonrpc_core::serde_json::{Map, Number};
use jsonrpc_http_server::jsonrpc_core::{Error, Params, Value};

pub async fn register_rpc(params: Params) -> Result<Value, Error> {
    let registries: Vec<Value> = Params::parse(params)?;
    let lock_hashes = parse_request_param::<32>(registries).map_err(|err| err.into())?;
    let (root_hash, registry_entry) = generate_registry_smt(lock_hashes)
        .await
        .map_err(|err| err.into())?;
    let block_number = get_block_number().map_err(|err| err.into())?;
    let mut response = Map::new();
    response.insert("smt_root_hash".to_string(), Value::String(root_hash));
    response.insert(
        "registry_smt_entry".to_string(),
        Value::String(registry_entry),
    );
    response.insert(
        "block_number".to_string(),
        Value::Number(Number::from(block_number)),
    );
    Ok(Value::Object(response))
}

pub async fn check_registered_rpc(params: Params) -> Result<Value, Error> {
    let registries: Vec<Value> = Params::parse(params)?;
    let lock_hashes = parse_request_param::<32>(registries).map_err(|err| err.into())?;
    let (registered, block_height) =
        check_lock_hashes_registered(lock_hashes).map_err(|err| err.into())?;
    let mut response = Map::new();
    response.insert("registered".to_string(), Value::Bool(registered));
    response.insert(
        "block_number".to_string(),
        Value::Number(Number::from(block_height)),
    );
    Ok(Value::Object(response))
}
