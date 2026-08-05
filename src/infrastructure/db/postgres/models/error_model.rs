use crate::domain::entities::Error;
use crate::domain::value_objects::Exchange;
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct ErrorModel {
    pub exchange: String,
    pub msg: String,
    pub updated_at: DateTime<Utc>,
}

impl From<ErrorModel> for Error {
    fn from(model: ErrorModel) -> Self {
        Self {
            exchange: Exchange::new(model.exchange),
            msg: model.msg,
            updated_at: model.updated_at,
        }
    }
}
