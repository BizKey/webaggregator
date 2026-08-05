use crate::domain::value_objects::{Money};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionAsset {
    pub exchange: String,
    pub asset_symbol: String,
    pub asset_total: Money,
    pub asset_available: Money,
    pub asset_hold: Money,
    pub updated_at: DateTime<Utc>,
}

impl PositionAsset {
    pub fn new(
        exchange: String,
        asset_symbol: String,
        asset_total: Money,
        asset_available: Money,
        asset_hold: Money,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            exchange,
            asset_symbol,
            asset_total,
            asset_available,
            asset_hold,
            updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionDebt {
    pub exchange: String,
    pub debt_symbol: String,
    pub debt_value: Money,
    pub updated_at: DateTime<Utc>,
}

impl PositionDebt {
    pub fn new(
        exchange: String,
        debt_symbol: String,
        debt_value: Money,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            exchange,
            debt_symbol,
            debt_value,
            updated_at,
        }
    }
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

impl PositionRatio {
    pub fn new(
        exchange: String,
        debt_ratio: f64,
        total_asset: f64,
        margin_coefficient_total_asset: String,
        total_debt: String,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            exchange,
            debt_ratio,
            total_asset,
            margin_coefficient_total_asset,
            total_debt,
            updated_at,
        }
    }
}
