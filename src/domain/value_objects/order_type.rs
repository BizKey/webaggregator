use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OrderType {
    Limit,
    Market,
    Stop,
    StopLimit,
}

impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Limit => "limit",
            Self::Market => "market",
            Self::Stop => "stop",
            Self::StopLimit => "stop_limit",
        };
        write!(f, "{}", s)
    }
}

impl From<String> for OrderType {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "limit" => Self::Limit,
            "market" => Self::Market,
            "stop" => Self::Stop,
            "stop_limit" => Self::StopLimit,
            _ => Self::Limit,
        }
    }
}
