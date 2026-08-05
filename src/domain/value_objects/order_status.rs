use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OrderStatus {
    Open,
    Filled,
    Cancelled,
    PartiallyFilled,
    Pending,
    Rejected,
    Expired,
}

impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Open => "open",
            Self::Filled => "filled",
            Self::Cancelled => "cancelled",
            Self::PartiallyFilled => "partially_filled",
            Self::Pending => "pending",
            Self::Rejected => "rejected",
            Self::Expired => "expired",
        };
        write!(f, "{}", s)
    }
}

impl From<String> for OrderStatus {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "open" => Self::Open,
            "filled" => Self::Filled,
            "cancelled" | "canceled" => Self::Cancelled,
            "partially_filled" | "partial" => Self::PartiallyFilled,
            "pending" => Self::Pending,
            "rejected" => Self::Rejected,
            "expired" => Self::Expired,
            _ => Self::Pending,
        }
    }
}
