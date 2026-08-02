use crate::domain::entities::Error;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct ErrorDto {
    pub exchange: String,
    pub msg: String,
    pub updated_at: DateTime<Utc>,
}

impl From<Error> for ErrorDto {
    fn from(error: Error) -> Self {
        Self {
            exchange: error.exchange.as_str().to_string(),
            msg: error.msg,
            updated_at: error.updated_at,
        }
    }
}
