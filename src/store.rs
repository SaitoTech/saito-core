use std::str;
use std::sync::Arc;
pub use kvdb::DBValue;
use kvdb::{DBTransaction, KeyValueDB};
use kvdb_rocksdb::{Database, DatabaseConfig};

pub const STORE_PATH: &str = "data/store";

pub const COL_BLOCK_HEADER: Option<u32> = Some(0);
const NUM_COLS: u32 = 1;

pub struct Store {
    storage: Arc<dyn KeyValueDB>,
}

impl Store {
    pub fn new(storage: Arc<dyn KeyValueDB>) -> Store {
       Store { storage }
    }

    pub fn get(&self, column: Option<u32>, key: &[u8]) -> Result<Option<Vec<u8>>, io::Error> {
        self.storage.get(column, key).map(|a| a.map(|b| b.to_vec()))
    } 
    
    pub fn set(&mut self, column: Option<u32>, key: &[u8], value: &[u8]) {
        self.transaction.put(column, key, value)
    }
}

pub fn create_store(path: &str) -> Arc<Store> {
    let db_config = DatabaseConfig::with_columns(Some(NUM_COLS));
    let db = Arc::new(Database::open(&db_config, path).expect("Failed to open the database"));
    Arc::new(Store::new(db))
}
