pub mod storage;
pub mod u256;
mod blockstore;
mod abort;
#[cfg(test)]
mod tests {
    use fvm_ipld_encoding::tuple::{Deserialize_tuple, Serialize_tuple};
    use crate::storage::Storage;

    #[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, Default)]
    struct StorageTest {
        count:u32,
    }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn storage() {
       let test = Storage::<StorageTest>::load();
       let storage = Storage{contract:test};
        storage.save();
    }
}
