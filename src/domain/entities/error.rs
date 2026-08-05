use crate::domain::value_objects::Exchange;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
    pub exchange: Exchange,
    pub msg: String,
    pub updated_at: DateTime<Utc>,
}

impl Error {
    pub fn new(exchange: Exchange, msg: String, updated_at: DateTime<Utc>) -> Self {
        Self {
            exchange,
            msg,
            updated_at,
        }
    }
}
