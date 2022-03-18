use crate::db::check_lock_hashes_registered;
use crate::smt::generate_registry_smt;
use crate::utils::parse_request_param;
use jsonrpc_http_server::jsonrpc_core::serde_json::Map;
use jsonrpc_http_server::jsonrpc_core::{Error, Params, Value};

pub async fn register_rpc(params: Params) -> Result<Value, Error> {
    let registries: Vec<Value> = Params::parse(params)?;
    let lock_hashes = parse_request_param::<32>(registries).map_err(|err| err.into())?;
    let response = generate_registry_smt(lock_hashes).map_err(|err| err.into())?;
    Ok(Value::Object(response))
}

pub async fn check_registered_rpc(params: Params) -> Result<Value, Error> {
    let registries: Vec<Value> = Params::parse(params)?;
    let lock_hashes = parse_request_param::<32>(registries).map_err(|err| err.into())?;
    let result = check_lock_hashes_registered(lock_hashes).map_err(|err| err.into())?;
    let mut response = Map::new();
    response.insert("result".to_string(), Value::Bool(result));
    Ok(Value::Object(response))
}
