use super::{Question, QuestionResponse, Table};
use crate::{handler_error::handler::Error, utils::parse_tags};
use sqlx::{mysql::MySqlRow, Row};
use tracing::event;

impl<'a> Table<'a, Question> {
    pub async fn get_question_by_id(
        &self,
        id: i64,
    ) -> Result<Question, Error> {
        //match sqlx::query_as::<_, Question>(
        //    r#"
        //    SELECT * FROM question WHERE id = ?
        //    "#,
        //)
        //.bind(id)
        //.fetch_one(&*self.pool)
        //.await
        //{
        //    Ok(question) => Ok(question),
        //    Err(err) => {
        //        event!(tracing::Level::ERROR, "{:?}", err);
        //        Err(Error::DatabaseError(err))
        //    }
        //}

        match sqlx::query(
            r#"
            SELECT * FROM question WHERE id = ?
            "#,
        )
        .bind(id)
        .map(|row: MySqlRow| Question {
            id: Some(row.get("id")),
            title: row.get("title"),
            content: row.get("content"),
            tags: row.get("tags"),
            create_time: row.get("create_time"),
            status: row.get("status"),
        })
        .fetch_one(&*self.pool)
        .await
        {
            Ok(question) => Ok(question),
            Err(err) => {
                event!(tracing::Level::ERROR, "{:?}", err);
                Err(Error::DatabaseError(err))
            }
        }
    }

    pub async fn get_questions(
        &self,
        limit: Option<i32>,
        offset: i32,
    ) -> Result<Vec<QuestionResponse>, Error> {
        let limit = limit.unwrap_or(10);
        if limit < 0 {
            return Err(Error::InvalidLimit(String::from(
                "parameter [limit] must be greater than or equal to 0",
            )));
        }
        if offset < 0 {
            return Err(Error::InvalidOffset(String::from(
                "parameter [offset] must be greater than or equal to 0",
            )));
        }

        let questions = sqlx::query_as::<_, Question>(
            r#"
                SELECT * FROM question LIMIT ? OFFSET ?
                "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&*self.pool)
        .await
        .map_err(|err: sqlx::Error| Error::DatabaseError(err))?;

        let questions = questions
            .into_iter()
            .map(|q| {
                let tags_ = if let Some(tags) = q.tags {
                    parse_tags(&tags)
                } else {
                    None
                };

                QuestionResponse {
                    id: q.id,
                    title: q.title,
                    content: q.content,
                    tags: tags_,
                    createTime: q.create_time,
                    status: q.status,
                }
            })
            .collect();

        Ok(questions)
    }

    pub async fn get_question_by_model(
        &self,
        question: &Question,
    ) -> Result<Question, Error> {
        let question = sqlx::query_as::<_, Question>(
            r#"
            SELECT * FROM question WHERE id = ? and title = ? and content = ? and tags = ?
            "#,
        )
        .bind(question.id)
        .bind(&question.title)
        .bind(&question.content)
        .bind(&question.tags)
        .fetch_one(&*self.pool)
        .await
        .map_err(|err| {
            event!(tracing::Level::ERROR, "{:?}", err);
            Error::DatabaseError(err)
        })?;

        Ok(question)
    }

    pub async fn add_question(
        &self,
        question: &Question,
    ) -> Result<Question, Error> {
        match sqlx::query(
            r#"
            INSERT INTO question (`title`, `content`, `tags`) VALUES (?, ?, ?)
            "#,
        )
        .bind(&question.title)
        .bind(&question.content)
        .bind(&question.tags)
        .execute(&*self.pool)
        .await
        {
            Ok(res) => {
                let last_insert_id = res.last_insert_id() as i64;
                event!(tracing::Level::INFO, "last_insert_id: {:?}", last_insert_id);
                let q = Question {
                    id: Some(last_insert_id),
                    title: question.title.clone(),
                    content: question.content.clone(),
                    tags: question.tags.clone(),
                    create_time: None,
                    status: question.status,
                };
                event!(tracing::Level::INFO, "question added return: {:?}", q);
                Ok(q)
            },
            Err(err) => {
                event!(tracing::Level::ERROR, "add_question failed: {:?}", err);
                Err(Error::DatabaseError(err))
            }
        }
    }

    // 更新问题
    pub async fn update_question(
        &self,
        question: &Question,
    ) -> Result<Question, Error> {
        match sqlx::query(
            r#"
            UPDATE question SET title = ?, content = ?, tags = ? WHERE id = ?
            "#,
        )
        .bind(&question.title)
        .bind(&question.content)
        .bind(&question.tags)
        .bind(question.id)
        .execute(&*self.pool)
        .await
        {
            Ok(result) => {
                if result.rows_affected() == 0 {
                    return Err(Error::UpdateError(String::from("update failed: update_question")));
                }
                Ok(question.clone())
            }
            Err(err) => Err(Error::DatabaseError(err)),
        }
    }

    // 永久删除
    pub async fn delete_question_permanently(
        &self,
        id: i64,
    ) -> Result<u64, Error> {
        sqlx::query(
            r#"
                DELETE FROM question WHERE id = ?
                "#,
        )
        .bind(id)
        .execute(&*self.pool)
        .await
        .map_err(|err: sqlx::Error| Error::DatabaseError(err))
        .map(|res| res.rows_affected())
    }

    // 逻辑删除
    pub async fn delete_question_logic(
        &self,
        id: i64,
    ) -> Result<u64, Error> {
        sqlx::query(
            r#"
                UPDATE `question` SET `status` = 0 WHERE `id` = ?
                "#,
        )
        .bind(id)
        .execute(&*self.pool)
        .await
        .map_err(|err: sqlx::Error| Error::DatabaseError(err))
        .map(|res| res.rows_affected())
    }
}
