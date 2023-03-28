use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use to_sql_condition::ToSqlCondition;

#[derive(Debug, Clone, Default, FromRow, Deserialize, Serialize)]
pub struct Vocabulary {
    pub id: i64,
    pub word: String,
    pub soundmark: String,
    pub roots: String,
    pub paraphrase: String,
    pub collocations: String,
    pub synonyms: String,
    pub examples: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Default, Builder, ToSqlCondition)]
#[builder(setter(into), default)]
pub struct VocabularyQuery {
    pub id: Option<i64>,
    pub word: Option<String>,
    pub soundmark: Option<String>,
    pub roots: Option<String>,
    pub paraphrase: Option<String>,
    pub collocations: Option<String>,
    pub synonyms: Option<String>,
    pub examples: Option<String>,
    pub offset: Option<i32>,
    pub limit: Option<i32>,
}
