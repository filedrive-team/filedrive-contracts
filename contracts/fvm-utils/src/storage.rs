use crate::blockstore::Blockstore;
use crate::abort;
use cid::multihash::Code;
use cid::Cid;
use fvm_ipld_encoding::{to_vec, CborStore, DAG_CBOR};
use fvm_sdk as sdk;
use serde::{de, ser};

#[allow(dead_code)]
pub struct Storage<T: ser::Serialize + de::DeserializeOwned> {
    pub contract: T,
}

impl<T: ser::Serialize + de::DeserializeOwned> Storage<T> {
    #[allow(dead_code)]
    pub fn load() -> T {
        // First, load the current state root.
        let root = match sdk::sself::root() {
            Ok(root) => root,
            Err(err) => abort!(USR_ILLEGAL_STATE, "failed to get root: {:?}", err),
        };

        // Load the actor state from the state tree.
        match Blockstore.get_cbor::<T>(&root) {
            Ok(Some(state)) => state,
            Ok(None) => abort!(USR_ILLEGAL_STATE, "state does not exist"),
            Err(err) => abort!(USR_ILLEGAL_STATE, "failed to get state: {}", err),
        }
    }

    #[allow(dead_code)]
    pub fn save(&self) -> Cid {
        let serialized = match to_vec(&self.contract) {
            Ok(s) => s,
            Err(err) => abort!(USR_SERIALIZATION, "failed to serialize state: {:?}", err),
        };
        let cid = match sdk::ipld::put(Code::Blake2b256.into(), 32, DAG_CBOR, serialized.as_slice())
        {
            Ok(cid) => cid,
            Err(err) => abort!(USR_SERIALIZATION, "failed to store initial state: {:}", err),
        };
        if let Err(err) = sdk::sself::set_root(&cid) {
            abort!(USR_ILLEGAL_STATE, "failed to set root ciid: {:}", err);
        }
        cid
    }
}