use crate::db::{check_lock_hashes_registered, RegistryState};
use crate::error::Error;
use crate::indexer::index::{get_registry_info, RegistryInfo};
use crate::smt::db::db::RocksDB;
use crate::smt::smt::{generate_history_smt, init_smt, Extension};
use crate::smt::transaction::store_transaction::StoreTransaction;
use cota_smt::common::{Byte32, BytesBuilder};
use cota_smt::molecule::prelude::*;
use cota_smt::registry::{
    CotaNFTRegistryEntriesBuilder, Registry, RegistryBuilder, RegistryVecBuilder,
};
use cota_smt::smt::H256;
use lazy_static::lazy_static;
use log::info;
use parking_lot::{Condvar, Mutex};
use std::sync::Arc;

lazy_static! {
    static ref SMT_LOCK: Arc<(Mutex<bool>, Condvar)> =
        Arc::new((Mutex::new(false), Condvar::new()));
}

pub async fn generate_registry_smt(
    db: &RocksDB,
    lock_hashes: Vec<[u8; 32]>,
) -> Result<(String, String), Error> {
    let update_leaves_count = lock_hashes.len();
    let registry_state = check_lock_hashes_registered(lock_hashes.clone())?.0;
    if registry_state == RegistryState::WithCCID {
        return Err(Error::LockHashHasRegisteredWithCCID);
    }
    let mut update_leaves: Vec<(H256, H256)> = Vec::with_capacity(update_leaves_count);
    let mut previous_leaves: Vec<(H256, H256)> = Vec::with_capacity(update_leaves_count);
    let mut registry_value = [255u8; 32];
    let RegistryInfo {
        smt_root,
        account_num,
    } = get_registry_info().await?;
    for (index, lock_hash) in lock_hashes.into_iter().enumerate() {
        let key: H256 = H256::from(lock_hash);
        registry_value[0..8].copy_from_slice(&(account_num + index as u64).to_be_bytes());
        let value: H256 = H256::from(registry_value);
        update_leaves.push((key, value));
        previous_leaves.push((key, H256::zero()));
    }

    let transaction = &StoreTransaction::new(db.transaction());
    let mut smt = init_smt(transaction)?;

    with_lock(|| {
        generate_history_smt(&mut smt, smt_root)?;
        smt.update_all(update_leaves.clone())
            .map_err(|e| Error::SMTError(e.to_string()))?;
        smt.save_root_and_leaves(previous_leaves.clone())?;
        transaction.commit()
    })?;

    let root_hash = hex::encode(smt.root().as_slice());
    info!("registry_smt_root_hash: {:?}", root_hash);

    let registry_merkle_proof = smt
        .merkle_proof(update_leaves.iter().map(|leave| leave.0).collect())
        .unwrap();
    let registry_merkle_proof_compiled = registry_merkle_proof
        .compile(update_leaves.clone())
        .unwrap();

    let merkel_proof_vec: Vec<u8> = registry_merkle_proof_compiled.into();

    let registry_leaves = if registry_state == RegistryState::NoCCID {
        update_leaves
            .into_iter()
            .map(|leaf| {
                let mut registry_value_no_ccid = [0xEEu8; 32];
                registry_value_no_ccid[0..8].copy_from_slice(&leaf.1.as_slice()[0..8]);
                (leaf.0, H256::from(registry_value_no_ccid))
            })
            .collect()
    } else {
        update_leaves
    };
    let registry_vec = registry_leaves
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

fn with_lock<F>(mut operator: F) -> Result<(), Error>
where
    F: FnMut() -> Result<(), Error>,
{
    let &(ref lock, ref cond) = &*Arc::clone(&SMT_LOCK);
    {
        let mut pending = lock.lock();
        while *pending {
            cond.wait(&mut pending);
        }
        *pending = true;
    }
    let unlock = || {
        let mut pending = lock.lock();
        *pending = false;
        cond.notify_all();
    };

    operator().map_err(|err| {
        unlock();
        err
    })?;
    unlock();
    Ok(())
}
