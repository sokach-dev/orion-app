use abi::QueryLearnWordRequest;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::errors;

use super::DbConnection;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WrapLearnWordResponse {
    pub id: i64,
    pub word: String,
    pub vocabulary_id: i64,
    pub word_list_id: i64,
    pub last_learned_at: String,
    pub next_learn_at: String,
    pub learn_status: i32,
}

impl From<abi::LearnWord> for WrapLearnWordResponse {
    fn from(word: abi::LearnWord) -> Self {
        Self {
            id: word.id,
            word: word.word,
            vocabulary_id: word.vocabulary_id,
            word_list_id: word.word_list_id,
            last_learned_at: word.last_learned_at,
            next_learn_at: word.next_learn_at,
            learn_status: word.learn_status,
        }
    }
}

impl DbConnection {
    pub async fn get_review_words(
        &self,
        dt: String,
    ) -> Result<Vec<WrapLearnWordResponse>, errors::Error> {
        let q = abi::LearnWordQueryBuilder::default()
            .next_learn_at(dt)
            .build()?;

        let request = tonic::Request::new(QueryLearnWordRequest { query: Some(q) });

        tracing::info!(
            "get_review_words: {:?}, url: {}",
            request,
            self.rpc_service_url.clone()
        );

        let mut conn = self.get_rpc_conn().await?;

        tracing::info!("get conn");

        let response = conn.query_learn_word(request).await?.into_inner();

        let words = response
            .word
            .into_iter()
            .map(|w| WrapLearnWordResponse::from(w))
            .collect();

        Ok(words)
    }
}
