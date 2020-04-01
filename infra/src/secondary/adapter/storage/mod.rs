use anyhow::Error;

use domain::executor::ports::secondary::TaskStoragePort;

use crate::secondary::adapter::storage::database::SqliteStorageAdapter;
use crate::secondary::adapter::storage::memory::InMemoryStorageAdapter;

pub mod database;
pub mod memory;

pub enum StorageType<'a> {
    Database {
        database_url: &'a str
    },
    InMemory,
}

pub fn new_storage_adapter(storage_type: StorageType) -> Result<Box<dyn TaskStoragePort>, Error> {
    match storage_type {
        StorageType::Database { database_url } => {
            // See README.md#limitations
            let type_trick= SqliteStorageAdapter::new(database_url)?;
            Ok(Box::new(type_trick))

        },
        StorageType::InMemory => Ok(Box::new(InMemoryStorageAdapter::new()))
    }
}