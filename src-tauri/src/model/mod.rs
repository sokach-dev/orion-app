use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

use crate::errors;

pub mod data_scaffold;
pub mod dialog;
pub mod learn_word;
pub mod story;
pub mod vocabulary;
pub mod word_list;

pub struct DbConnection {
    conn: SqlitePool,
}

impl DbConnection {
    pub async fn new(sqlite_database_url: Option<String>) -> Result<Self, errors::Error> {
        let sqlite_conn_url = sqlite_database_url.unwrap_or_else(|| {
            std::env::var("SQLITE_DATABASE_URL")
                .unwrap_or_else(|_| "../data/db.sqlite3".to_string())
        });

        let sqlite_conn = SqlitePoolOptions::new()
            .max_connections(50)
            .connect(&sqlite_conn_url)
            .await?;
        Ok(Self { conn: sqlite_conn })
    }
}
