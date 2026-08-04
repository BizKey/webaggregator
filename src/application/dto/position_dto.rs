use crate::domain::entities::{PositionAsset, PositionDebt, PositionRatio};
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct PositionAssetDto {
    pub exchange: String,
    pub asset_symbol: String,
    pub asset_total: String,
    pub asset_available: String,
    pub asset_hold: String,
    pub updated_at: DateTime<Utc>,
}

impl From<PositionAsset> for PositionAssetDto {
    fn from(position: PositionAsset) -> Self {
        Self {
            exchange: position.exchange.as_str().to_string(),
            asset_symbol: position.asset_symbol.as_str().to_string(),
            asset_total: position.asset_total,
            asset_available: position.asset_available,
            asset_hold: position.asset_hold,
            updated_at: position.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct PositionDebtDto {
    pub exchange: String,
    pub debt_symbol: String,
    pub debt_value: String,
    pub updated_at: DateTime<Utc>,
}

impl From<PositionDebt> for PositionDebtDto {
    fn from(position: PositionDebt) -> Self {
        Self {
            exchange: position.exchange.as_str().to_string(),
            debt_symbol: position.debt_symbol.as_str().to_string(),
            debt_value: position.debt_value,
            updated_at: position.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct PositionRatioDto {
    pub exchange: String,
    pub debt_ratio: f64,
    pub total_asset: f64,
    pub margin_coefficient_total_asset: String,
    pub total_debt: String,
    pub updated_at: DateTime<Utc>,
}

impl From<PositionRatio> for PositionRatioDto {
    fn from(position: PositionRatio) -> Self {
        Self {
            exchange: position.exchange.as_str().to_string(),
            debt_ratio: position.debt_ratio,
            total_asset: position.total_asset,
            margin_coefficient_total_asset: position.margin_coefficient_total_asset,
            total_debt: position.total_debt,
            updated_at: position.updated_at,
        }
    }
}
