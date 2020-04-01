use anyhow::Error;

use domain::executor::ports::secondary::TaskStoragePort;

use crate::secondary::adapter::storage::database::SqliteStorageAdapter;
use crate::secondary::adapter::storage::memory::InMemoryStorageAdapter;
use crate::primary::settings::StorageConfiguration;

pub mod database;
pub mod memory;

pub fn new_storage_adapter(storage_type: StorageConfiguration) -> Result<Box<dyn TaskStoragePort>, Error> {
    match storage_type {
        StorageConfiguration::Database { database_url } => {
            // See README.md#limitations
            let type_trick= SqliteStorageAdapter::new(&database_url)?;
            Ok(Box::new(type_trick))

        },
        StorageConfiguration::InMemory => Ok(Box::new(InMemoryStorageAdapter::new()))
    }
}