use crate::utils::{check_request_params, parse_values};
use jsonrpc_http_server::jsonrpc_core::serde_json::Map;
use jsonrpc_http_server::jsonrpc_core::{IoHandler, Params, Value};
use jsonrpc_http_server::ServerBuilder;
use smt::generate_registry_smt;

mod smt;
mod utils;

fn main() {
    let mut io = IoHandler::default();
    io.add_method("register_compact_nft", |params: Params| async move {
        match params {
            Params::Array(values) => {
                println!("params: {:?}", values);
                if let Some(error) = check_request_params(values.clone()) {
                    return Ok(error);
                }
                let lock_hashes = parse_values(values);
                let (root_hash, witness_data) = generate_registry_smt(lock_hashes);

                let mut response = Map::new();
                response.insert("registry_entries".to_string(), Value::String(witness_data));
                response.insert("smt_root_hash".to_string(), Value::String(root_hash));

                Ok(Value::Object(response))
            }
            _ => Ok(Value::String("Request parameter format error".to_owned())),
        }
    });

    let server = ServerBuilder::new(io)
        .threads(3)
        .start_http(&"127.0.0.1:3030".parse().unwrap())
        .unwrap();

    server.wait();
}
