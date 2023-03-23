use abi::orion_service_client::OrionServiceClient;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use tokio::sync::RwLock;
use tonic::transport::Channel;

use crate::errors;

pub mod learn_word;

pub struct DbConnection {
    rpc_service_url: String,
    #[allow(dead_code)]
    sqlite_conn: RwLock<SqlitePool>,
}

impl DbConnection {
    pub async fn new(
        rpc_service_url: Option<String>,
        sqlite_database_url: Option<String>,
    ) -> Result<Self, errors::Error> {
        let rpc_service_url =
            rpc_service_url.unwrap_or_else(|| std::env::var("RPC_SERVICE_URL").unwrap());
        let sqlite_conn_url = sqlite_database_url.unwrap_or_else(|| {
            std::env::var("SQLITE_DATABASE_URL").unwrap_or_else(|_| "data/db.sqlite3".to_string())
        });

        let sqlite_conn = RwLock::new(
            SqlitePoolOptions::new()
                .max_connections(50)
                .connect(&sqlite_conn_url)
                .await?,
        );
        Ok(Self {
            rpc_service_url,
            sqlite_conn,
        })
    }
    async fn get_rpc_conn(&self) -> Result<OrionServiceClient<Channel>, errors::Error> {
        let rpc = OrionServiceClient::connect(self.rpc_service_url.clone()).await?;
        Ok(rpc)
    }
}
