use crate::db::check_lock_hashes_registered;
use crate::error::Error;
use crate::smt::db::cota_db::CotaRocksDB;
use crate::smt::smt::{generate_history_smt, RootSaver};
use cota_smt::common::{Byte32, BytesBuilder};
use cota_smt::molecule::prelude::*;
use cota_smt::registry::{
    CotaNFTRegistryEntriesBuilder, Registry, RegistryBuilder, RegistryVecBuilder,
};
use cota_smt::smt::H256;
use log::info;

pub async fn generate_registry_smt(
    db: &CotaRocksDB,
    lock_hashes: Vec<[u8; 32]>,
) -> Result<(String, String), Error> {
    let update_leaves_count = lock_hashes.len();
    if check_lock_hashes_registered(lock_hashes.clone())?.0 {
        return Err(Error::LockHashHasRegistered);
    }
    let mut smt = generate_history_smt(db).await?;

    let mut update_leaves: Vec<(H256, H256)> = Vec::with_capacity(update_leaves_count);
    let mut previous_leaves: Vec<(H256, H256)> = Vec::with_capacity(update_leaves_count);
    for lock_hash in lock_hashes {
        let key: H256 = H256::from(lock_hash);
        let value: H256 = H256::from([255u8; 32]);
        update_leaves.push((key, value));
        previous_leaves.push((key, H256::zero()));
        smt.update(key, value).expect("SMT update leave error");
    }
    let root_hash = hex::encode(smt.root().as_slice());
    smt.save_root_and_leaves(previous_leaves)?;

    info!("registry_smt_root_hash: {:?}", root_hash);

    let registry_merkle_proof = smt
        .merkle_proof(update_leaves.iter().map(|leave| leave.0).collect())
        .unwrap();
    let registry_merkle_proof_compiled = registry_merkle_proof
        .compile(update_leaves.clone())
        .unwrap();

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

    Ok((root_hash, registry_entry))
}
