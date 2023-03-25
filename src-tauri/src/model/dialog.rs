use super::{data_scaffold::dialog::Dialog, DbConnection};

use crate::errors;

impl DbConnection {
    pub async fn add_new_dialog(
        &self,
        person: String,
        content: String,
    ) -> Result<(), errors::Error> {
        let sql = format!("INSERT INTO dialog (person, content) VALUES (?, ?)");

        sqlx::query(&sql)
            .bind(person)
            .bind(content)
            .execute(&self.conn)
            .await?;

        Ok(())
    }

    pub async fn get_dialog(
        &self,
        page_size: u32,
        page_num: u32,
    ) -> Result<Vec<Dialog>, errors::Error> {
        let sql = format!(
            "SELECT * FROM dialog ORDER BY id DESC LIMIT {}, {}",
            page_size * page_num,
            page_size
        );

        let dialogs: Vec<Dialog> = sqlx::query_as(&sql).fetch_all(&self.conn).await?;

        Ok(dialogs)
    }
}
