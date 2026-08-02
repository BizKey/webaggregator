use crate::application::dto::*;
use askama::Template;

#[derive(Template)]
#[template(path = "tickers.html")]
pub struct TickersTemplate {
    pub tickers: Vec<(usize, TickerDto)>,
    pub elapsed_ms: u128,
}

#[derive(Template)]
#[template(path = "symbols.html")]
pub struct SymbolsTemplate {
    pub symbols: Vec<(usize, SymbolDto)>,
    pub elapsed_ms: u128,
}

#[derive(Template)]
#[template(path = "currencies.html")]
pub struct CurrenciesTemplate {
    pub currencies: Vec<(usize, CurrencyDto)>,
    pub elapsed_ms: u128,
}

#[derive(Template)]
#[template(path = "balance/balance.html")]
pub struct BalanceTemplate {
    pub balances: Vec<BalanceDto>,
    pub elapsed_ms: u128,
}

#[derive(Template)]
#[template(path = "position/positionasset.html")]
pub struct PositionAssetTemplate {
    pub position_asset: Vec<PositionAssetDto>,
    pub elapsed_ms: u128,
}

#[derive(Template)]
#[template(path = "position/positiondebt.html")]
pub struct PositionDebtTemplate {
    pub position_debt: Vec<PositionDebtDto>,
    pub elapsed_ms: u128,
}

#[derive(Template)]
#[template(path = "position/positionratio.html")]
pub struct PositionRatioTemplate {
    pub position_ratio: Vec<PositionRatioDto>,
    pub elapsed_ms: u128,
}

#[derive(Template)]
#[template(path = "orders/eventorders.html")]
pub struct EventOrderTemplate {
    pub event_orders: Vec<EventOrderDto>,
    pub elapsed_ms: u128,
}

#[derive(Template)]
#[template(path = "bots/bots.html")]
pub struct BotsTemplate {
    pub bots: Vec<(usize, BotDto)>,
    pub init_balance: f64,
    pub final_balance: f64,
    pub elapsed_ms: u128,
}

#[derive(Template)]
#[template(path = "events/events.html")]
pub struct EventsTemplate {
    pub events: Vec<EventDto>,
    pub elapsed_ms: u128,
}

#[derive(Template)]
#[template(path = "events/msgevents.html")]
pub struct MsgEventTemplate {
    pub msgevents: Vec<MsgEventDto>,
    pub elapsed_ms: u128,
}

#[derive(Template)]
#[template(path = "events/msgsend.html")]
pub struct MsgSendTemplate {
    pub msgsend: Vec<MsgSendDto>,
    pub elapsed_ms: u128,
}

#[derive(Template)]
#[template(path = "errors/errors.html")]
pub struct ErrorsTemplate {
    pub errors: Vec<ErrorDto>,
    pub elapsed_ms: u128,
}

#[derive(Template)]
#[template(path = "pg/pg.html")]
pub struct PgTemplate {
    pub pg_stats_connections: Vec<PgConnectionDto>,
    pub pg_stats_table_info: Vec<PgTableInfoDto>,
    pub pg_stats_table_index: Vec<PgTableIndexDto>,
    pub pg_stat_statements: Vec<PgStatStatementsDto>,
    pub pg_stat_table_size: Vec<PgStatTableSizeDto>,
    pub elapsed_ms: u128,
}

#[derive(Template)]
#[template(path = "index/index.html")]
pub struct IndexTemplate {}
