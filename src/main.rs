use jsonrpc_http_server::jsonrpc_core::{IoHandler, Value, Params};
use jsonrpc_http_server::ServerBuilder;

fn main() {
    let mut io = IoHandler::default();
    io.add_method("register_compact_nft", |_params: Params| async move {
        println!("params: {:?}", _params);
        Ok(Value::String("register compact nft".to_owned()))
    });

    let server = ServerBuilder::new(io)
        .threads(3)
        .start_http(&"127.0.0.1:3030".parse().unwrap())
        .unwrap();

    server.wait();
}
