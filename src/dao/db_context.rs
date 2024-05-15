use sqlx::{
    mysql::{MySqlPoolOptions, MySqlRow},
    FromRow, MySqlPool,
};
use std::{marker::PhantomData, sync::Arc};

use super::{Answer, Question};

#[derive(Debug)]
pub struct Table<'a, T>
where
    T: FromRow<'a, MySqlRow>,
{
    pub pool: Arc<MySqlPool>,
    _from_row: fn(&'a MySqlRow) -> Result<T, sqlx::Error>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Table<'a, T>
where
    T: FromRow<'a, MySqlRow>,
{
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        Table {
            pool,
            _from_row: T::from_row,
            _marker: PhantomData,
        }
    }
}

pub struct JoinTable<'a, T1, T2>
where
    T1: FromRow<'a, MySqlRow>,
    T2: FromRow<'a, MySqlRow>,
{
    pub pool: Arc<MySqlPool>,
    #[allow(clippy::type_complexity)]
    _from_row: (
        fn(&'a MySqlRow) -> Result<T1, sqlx::Error>,
        fn(&'a MySqlRow) -> Result<T2, sqlx::Error>,
    ),
    _marker_t1: PhantomData<&'a T1>,
    _marker_t2: PhantomData<&'a T2>,
}

impl<'a, T1, T2> JoinTable<'a, T1, T2>
where
    T1: FromRow<'a, MySqlRow>,
    T2: FromRow<'a, MySqlRow>,
{
    #[allow(dead_code)]
    fn new(pool: Arc<MySqlPool>) -> Self {
        JoinTable {
            pool,
            _from_row: (T1::from_row, T2::from_row),
            _marker_t1: PhantomData,
            _marker_t2: PhantomData,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DataBase<'a> {
    pub questions: Arc<Table<'a, Question>>,
    pub answers: Arc<Table<'a, Answer>>,
    // TODO: Add more tables here
}

impl<'a> DataBase<'a> {
    pub async fn new(db_url: &str) -> DataBase<'a> {
        let db_pool = match MySqlPoolOptions::new()
            .max_connections(10)
            .connect(db_url)
            .await
        {
            Ok(pool) => Arc::new(pool),
            Err(err) => panic!("DB Connection Error: {}", err),
        };

        DataBase {
            questions: Arc::new(Table::new(db_pool.clone())),
            answers: Arc::new(Table::new(db_pool.clone())),
        }
    }
}
