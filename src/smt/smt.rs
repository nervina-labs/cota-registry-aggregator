use crate::db::get_registered_lock_leaves;
use crate::error::Error;
use crate::smt::db::schema::{
    COLUMN_SMT_BRANCH, COLUMN_SMT_LEAF, COLUMN_SMT_ROOT, COLUMN_SMT_TEMP_LEAVES,
};
use crate::smt::store::smt_store::SMTStore;
use crate::smt::transaction::store_transaction::StoreTransaction;
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
        self.store().insert_leaves(leaves)?;
        debug!("Save latest smt root: {:?} and leaves", self.root());
        Ok(())
    }
}

pub fn init_smt<'a>(transaction: &'a StoreTransaction) -> Result<CotaSMT<'a>, Error> {
    let smt_store = SMTStore::new(
        COLUMN_SMT_LEAF,
        COLUMN_SMT_BRANCH,
        COLUMN_SMT_ROOT,
        COLUMN_SMT_TEMP_LEAVES,
        transaction,
    );
    let root = smt_store
        .get_root()
        .map_err(|_e| Error::SMTError("Get smt root".to_string()))?
        .unwrap_or_default();
    debug!("rocksdb smt root: {:?}", root,);
    Ok(CotaSMT::new(root, smt_store))
}

pub fn generate_history_smt<'a>(
    smt: &mut CotaSMT<'a>,
    smt_root_opt: Option<[u8; 32]>,
) -> Result<(), Error> {
    let root = *smt.root();
    if root == H256::zero() {
        return generate_mysql_smt(smt);
    }
    debug!("registry cell smt root: {:?}", smt_root_opt,);
    if let Some(smt_root) = smt_root_opt {
        if smt_root.as_slice() == root.as_slice() {
            debug!("The smt leaves and root in rocksdb are right");
            return Ok(());
        } else {
            reset_smt_temp_leaves(smt)?;
            if smt_root.as_slice() == smt.root().as_slice() {
                debug!("The smt leaves and root in rocksdb are right after reset");
                return Ok(());
            }
        }
    }
    generate_mysql_smt(smt)
}

fn generate_mysql_smt<'a>(smt: &mut CotaSMT<'a>) -> Result<(), Error> {
    let start_time = Local::now().timestamp_millis();
    let registered_lock_leaves: Vec<(H256, H256)> = get_registered_lock_leaves()?;
    smt.update_all(registered_lock_leaves)
        .expect("SMT update leave error");
    let diff_time = (Local::now().timestamp_millis() - start_time) as f64 / 1000f64;
    debug!("Push registry history leaves to smt: {}s", diff_time);
    Ok(())
}

fn reset_smt_temp_leaves<'a>(smt: &mut CotaSMT<'a>) -> Result<(), Error> {
    let leaves_opt = smt.store().get_leaves()?;
    if let Some(leaves) = leaves_opt {
        smt.update_all(leaves)
            .expect("SMT update temp leaves error");
    }
    debug!("Reset temp leaves successfully");
    Ok(())
}
