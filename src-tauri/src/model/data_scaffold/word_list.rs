use derive_builder::Builder;
use sqlx::FromRow;
use to_sql_condition::ToSqlCondition;

#[derive(Debug, Clone, FromRow)]
pub struct WordList {
    pub id: i64,
    pub word: String,
    pub paraphrase: String,
    pub classification: String, // 'CET-4', 'CET-6', 'TOEFL', 'AST', 'junior', 'senior', 'graduate', 'unknown'
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Default, Builder, ToSqlCondition)]
#[builder(setter(into), default)]
pub struct WordListQuery {
    pub id: Option<i64>,
    pub word: Option<String>,
    pub paraphrase: Option<String>,
    pub classification: Option<String>, // 'CET-4', 'CET-6', 'TOEFL', 'AST', 'junior', 'senior', 'graduate', 'unknown'
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}
