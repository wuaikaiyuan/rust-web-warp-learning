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
    if !param.contains_key("limit") {
        return Err(Error::MissingParamError("limit".to_string()));
    }

    if !param.contains_key("offset") {
        return Err(Error::MissingParamError("offset".to_string()));
    }

    Ok(Pagination {
        limit: Some(param.get("limit").unwrap().parse::<i32>().map_err(
            |_err| {
                Error::ParseError("limit must not be int type".to_string())
            },
        )?),
        offset: param.get("offset").unwrap().parse::<i32>().map_err(
            |_err| {
                Error::ParseError(String::from(
                    "offset must not be int type",
                ))
            },
        )?,
    })
}
