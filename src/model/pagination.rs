use std::collections::HashMap;

use serde::Serialize;

use crate::handler_error::handler::Error;

#[derive(Debug, Default, PartialEq, Serialize)]
pub struct Pagination {
    pub limit: Option<i32>,
    pub offset: i32,
}

pub fn extract_pagination(
    param: HashMap<String, String>,
) -> Result<Pagination, Error> {
    if param.contains_key("limit") && param.contains_key("offset") {
        return Ok(Pagination {
            limit: Some(
                param
                    .get("limit")
                    .unwrap()
                    .parse::<i32>()
                    .map_err(Error::ParseError)?,
            ),
            offset: param
                .get("offset")
                .unwrap()
                .parse::<i32>()
                .map_err(Error::ParseError)?,
        });
    }

    Err(Error::MissingParamError)
}
