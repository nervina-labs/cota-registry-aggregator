use crate::config::load_config;
use crate::error::Error;
use crate::utils::parse_bytes_n;
use lazy_static::lazy_static;
use log::error;
use mysql::prelude::*;
use mysql::*;
use std::{result::Result, sync::Mutex};
use std::env;

lazy_static! {
    pub static ref CONN: Mutex<PooledConn> = {
        let url = load_config().database_url;
        let max: usize = if let Ok(max_pool) = env::var("MAX_POOL") {
            max_pool.parse::<usize>().unwrap()
        } else {
            20
        };
        let pool = Pool::new_manual(10usize, max, Opts::from_url(&url).unwrap()).expect("Database pool error");
        Mutex::new(pool.get_conn().expect("Database connection error"))
    };
}

pub fn get_registered_lock_hashes() -> Result<Vec<[u8; 32]>, Error> {
    CONN.lock()
        .unwrap()
        .query_map(
            "select lock_hash from register_cota_kv_pairs",
            |lock_hash| parse_mysql_bytes_n::<32>(lock_hash),
        )
        .map_err(|e| {
            error!("Query registry error: {}", e.to_string());
            Error::DatabaseQueryError(e.to_string())
        })
}

fn parse_mysql_bytes_n<const N: usize>(v: Value) -> [u8; N] {
    let vec = from_value::<Vec<u8>>(v);
    parse_bytes_n::<N>(String::from_utf8(vec).unwrap()).unwrap()
}

pub fn check_lock_hashes_registered(lock_hashes: Vec<[u8; 32]>) -> Result<bool, Error> {
    let lock_hash_vec: Vec<String> = lock_hashes.iter().map(|hash| hex::encode(hash)).collect();
    let lock_hash_array = lock_hash_vec.join("','");
    let result: Vec<String> = CONN
        .lock()
        .unwrap()
        .query(format!(
            "select lock_hash from register_cota_kv_pairs where lock_hash in ('{}')",
            lock_hash_array
        ))
        .map_err(|e| {
            error!("Query registry error: {}", e.to_string());
            Error::DatabaseQueryError(e.to_string())
        })?;
    Ok(result.len() > 0)
}
