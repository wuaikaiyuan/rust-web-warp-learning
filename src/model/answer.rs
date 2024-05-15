use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, FromRow, Row};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Answer {
    pub id: u64,
    pub content: Option<String>,
    pub question_id: String,
}

impl<'a> FromRow<'a, MySqlRow> for Answer {
    fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.get("id"),
            content: row.try_get("content").ok(),
            question_id: row.get("question_id"),
        })
    }
}
