use super::{
    data_scaffold::word_list::{WordList, WordListQueryBuilder},
    DbConnection,
};

use crate::errors;

impl DbConnection {
    pub async fn get_brief_word_info(
        &self,
        word_id: Option<i64>,
        word: Option<String>,
    ) -> Result<Vec<WordList>, errors::Error> {
        let q = WordListQueryBuilder::default()
            .id(word_id)
            .word(word)
            .build()?;

        let sql = format!("SELECT * FROM word_list {}", q.to_sql_condition());

        let words: Vec<WordList> = sqlx::query_as(&sql).fetch_all(&self.conn).await?;

        Ok(words)
    }
}
