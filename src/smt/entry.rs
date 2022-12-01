use crate::db::check_lock_hashes_registered;
use crate::error::Error;
use crate::indexer::index::get_registry_root_hash;
use crate::smt::smt::{generate_history_smt, init_smt, Extension};
use crate::smt::transaction::store_transaction::StoreTransaction;
use crate::ROCKS_DB;
use cota_smt::common::{Byte32, BytesBuilder};
use cota_smt::molecule::prelude::*;
use cota_smt::registry::{
    CotaNFTRegistryEntriesBuilder, Registry, RegistryBuilder, RegistryVecBuilder,
};
use lazy_static::lazy_static;
use log::info;
use parking_lot::{Condvar, Mutex};
use sparse_merkle_tree::H256;
use std::sync::Arc;

lazy_static! {
    static ref SMT_LOCK: Arc<(Mutex<bool>, Condvar)> =
        Arc::new((Mutex::new(false), Condvar::new()));
}

pub async fn generate_registry_smt(lock_hashes: Vec<[u8; 32]>) -> Result<(String, String), Error> {
    let update_leaves_count = lock_hashes.len();
    if check_lock_hashes_registered(lock_hashes.clone())?.0 {
        return Err(Error::LockHashHasRegistered);
    }
    let mut update_leaves: Vec<(H256, H256)> = Vec::with_capacity(update_leaves_count);
    let mut previous_leaves: Vec<(H256, H256)> = Vec::with_capacity(update_leaves_count);
    let smt_root = get_registry_root_hash().await?;

    for lock_hash in lock_hashes.into_iter() {
        let key: H256 = H256::from(lock_hash);
        update_leaves.push((key, H256::from([255u8; 32])));
        previous_leaves.push((key, H256::zero()));
    }

    let transaction = &StoreTransaction::new(ROCKS_DB.transaction());
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
        .compile(update_leaves.iter().map(|leave| leave.0).collect())
        .unwrap();

    let merkel_proof_vec: Vec<u8> = registry_merkle_proof_compiled.into();

    let registry_vec = update_leaves
        .iter()
        .map(|leave| {
            RegistryBuilder::default()
                .lock_hash(Byte32::from_slice(leave.0.as_slice()).unwrap())
                .state(Byte32::from_slice(leave.1.as_slice()).unwrap())
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
