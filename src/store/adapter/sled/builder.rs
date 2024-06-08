use std::{fs, path::PathBuf, sync::Arc};

use crate::{StoreError, DEFAUTL_NAMESPACE_NAME};

use super::SledStore;

use sled::Db;

/// Builder for creating a `SledStore`.
///
/// This builder enables configuring a `SledStore` with custom
/// settings such as a database name, or using an existing database instance.
/// It provides flexibility for setting up the store based on the needs of the application.
///
/// # Examples
///
/// ## Initializing with a SledDB database path
///
/// ```rust,no_run
/// # use keyv::adapter::sled::{SledStoreBuilder};
/// # use std::sync::Arc;
/// # #[tokio::main]
/// # async fn main() {
/// let store = SledStoreBuilder::new()                        
///     .db_name("custom_database")
///     .build()
///     .await.unwrap();
/// }
/// ```
///
/// ## Using an Existing Client
///
/// ```rust,no_run
/// # use std::sync::Arc;
/// # use keyv::adapter::sled::{SledStoreBuilder};
/// # #[tokio::main]
/// # async fn main() {
/// let db_name = "custom_database";
/// let db = sled::open(db_name).unwrap();
/// let store = SledStoreBuilder::new()
///     .db(Arc::new(db))
///     .db_name(db_name)
///     .build()
///     .await.unwrap();
/// }
/// ```
pub struct SledStoreBuilder {
    db: Option<Arc<Db>>,
    db_name: Option<String>,
}

impl SledStoreBuilder {
    /// Creates a new builder instance with default configuration.
    ///
    /// Initializes the builder with no predefined path, database,
    /// allowing these to be set according to specific requirements.
    pub fn new() -> Self {
        Self {
            db: None,
            db_name: None,
        }
    }

    /// Sets the database path for the `SledStore`.
    ///
    /// This method selects the database within SledDB to be used. If not set, a default path is used.
    ///
    /// # Arguments
    ///
    /// * `database_name` - The path of the database.
    pub fn db_name<S: Into<String>>(mut self, db_name: S) -> Self {
        self.db_name = Some(db_name.into());
        self
    }

    /// Uses an existing database for the `SledStore`.
    ///
    /// This method allows for using an already configured SledDB `Db`. If set,
    /// the `db_name` option is ignored.
    ///
    /// # Arguments
    ///
    /// * `database` - Shared reference to an existing `Db`.
    pub fn db(mut self, db: Arc<Db>) -> Self {
        self.db = Some(db);
        self
    }

    /// Builds the `SledStore` based on the provided configurations.
    ///
    /// Finalizes the builder and creates a `SledStore` instance.
    /// It requires either a SledDB database path or an existing client to be set.
    ///
    /// # Returns
    ///
    /// This method returns a `Result` which, on success, contains the initialized `SledStore`.
    /// On failure, it returns a `StoreError` indicating what went wrong during the initialization.
    pub async fn build(self) -> Result<SledStore, StoreError> {
        let db_name = match &self.db_name {
            Some(db_name) => db_name.to_string(),
            None => {
                println!("Database path not provided, using default");
                DEFAUTL_NAMESPACE_NAME.to_string()
            }
        };

        let db = match self.db {
            Some(db) => db,
            None => {
                if fs::metadata(&db_name).is_err() {
                    println!("will create database {}", db_name);
                }
                let ab_db_name = db_name.replace("~", std::env::var("HOME").unwrap().as_str());
                let path = PathBuf::from(ab_db_name);
                Arc::new(sled::open(path).map_err(|e| StoreError::ConnectionError(e.to_string()))?)
            }
        };

        Ok(SledStore { db, db_name })
    }
}
