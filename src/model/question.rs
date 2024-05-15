use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, FromRow, Row};
// sqlx::FrowRow
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[allow(non_snake_case)]
pub struct Question {
    // #[serde(skip_deserializing)]
    // #[serde(skip)]
    pub id: Option<i64>,
    pub title: String,
    pub content: Option<String>,
    pub tags: Option<String>,
    pub create_time: Option<chrono::NaiveDateTime>,
    pub status: Option<u8>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QuestionId(pub i64);

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct QuestionResponse {
    pub id: Option<i64>,
    pub title: String,
    pub content: Option<String>,
    pub tags: Option<Vec<String>>,
    pub createTime: Option<chrono::NaiveDateTime>,
    pub status: Option<u8>,
}

impl Question {
    pub fn new(
        id: Option<i64>,
        title: String,
        content: Option<String>,
        tags: Option<String>,
        create_time: Option<chrono::NaiveDateTime>,
        status: Option<u8>,
    ) -> Self {
        Self {
            id,
            title,
            content,
            tags,
            create_time,
            status,
        }
    }
}

impl<'a> FromRow<'a, MySqlRow> for Question {
    fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.get("id"),
            title: row.get("title"),
            content: row.get("content"),
            tags: row.get("tags"),
            create_time: row.get("create_time"),
            status: row.get("status"),
        })
    }
}
