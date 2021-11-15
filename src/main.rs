use crate::utils::{check_request_params_error, values_to_u8_vec};
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
                if check_request_params_error(values.clone()) {
                    return Ok(Value::String("Request parameters error".to_owned()));
                }
                let lock_hashes = values_to_u8_vec(values);
                let (root_hash, witness_data) = generate_registry_smt(lock_hashes);

                Ok(Value::Array(vec![
                    Value::String(root_hash),
                    Value::String(witness_data),
                ]))
            }
            _ => Ok(Value::String("Parameters error".to_owned())),
        }
    });

    let server = ServerBuilder::new(io)
        .threads(3)
        .start_http(&"127.0.0.1:3030".parse().unwrap())
        .unwrap();

    server.wait();
}
