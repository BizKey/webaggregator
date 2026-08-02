mod balance_service;
mod bot_service;
mod currency_service;
mod error_service;
mod event_service;
mod order_service;
mod pg_stat_service;
mod position_service;
mod symbol_service;
mod ticker_service;

pub use balance_service::BalanceService;
pub use bot_service::BotService;
pub use currency_service::CurrencyService;
pub use error_service::ErrorService;
pub use event_service::EventService;
pub use order_service::OrderService;
pub use pg_stat_service::PgStatService;
pub use position_service::PositionService;
pub use symbol_service::SymbolService;
pub use ticker_service::TickerService;

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Repository error: {0}")]
    Repository(String),
    #[error("Not found")]
    NotFound,
}
