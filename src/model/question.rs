use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    pub id: QuestionId,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionId(pub String);

impl Question {
    pub fn new(
        id: QuestionId,
        title: String,
        content: String,
        tags: Option<Vec<String>>,
    ) -> Self {
        Self {
            id,
            title,
            content,
            tags,
        }
    }
}

impl std::str::FromStr for QuestionId {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "question id can not be empty",
            )),
        }
    }
}
