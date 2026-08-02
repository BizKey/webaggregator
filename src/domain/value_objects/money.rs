use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Money(pub f64);

impl Money {
    pub fn new(amount: f64) -> Self {
        Self(amount)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
