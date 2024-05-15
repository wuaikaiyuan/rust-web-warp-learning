mod answer;
mod pagination;
mod question;

pub type Question = question::Question;
pub type QuestionResponse = question::QuestionResponse;
pub type Pagination = pagination::Pagination;
pub type Answer = answer::Answer;
pub use pagination::extract_pagination;
pub use question::QuestionId;
