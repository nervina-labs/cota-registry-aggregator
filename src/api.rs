use crate::db::{check_lock_hashes_registered, get_syncer_tip_block_number};
use crate::smt::entry::generate_registry_smt;
use crate::utils::{parse_request_param, Inserter};
use jsonrpc_http_server::jsonrpc_core::serde_json::Map;
use jsonrpc_http_server::jsonrpc_core::{Error, Params, Value};
use log::info;

pub async fn register_rpc(params: Params) -> Result<Value, Error> {
    info!("Register cota cells request: {:?}", params);
    let registries: Vec<Value> = Params::parse(params)?;
    let lock_hashes = parse_request_param::<32>(registries).map_err(|err| err.into())?;
    let (root_hash, registry_entry) = generate_registry_smt(lock_hashes)
        .await
        .map_err(|err| err.into())?;
    let block_number = get_syncer_tip_block_number().map_err(|err| err.into())?;
    let mut response = Map::new();
    response.insert_str("smt_root_hash", root_hash);
    response.insert_str("registry_smt_entry", registry_entry);
    response.insert_u64("block_number", block_number);
    Ok(Value::Object(response))
}

pub async fn check_registered_rpc(params: Params) -> Result<Value, Error> {
    info!("Check registered request: {:?}", params);
    let registries: Vec<Value> = Params::parse(params)?;
    let lock_hashes = parse_request_param::<32>(registries).map_err(|err| err.into())?;
    let (registered, block_height) =
        check_lock_hashes_registered(lock_hashes).map_err(|err| err.into())?;
    let mut response = Map::new();
    response.insert_bool("registered", registered);
    response.insert_u64("block_number", block_height);
    Ok(Value::Object(response))
}
