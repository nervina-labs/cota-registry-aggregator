use crate::db::{check_lock_hashes_registered, get_registered_lock_hashes};
use crate::error::Error;
use cota_smt::common::{Byte32, BytesBuilder};
use cota_smt::molecule::prelude::*;
use cota_smt::registry::{
    CotaNFTRegistryEntriesBuilder, Registry, RegistryBuilder, RegistryVecBuilder,
};
use cota_smt::smt::{Blake2bHasher, H256, SMT};
use jsonrpc_http_server::jsonrpc_core::serde_json::{Map, Number};
use jsonrpc_http_server::jsonrpc_core::Value;
use log::info;

pub fn generate_registry_smt(lock_hashes: Vec<[u8; 32]>) -> Result<Map<String, Value>, Error> {
    let mut smt = SMT::default();
    let update_leaves_count = lock_hashes.len();

    if check_lock_hashes_registered(lock_hashes.clone())?.0 {
        return Err(Error::LockHashHasRegistered);
    }

    let (registered_lock_hashes, block_number) = get_registered_lock_hashes()?;
    if !registered_lock_hashes.is_empty() {
        for lock_hash in registered_lock_hashes {
            let key: H256 = H256::from(lock_hash);
            let value: H256 = H256::from([255u8; 32]);
            smt.update(key, value).expect("SMT update leave error");
        }
    }

    let mut update_leaves: Vec<(H256, H256)> = Vec::with_capacity(update_leaves_count);
    for lock_hash in lock_hashes {
        let key: H256 = H256::from(lock_hash);
        let value: H256 = H256::from([255u8; 32]);
        update_leaves.push((key, value));
        smt.update(key, value).expect("SMT update leave error");
    }
    let root_hash = smt.root().clone();

    let mut root_hash_bytes = [0u8; 32];
    root_hash_bytes.copy_from_slice(root_hash.as_slice());
    let root_hash_hex = hex::encode(root_hash_bytes);

    info!("registry_smt_root_hash: {:?}", root_hash_hex);

    let registry_merkle_proof = smt
        .merkle_proof(update_leaves.iter().map(|leave| leave.0).collect())
        .unwrap();
    let registry_merkle_proof_compiled = registry_merkle_proof
        .compile(update_leaves.clone())
        .unwrap();
    registry_merkle_proof_compiled
        .verify::<Blake2bHasher>(&root_hash, update_leaves.clone())
        .expect("smt proof verify failed");

    let merkel_proof_vec: Vec<u8> = registry_merkle_proof_compiled.into();

    let registry_vec = update_leaves
        .iter()
        .map(|leave| {
            let key: [u8; 32] = leave.0.into();
            let value: [u8; 32] = leave.1.into();
            RegistryBuilder::default()
                .lock_hash(Byte32::from_slice(&key).unwrap())
                .state(Byte32::from_slice(&value).unwrap())
                .build()
        })
        .collect::<Vec<Registry>>();

    let registry_vec = RegistryVecBuilder::default().extend(registry_vec).build();
    let merkel_proof_bytes = BytesBuilder::default()
        .extend(merkel_proof_vec.iter().map(|v| Byte::from(*v)))
        .build();

    let registry_entries = CotaNFTRegistryEntriesBuilder::default()
        .registries(registry_vec)
        .proof(merkel_proof_bytes)
        .build();

    let registry_entry = hex::encode(registry_entries.as_slice());

    info!("registry_smt_entry: {:?}", registry_entry);

    let mut result = Map::new();
    result.insert("smt_root_hash".to_string(), Value::String(root_hash_hex));
    result.insert(
        "registry_smt_entry".to_string(),
        Value::String(registry_entry),
    );
    result.insert(
        "block_number".to_string(),
        Value::Number(Number::from(block_number)),
    );
    Ok(result)
}
