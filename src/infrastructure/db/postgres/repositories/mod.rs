use sqlx::PgPool;
mod balance_repository_impl;
mod bot_repository_impl;
mod currency_repository_impl;
mod error_repository_impl;
mod event_repository_impl;
mod order_repository_impl;
mod pg_stat_repository_impl;
mod position_repository_impl;
mod symbol_repository_impl;
mod ticker_repository_impl;

pub use balance_repository_impl::PostgresBalanceRepository;
pub use bot_repository_impl::PostgresBotRepository;
pub use currency_repository_impl::PostgresCurrencyRepository;
pub use error_repository_impl::PostgresErrorRepository;
pub use event_repository_impl::PostgresEventRepository;
pub use order_repository_impl::PostgresOrderRepository;
pub use pg_stat_repository_impl::PostgresPgStatRepository;
pub use position_repository_impl::PostgresPositionRepository;
pub use symbol_repository_impl::PostgresSymbolRepository;
pub use ticker_repository_impl::PostgresTickerRepository;

pub struct Repositories {
    pub balance: PostgresBalanceRepository,
    pub bot: PostgresBotRepository,
    pub currency: PostgresCurrencyRepository,
    pub error: PostgresErrorRepository,
    pub event: PostgresEventRepository,
    pub order: PostgresOrderRepository,
    pub pg_stat: PostgresPgStatRepository,
    pub position: PostgresPositionRepository,
    pub symbol: PostgresSymbolRepository,
    pub ticker: PostgresTickerRepository,
}

impl Repositories {
    pub fn new(pool: PgPool) -> Self {
        Self {
            balance: PostgresBalanceRepository::new(pool.clone()),
            bot: PostgresBotRepository::new(pool.clone()),
            currency: PostgresCurrencyRepository::new(pool.clone()),
            error: PostgresErrorRepository::new(pool.clone()),
            event: PostgresEventRepository::new(pool.clone()),
            order: PostgresOrderRepository::new(pool.clone()),
            pg_stat: PostgresPgStatRepository::new(pool.clone()),
            position: PostgresPositionRepository::new(pool.clone()),
            symbol: PostgresSymbolRepository::new(pool.clone()),
            ticker: PostgresTickerRepository::new(pool.clone()),
        }
    }
}
