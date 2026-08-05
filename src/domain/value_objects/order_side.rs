use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OrderSide {
    Buy,
    Sell,
}

impl fmt::Display for OrderSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Buy => "buy",
            Self::Sell => "sell",
        };
        write!(f, "{}", s)
    }
}

impl From<String> for OrderSide {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "buy" => Self::Buy,
            "sell" => Self::Sell,
            _ => Self::Buy,
        }
    }
}
