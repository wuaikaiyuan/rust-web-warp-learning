#[allow(dead_code)]
#[derive(Debug)]
pub struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct QuestionId(String);

impl Question {
    #[allow(dead_code)]
    fn new(
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
