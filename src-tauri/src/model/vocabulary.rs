use crate::errors;

use super::{
    data_scaffold::vocabulary::{Vocabulary, VocabularyQueryBuilder},
    DbConnection,
};

impl DbConnection {
    pub async fn get_vocabulary_info(
        &self,
        word_id: Option<i64>,
        word: Option<String>,
    ) -> Result<Vec<Vocabulary>, errors::Error> {
        let q = VocabularyQueryBuilder::default()
            .id(word_id)
            .word(word)
            .build()?;

        let sql = format!("SELECT * FROM vocabulary {}", q.to_sql_condition());

        let words: Vec<Vocabulary> = sqlx::query_as(&sql).fetch_all(&self.conn).await?;

        Ok(words)
    }
}
