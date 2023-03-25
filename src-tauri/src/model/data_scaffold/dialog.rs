use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use to_sql_condition::ToSqlCondition;

#[derive(Debug, Clone, Default, FromRow, Serialize, Deserialize)]
pub struct Dialog {
    pub id: i64,
    pub person: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Default, Builder, ToSqlCondition)]
#[builder(setter(into), default)]
pub struct DialogQuery {
    pub id: Option<i64>,
    pub person: Option<String>,
    pub content: Option<String>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}
