// use crate::{
//     handler_error::handler::Error,
//     model::{Question, QuestionResponse},
// };
// use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
//
// // 1. 定义默认值常量
// const DEFAULT_LIMIT: i32 = 10;
//
// #[derive(Debug, Clone)]
// pub struct Store {
//     pub connection: MySqlPool,
// }
//
// impl Store {
//     // DATABASE_URL="mysql://root:123456@localhost:3306/mqtt"
//     pub async fn new(db_url: &str) -> Self {
//         let db_pool = match MySqlPoolOptions::new()
//             .max_connections(10)
//             .connect(db_url)
//             .await
//         {
//             Ok(pool) => pool,
//             Err(err) => panic!("DB Connection Error: {}", err),
//         };
//
//         Store {
//             connection: db_pool,
//         }
//     }
//
//     pub async fn get_questions(
//         &self,
//         limit: Option<i32>,
//         offset: i32,
//     ) -> Result<Vec<QuestionResponse>, Error> {
//         // 验证 limit 和 offset，确保它们是合理的值
//         let limit = limit.unwrap_or(DEFAULT_LIMIT);
//         if limit < 0 {
//             return Err(Error::InvalidLimit(String::from(
//                 "parameter [limit] must be greater than or equal to 0",
//             )));
//         }
//         if offset < 0 {
//             return Err(Error::InvalidOffset(String::from(
//                 "parameter [offset] must be greater than or equal to 0",
//             )));
//         }
//
//         let questions: Vec<Question> = sqlx::query_as!(
//             Question,
//             "select id, title, content, tags, create_time from question
// limit ? offset ?",             limit,
//             offset
//         )
//         .fetch_all(&self.connection)
//         .await
//         .map_err(|err| Error::DatabaseError(err))?;
//
//         let res = questions
//             .into_iter()
//             .map(|q| {
//                 let tags = if let Some(tags) = q.tags {
//                     let tags = tags
//                         .split(",")
//                         .map(|s| s.trim())
//                         .collect::<Vec<&str>>()
//                         .iter()
//                         .map(|tag| tag.to_string())
//                         .collect::<Vec<String>>();
//
//                     Some(tags)
//                 } else {
//                     None
//                 };
//                 QuestionResponse {
//                     id: q.id.0,
//                     title: q.title,
//                     content: q.content,
//                     tags: tags,
//                     createTime: q.create_time,
//                     status: q.status,
//                 }
//             })
//             .collect::<Vec<QuestionResponse>>();
//
//         Ok(res)
//     }
// }
//
// #[cfg(test)]
// mod test {
//
//     use super::*;
//
//     const DATABASE_URL: &str = "mysql://root:123456@localhost:3306/mqtt";
//
//     #[tokio::test]
//     async fn get_question_test() {
//         let store = Store::new(DATABASE_URL).await;
//
//         let questions = store.get_questions(Some(10), 0).await.unwrap();
//
//         println!("{:?}", questions);
//     }
// }
