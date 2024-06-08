use async_trait::async_trait;
use serde_json::Value;
use std::sync::Arc;

use crate::{Store, StoreError};

use sled::{Db, IVec};

pub struct SledStore {
    pub(crate) db: Arc<Db>,
    pub(crate) db_name: String,
}

#[async_trait]
impl Store for SledStore {
    async fn initialize(&self) -> Result<(), StoreError> {
        // Sled creates databases and collections automatically when you insert data,
        // so explicit creation is not needed.
        Ok(())
    }

    async fn get(&self, key: &str) -> Result<Option<Value>, StoreError> {
        match self.db.get(key) {
            Ok(val) => match val {
                Some(val) => Ok(serde_json::from_str(&ivec2str(&val))
                    .map(Some)
                    .map_err(|e| StoreError::SerializationError { source: e })?),
                None => Ok(None),
            },
            Err(e) => Err(StoreError::QueryError(e.to_string())),
        }
    }

    async fn set(&self, key: &str, value: Value, _: Option<u64>) -> Result<(), StoreError> {
        let value_str = serde_json::to_string(&value)
            .map_err(|e| StoreError::SerializationError { source: e })?;
        match self.db.insert(key, str2ivec(&value_str)) {
            Ok(_) => Ok(()),
            Err(e) => Err(StoreError::QueryError(e.to_string())),
        }
    }

    async fn remove(&self, key: &str) -> Result<(), StoreError> {
        match self.db.remove(key) {
            Ok(_) => Ok(()),
            Err(e) => Err(StoreError::QueryError(e.to_string())),
        }
    }

    async fn remove_many(&self, keys: &[&str]) -> Result<(), StoreError> {
        for &key in keys {
            match self.db.remove(key) {
                Ok(_) => continue,
                Err(e) => return Err(StoreError::QueryError(e.to_string())),
            }
        }
        Ok(())
    }

    async fn clear(&self) -> Result<(), StoreError> {
        Ok(())
    }
}

pub(super) fn str2ivec(s: &str) -> IVec {
    IVec::from(s.as_bytes())
}

pub(super) fn ivec2str(val: &IVec) -> String {
    String::from_utf8_lossy(val.to_vec().as_slice()).to_string()
}
