use crate::db::{
    check_lock_hashes_registered, get_50_registered_lock_hashes, get_syncer_tip_block_number,
    RegistryState,
};
use crate::smt::db::db::RocksDB;
use crate::smt::entry::generate_registry_smt;
use crate::utils::{parse_request_param, Inserter};
use jsonrpc_http_server::jsonrpc_core::serde_json::Map;
use jsonrpc_http_server::jsonrpc_core::{Error, Params, Value};
use log::info;

pub async fn register_rpc(params: Params, db: &RocksDB) -> Result<Value, Error> {
    info!("Register cota cells request: {:?}", params);
    let registries: Vec<Value> = Params::parse(params)?;
    let lock_hashes = parse_request_param::<32>(registries).map_err(|err| err.into())?;
    let (root_hash, registry_entry, output_account_num) = generate_registry_smt(db, lock_hashes)
        .await
        .map_err(|err| err.into())?;
    let block_number = get_syncer_tip_block_number().map_err(|err| err.into())?;
    let mut response = Map::new();
    response.insert_str("smt_root_hash", root_hash);
    response.insert_str("registry_smt_entry", registry_entry);
    response.insert_u64("output_account_num", output_account_num);
    response.insert_u64("block_number", block_number);
    Ok(Value::Object(response))
}

pub async fn check_registered_rpc(params: Params) -> Result<Value, Error> {
    info!("Check registered request: {:?}", params);
    let registries: Vec<Value> = Params::parse(params)?;
    let lock_hashes = parse_request_param::<32>(registries).map_err(|err| err.into())?;
    let (registry_state, block_height) =
        check_lock_hashes_registered(lock_hashes).map_err(|err| err.into())?;
    let mut response = Map::new();
    response.insert_bool("registered", registry_state != RegistryState::Unregister);
    response.insert_u64("block_number", block_height);
    Ok(Value::Object(response))
}

pub async fn update_ccid_rpc(db: &RocksDB) -> Result<Value, Error> {
    info!("Update registered ccid request");
    let lock_hashes = get_50_registered_lock_hashes().map_err(|err| err.into())?;
    let (root_hash, registry_entry, output_account_num) = generate_registry_smt(db, lock_hashes)
        .await
        .map_err(|err| err.into())?;
    let block_number = get_syncer_tip_block_number().map_err(|err| err.into())?;
    let mut response = Map::new();
    response.insert_str("smt_root_hash", root_hash);
    response.insert_str("registry_smt_entry", registry_entry);
    response.insert_u64("output_account_num", output_account_num);
    response.insert_u64("block_number", block_number);
    Ok(Value::Object(response))
}
