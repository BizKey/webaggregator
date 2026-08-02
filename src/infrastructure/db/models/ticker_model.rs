use crate::domain::entities::Ticker;
use crate::domain::value_objects::{Exchange, Symbol, Percentage};
use crate::domain::repositories::RepositoryError;
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct TickerModel {
    pub exchange: String,
    pub symbol: String,
    pub symbol_name: String,
    pub taker_fee_rate: Option<f64>,
    pub maker_fee_rate: Option<f64>,
    pub taker_coefficient: Option<f64>,
    pub maker_coefficient: Option<f64>,
    pub updated_at: DateTime<Utc>,
}

impl TryFrom<TickerModel> for Ticker {
    type Error = RepositoryError;

    fn try_from(model: TickerModel) -> Result<Self, Self::Error> {
        Ok(Self {
            exchange: Exchange::new(model.exchange)
                .map_err(|e| RepositoryError::Validation(e))?,
            symbol: Symbol::new(model.symbol)
                .map_err(|e| RepositoryError::Validation(e))?,
            symbol_name: model.symbol_name,
            taker_fee_rate: model.taker_fee_rate
                .map(Percentage::new)
                .transpose()
                .map_err(|e| RepositoryError::Validation(e))?,
            maker_fee_rate: model.maker_fee_rate
                .map(Percentage::new)
                .transpose()
                .map_err(|e| RepositoryError::Validation(e))?,
            taker_coefficient: model.taker_coefficient,
            maker_coefficient: model.maker_coefficient,
            updated_at: model.updated_at,
        })
    }
}