use jsonrpc_http_server::jsonrpc_core::{IoHandler, Params};
use jsonrpc_http_server::ServerBuilder;
use log::info;
use rpc::api::register_rpc;

const REGISTER_RPC: &'static str = "register_cota_cells";

fn main() {
    env_logger::Builder::from_default_env()
        .format_timestamp(Some(env_logger::fmt::TimestampPrecision::Millis))
        .init();

    let mut io = IoHandler::default();
    io.add_method(REGISTER_RPC, move |params: Params| register_rpc(params));

    let server = ServerBuilder::new(io)
        .threads(3)
        .start_http(&"0.0.0.0:3050".parse().unwrap())
        .unwrap();

    info!("Registry cota aggregator server start");

    server.wait();
}
