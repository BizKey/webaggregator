// src/repositories/mod.rs
pub mod balance_repository;
pub mod bot_repository;
pub mod currency_repository;
pub mod error_repository;
pub mod event_repository;
pub mod msgevent_repository;
pub mod msgsend_repository;
pub mod order_repository;
pub mod pg_repository;
pub mod position_repository;
pub mod symbol_repository;
pub mod ticker_repository;

pub use balance_repository::{BalanceRepository, PostgresBalanceRepository};
pub use bot_repository::{BotRepository, PostgresBotRepository};
pub use currency_repository::{CurrencyRepository, PostgresCurrencyRepository};
pub use error_repository::{ErrorRepository, PostgresErrorRepository};
pub use event_repository::{EventRepository, PostgresEventRepository};
pub use msgevent_repository::{MsgEventRepository, PostgresMsgEventRepository};
pub use msgsend_repository::{MsgSendRepository, PostgresMsgSendRepository};
pub use order_repository::{EventOrderRepository, PostgresEventOrderRepository};
pub use pg_repository::{
    ConnectionStatsRepository, PostgresPgRepository, QueryStatsRepository, TableSizeRepository,
    TableStatsRepository,
};
pub use position_repository::{PositionRepository, PostgresPositionRepository};
pub use symbol_repository::{PostgresSymbolRepository, SymbolRepository};
pub use ticker_repository::{PostgresTickerRepository, TickerRepository};

use anyhow::Result;
pub type RepositoryResult<T> = Result<T, anyhow::Error>;
