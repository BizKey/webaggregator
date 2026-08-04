use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
    pub exchange: String,
    pub msg: String,
    pub updated_at: DateTime<Utc>,
}
