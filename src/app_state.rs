use crate::repositories::{
    BalanceRepository, BotRepository, CurrencyRepository, ErrorRepository, EventOrderRepository,
    EventRepository, MsgEventRepository, MsgSendRepository, PgRepository, PositionRepository,
    PostgresBalanceRepository, PostgresBotRepository, PostgresCurrencyRepository,
    PostgresErrorRepository, PostgresEventOrderRepository, PostgresEventRepository,
    PostgresMsgEventRepository, PostgresMsgSendRepository, PostgresPgRepository,
    PostgresPositionRepository, PostgresSymbolRepository, PostgresTickerRepository,
    SymbolRepository, TickerRepository,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub balance_repo: Arc<dyn BalanceRepository>,
    pub ticker_repo: Arc<dyn TickerRepository>,
    pub symbol_repo: Arc<dyn SymbolRepository>,
    pub currency_repo: Arc<dyn CurrencyRepository>,
    pub event_repo: Arc<dyn EventRepository>,
    pub error_repo: Arc<dyn ErrorRepository>,
    pub order_repo: Arc<dyn EventOrderRepository>,
    pub position_repo: Arc<dyn PositionRepository>,
    pub bot_repo: Arc<dyn BotRepository>,
    pub msgevent_repo: Arc<dyn MsgEventRepository>,
    pub msgsend_repo: Arc<dyn MsgSendRepository>,
    pub pg_repo: Arc<dyn PgRepository>,
}

impl AppState {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self {
            balance_repo: Arc::new(PostgresBalanceRepository::new(pool.clone())),
            ticker_repo: Arc::new(PostgresTickerRepository::new(pool.clone())),
            symbol_repo: Arc::new(PostgresSymbolRepository::new(pool.clone())),
            currency_repo: Arc::new(PostgresCurrencyRepository::new(pool.clone())),
            event_repo: Arc::new(PostgresEventRepository::new(pool.clone())),
            error_repo: Arc::new(PostgresErrorRepository::new(pool.clone())),
            order_repo: Arc::new(PostgresEventOrderRepository::new(pool.clone())),
            position_repo: Arc::new(PostgresPositionRepository::new(pool.clone())),
            bot_repo: Arc::new(PostgresBotRepository::new(pool.clone())),
            msgevent_repo: Arc::new(PostgresMsgEventRepository::new(pool.clone())),
            msgsend_repo: Arc::new(PostgresMsgSendRepository::new(pool.clone())),
            pg_repo: Arc::new(PostgresPgRepository::new(pool.clone())),
        }
    }
}
