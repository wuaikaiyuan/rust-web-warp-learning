use super::model::{Answer, Question, QuestionResponse};

pub mod db_context;
pub mod question;

pub type DataBase<'a> = db_context::DataBase<'a>;
pub type Table<'a, T> = db_context::Table<'a, T>;
pub type JoinTable<'a, T1, T2> = db_context::JoinTable<'a, T1, T2>;
