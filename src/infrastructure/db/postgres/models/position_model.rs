use crate::domain::entities::{PositionAsset, PositionDebt, PositionRatio};
use crate::domain::value_objects::{Exchange, Money, SymbolName};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct PositionAssetModel {
    pub exchange: String,
    pub asset_symbol: String,
    pub asset_total: String,
    pub asset_available: String,
    pub asset_hold: String,
    pub updated_at: DateTime<Utc>,
}

impl From<PositionAssetModel> for PositionAsset {
    fn from(model: PositionAssetModel) -> Self {
        Self {
            exchange: Exchange::new(model.exchange),
            asset_symbol: SymbolName::new(model.asset_symbol),
            asset_total: Money::new(model.asset_total.parse().unwrap_or(0.0)),
            asset_available: Money::new(model.asset_available.parse().unwrap_or(0.0)),
            asset_hold: Money::new(model.asset_hold.parse().unwrap_or(0.0)),
            updated_at: model.updated_at,
        }
    }
}

#[derive(Debug, FromRow)]
pub struct PositionDebtModel {
    pub exchange: String,
    pub debt_symbol: String,
    pub debt_value: String,
    pub updated_at: DateTime<Utc>,
}

impl From<PositionDebtModel> for PositionDebt {
    fn from(model: PositionDebtModel) -> Self {
        Self {
            exchange: Exchange::new(model.exchange),
            debt_symbol: SymbolName::new(model.debt_symbol),
            debt_value: Money::new(model.debt_value.parse().unwrap_or(0.0)),
            updated_at: model.updated_at,
        }
    }
}

#[derive(Debug, FromRow)]
pub struct PositionRatioModel {
    pub exchange: String,
    pub debt_ratio: f64,
    pub total_asset: f64,
    pub margin_coefficient_total_asset: String,
    pub total_debt: String,
    pub updated_at: DateTime<Utc>,
}

impl From<PositionRatioModel> for PositionRatio {
    fn from(model: PositionRatioModel) -> Self {
        Self {
            exchange: Exchange::new(model.exchange),
            debt_ratio: model.debt_ratio,
            total_asset: model.total_asset,
            margin_coefficient_total_asset: model.margin_coefficient_total_asset,
            total_debt: model.total_debt,
            updated_at: model.updated_at,
        }
    }
}
