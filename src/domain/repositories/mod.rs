mod balance_repository;
mod bot_repository;
mod currency_repository;
mod error_repository;
mod event_repository;
mod order_repository;
mod pg_stat_repository;
mod position_repository;
mod symbol_repository;
mod ticker_repository;

pub use balance_repository::*;
pub use bot_repository::*;
pub use currency_repository::*;
pub use error_repository::*;
pub use event_repository::*;
pub use order_repository::*;
pub use pg_stat_repository::*;
pub use position_repository::*;
pub use symbol_repository::*;
pub use ticker_repository::*;

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    Database(String),
}
