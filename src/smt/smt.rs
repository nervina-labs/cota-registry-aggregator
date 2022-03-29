use crate::db::get_registered_lock_hashes;
use crate::error::Error;
use crate::indexer::index::get_registry_smt_root;
use crate::smt::db::cota_db::CotaRocksDB;
use crate::smt::db::schema::{
    COLUMN_SMT_BRANCH, COLUMN_SMT_LEAF, COLUMN_SMT_ROOT, COLUMN_SMT_TEMP_LEAVES,
};
use crate::smt::store::smt_store::SMTStore;
use chrono::prelude::*;
use cota_smt::smt::{Blake2bHasher, H256};
use log::debug;
use sparse_merkle_tree::SparseMerkleTree;

pub type CotaSMT<'a> = SparseMerkleTree<Blake2bHasher, H256, SMTStore<'a>>;

pub trait RootSaver {
    fn save_root_and_leaves(&self, leaves: Vec<(H256, H256)>) -> Result<(), Error>;
}

impl<'a> RootSaver for CotaSMT<'a> {
    fn save_root_and_leaves(&self, leaves: Vec<(H256, H256)>) -> Result<(), Error> {
        self.store()
            .save_root(self.root())
            .expect("Save smt root error");
        if !leaves.is_empty() {
            self.store().insert_leaves(leaves)?;
        }
        debug!("Save latest smt root: {:?} and leaves", self.root());
        Ok(())
    }
}

pub async fn generate_history_smt<'a>(db: &'a CotaRocksDB) -> Result<CotaSMT<'a>, Error> {
    let smt_store = SMTStore::new(
        COLUMN_SMT_LEAF,
        COLUMN_SMT_BRANCH,
        COLUMN_SMT_ROOT,
        COLUMN_SMT_TEMP_LEAVES,
        db,
    );
    let root = smt_store
        .get_root()
        .map_err(|_e| Error::SMTError("Get smt root".to_string()))?
        .unwrap_or_default();
    debug!("rocksdb smt root: {:?}", root,);
    let mut smt: CotaSMT = CotaSMT::new(root, smt_store);

    if root == H256::zero() {
        return generate_mysql_smt(smt);
    }
    let smt_root_opt = get_registry_smt_root().await?;
    debug!("registry cell smt root: {:?}", smt_root_opt,);
    if let Some(smt_root) = smt_root_opt {
        if smt_root.as_slice() == root.as_slice() {
            return Ok(smt);
        }
    }
    smt = reset_smt_temp_leaves(smt)?;
    generate_mysql_smt(smt)
}

fn generate_mysql_smt<'a>(mut smt: CotaSMT<'a>) -> Result<CotaSMT<'a>, Error> {
    let start_time = Local::now().timestamp_millis();
    let registered_lock_hashes = get_registered_lock_hashes()?;
    if !registered_lock_hashes.is_empty() {
        for lock_hash in registered_lock_hashes {
            let key: H256 = H256::from(lock_hash);
            let value: H256 = H256::from([255u8; 32]);
            smt.update(key, value).expect("SMT update leave error");
        }
    }
    let diff_time = (Local::now().timestamp_millis() - start_time) as f64 / 1000f64;
    debug!("Push registry history leaves to smt: {}s", diff_time);
    Ok(smt)
}

fn reset_smt_temp_leaves<'a>(mut smt: CotaSMT<'a>) -> Result<CotaSMT<'a>, Error> {
    let leaves_opt = smt.store().get_leaves()?;
    if let Some(leaves) = leaves_opt {
        smt.update_all(leaves)
            .expect("SMT update temp leaves error");
    }
    debug!("Reset temp leaves successfully");
    Ok(smt)
}
