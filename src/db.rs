use super::schema::register_cota_kv_pairs::dsl::*;
use crate::error::Error;
use crate::schema::check_infos::dsl::block_number;
use crate::schema::check_infos::dsl::check_infos;
use crate::utils::parse_bytes_n;
use diesel::r2d2::{self, ConnectionManager, PooledConnection};
use diesel::*;
use dotenv::dotenv;
use jsonrpc_http_server::jsonrpc_core::serde_json::from_str;
use log::error;
use std::env;

pub type SqlConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

fn establish_connection() -> SqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let max: u32 = match env::var("MAX_POOL") {
        Ok(max_) => from_str::<u32>(&max_).unwrap(),
        Err(_e) => 20,
    };
    let pool = r2d2::Pool::builder().max_size(max).build(manager).unwrap();
    pool.get().expect("Error connecting to database")
}

pub fn get_registered_lock_hashes() -> Result<(Vec<[u8; 32]>, u64), Error> {
    let conn = &establish_connection();
    const PAGE_SIZE: i64 = 1000;
    let mut lock_hashes: Vec<[u8; 32]> = Vec::new();
    let mut page: i64 = 0;
    loop {
        let lock_hashes_page = register_cota_kv_pairs
            .select(lock_hash)
            .limit(PAGE_SIZE)
            .offset(PAGE_SIZE * page)
            .load::<String>(conn)
            .map_or_else(
                |e| {
                    error!("Query registry error: {}", e.to_string());
                    Err(Error::DatabaseQueryError(e.to_string()))
                },
                |registries| Ok(parse_registry_cota_nft(registries)),
            )?;
        let length = lock_hashes_page.len();
        lock_hashes.extend(lock_hashes_page);
        if length < (PAGE_SIZE as usize) {
            break;
        }
        page += 1;
    }
    let block_height = get_syncer_tip_block_number_with_conn(conn)?;
    Ok((lock_hashes, block_height))
}

pub fn check_lock_hashes_registered(lock_hashes: Vec<[u8; 32]>) -> Result<bool, Error> {
    let conn = &establish_connection();
    let lock_hash_vec: Vec<String> = lock_hashes.iter().map(|hash| hex::encode(hash)).collect();
    let lock_count = lock_hash_vec.len() as i64;
    let registry_count = register_cota_kv_pairs
        .filter(lock_hash.eq_any(lock_hash_vec))
        .count()
        .get_result::<i64>(conn)
        .map_err(|e| {
            error!("Query registry error: {}", e.to_string());
            Error::DatabaseQueryError(e.to_string())
        })?;
    Ok(registry_count == lock_count)
}

fn get_syncer_tip_block_number_with_conn(conn: &SqlConnection) -> Result<u64, Error> {
    check_infos
        .select(block_number)
        .first::<u64>(conn)
        .map_err(|e| {
            error!("Query block number error: {}", e.to_string());
            Error::DatabaseQueryError(e.to_string())
        })
}

fn parse_registry_cota_nft(registries: Vec<String>) -> Vec<[u8; 32]> {
    registries
        .into_iter()
        .map(|registry| parse_bytes_n(registry).unwrap())
        .collect()
}
