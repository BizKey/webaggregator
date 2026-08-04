use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionAsset {
    pub exchange: String,
    pub asset_symbol: String,
    pub asset_total: String,
    pub asset_available: String,
    pub asset_hold: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionDebt {
    pub exchange: String,
    pub debt_symbol: String,
    pub debt_value: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionRatio {
    pub exchange: String,
    pub debt_ratio: f64,
    pub total_asset: f64,
    pub margin_coefficient_total_asset: String,
    pub total_debt: String,
    pub updated_at: DateTime<Utc>,
}
