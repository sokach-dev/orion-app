use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use to_sql_condition::ToSqlCondition;

#[derive(Debug, Clone, Default, FromRow, Serialize, Deserialize)]
pub struct LearnWord {
    pub id: i64,
    pub word: String,
    pub vocabulary_id: i64,
    pub paraphrase: String,
    pub learn_count: i64,
    pub next_learn_at: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Default, Builder, ToSqlCondition)]
#[builder(setter(into), default)]
pub struct LearnWordQuery {
    pub id: Option<i64>,
    pub word: Option<String>,
    pub vocabulary_id: Option<i64>,
    pub paraphrase: Option<String>,
    pub learn_count: Option<i64>,
    pub next_learn_at: Option<String>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}
