use std::sync::Arc;

pub use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::{StoreError, DEFAUTL_NAMESPACE_NAME};

use super::PostgresStore;

/// Builder for creating a `PostgresStore`.
///
/// This builder allows for configuring a `PostgresStore` with custom
/// settings such as a specific database URI, an existing connection pool,
/// and a table name. It provides a flexible way to initialize the store
/// depending on the application's requirements.
///
/// # Examples
///
/// ## Initializing with a Database URI
///
/// ```rust,no_run
/// # use keyv::adapter::postgres::{PostgresStoreBuilder};
/// # use std::sync::Arc;
/// # #[tokio::main]
/// # async fn main(){
/// let store = PostgresStoreBuilder::new()
///     .uri("postgres://username:password@localhost/database")
///     .table_name("custom_table_name")
///     .build()
///     .await.unwrap();
///  }
/// ```
///
/// ## Using an Existing Connection Pool
///
/// ```rust,no_run
/// # use keyv::adapter::postgres::{PostgresStoreBuilder};
/// # use std::sync::Arc;
/// # #[tokio::main]
/// # async fn main() {
/// let pool: Arc<sqlx::PgPool> = Arc::new(sqlx::postgres::PgPoolOptions::new()
///     .connect("postgres://username:password@localhost/database").await.unwrap());
///
/// let store = PostgresStoreBuilder::new()
///     .pool(pool)
///     .table_name("custom_table_name")
///     .build()
///     .await.unwrap();
///  }
/// ```
pub struct PostgresStoreBuilder {
    uri: Option<String>,
    pool: Option<Arc<PgPool>>,
    table_name: Option<String>,
    schema: Option<String>,
}

/// Creates a new builder instance with default configuration.
///
/// Initializes the builder with the default table name and no predefined URI or connection pool.
/// The default table name is defined by `DEFAULT_TABLE_NAME`.
impl PostgresStoreBuilder {
    pub fn new() -> Self {
        Self {
            uri: None,
            pool: None,
            table_name: None,
            schema: None,
        }
    }

    /// Sets the schema name for the `PostgresStore`.
    ///
    /// This method configures the schema name to be used by the store. If not set,
    /// a default schema (usually "public" in PostgreSQL) will be used.
    ///
    /// # Arguments
    ///
    /// * `schema` - The name of the schema.
    pub fn schema<S: Into<String>>(mut self, schema: S) -> Self {
        self.schema = Some(schema.into());
        self
    }

    /// Sets the table name for the `PostgresStore`.
    ///
    /// This method configures the table name to be used by the store. If not set,
    /// `DEFAULT_TABLE_NAME` will be used.
    ///
    /// # Arguments
    ///
    /// * `table` - The name of the table used to store key-value pairs.
    pub fn table_name<S: Into<String>>(mut self, table: S) -> Self {
        self.table_name = Some(table.into());
        self
    }
    /// Sets the database URI for connecting to the PostgreSQL database.
    ///
    /// This method configures the database URI. It's required if no existing connection pool is provided.
    ///
    /// # Arguments
    ///
    /// * `uri` - The database URI string.
    pub fn uri<S: Into<String>>(mut self, uri: S) -> Self {
        self.uri = Some(uri.into());
        self
    }

    /// Uses an existing connection pool for the `PostgresStore`.
    ///
    /// This method allows for using an already configured `PgPool`. If set,
    /// the `uri` option is ignored.
    ///
    /// # Arguments
    ///
    /// * `pool` - Shared reference to an existing `PgPool`.
    pub fn pool(mut self, pool: Arc<PgPool>) -> Self {
        self.pool = Some(pool);
        self
    }

    /// Builds the `PostgresStore` based on the provided configurations.
    ///
    /// Finalizes the builder and creates a `PostgresStore` instance.
    /// It requires either a database URI or an existing connection pool to be set.
    ///
    /// # Returns
    ///
    /// This method returns a `Result` which, on success, contains the initialized `PostgresStore`.
    /// On failure, it returns a `KeyvError` indicating what went wrong during the initialization.
    pub async fn build(self) -> Result<PostgresStore, StoreError> {
        let pool = match self.pool {
            Some(pool) => pool,
            None => {
                let uri = self
                    .uri
                    .expect("PostgresStore requires either a URI or an existing pool to be set");
                Arc::new(PgPoolOptions::new().connect(&uri).await.map_err(|_| {
                    StoreError::ConnectionError("Failed to connect to the database".to_string())
                })?)
            }
        };

        let table_name = match &self.table_name {
            Some(table_name) => table_name.to_string(),
            None => {
                log::warn!("Table name not set, using default table name");
                DEFAUTL_NAMESPACE_NAME.to_string()
            }
        };

        Ok(PostgresStore {
            pool,
            table_name,
            schema: self.schema,
        })
    }
}
