#[macro_use]
extern crate diesel;
extern crate dotenv;

use crate::api::{check_registered_rpc, register_rpc, update_registered_ccid_rpc};
use crate::db::{init_connection_pool, SqlConnectionPool};
use crate::smt::db::db::RocksDB;
use dotenv::dotenv;
use jsonrpc_http_server::jsonrpc_core::IoHandler;
use jsonrpc_http_server::ServerBuilder;
use lazy_static::lazy_static;
use log::info;

mod api;
mod db;
mod error;
mod indexer;
mod schema;
mod smt;
mod utils;

lazy_static! {
    static ref DB: RocksDB = RocksDB::default().expect("RocksDB open error");
    static ref POOL: SqlConnectionPool = init_connection_pool();
}

fn main() {
    dotenv().ok();
    env_logger::Builder::from_default_env()
        .format_timestamp(Some(env_logger::fmt::TimestampPrecision::Millis))
        .init();

    let mut io = IoHandler::default();
    io.add_method("register_cota_cells", |req| register_rpc(req, &DB));
    io.add_method("update_registered_ccid", |_req| {
        update_registered_ccid_rpc(&DB)
    });
    io.add_method("check_registered_lock_hashes", check_registered_rpc);

    let server = ServerBuilder::new(io)
        .threads(3)
        .start_http(&"0.0.0.0:3050".parse().unwrap())
        .unwrap();

    let version = env!("CARGO_PKG_VERSION");
    info!(
        "{}",
        format!("Registry cota aggregator v{} server start", version)
    );

    server.wait();
}
