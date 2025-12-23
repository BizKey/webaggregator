use crate::models::{
    ActiveOrder, Balance, Currency, Error, Event, EventOrder, MsgEvent, MsgSend, PgConnection,
    PgStatStatements, PgStatTableSize, PgTableIndex, PgTableInfo, PositionAsset, PositionDebt,
    PositionRatio, Symbol, Ticker, TradeSymbol,
};
use askama::Template;

// Tickers template
#[derive(Template)]
#[template(path = "tickers.html")]
pub struct TickersTemplate {
    pub tickers: Vec<(usize, Ticker)>,
    pub elapsed_ms: u128,
}
// Symbols template
#[derive(Template)]
#[template(path = "symbols.html")]
pub struct SymbolsTemplate {
    pub symbols: Vec<(usize, Symbol)>,
    pub elapsed_ms: u128,
}
// Trade Symbol template
#[derive(Template)]
#[template(path = "trade_symbols.html")]
pub struct TradeSymbolTemplate {
    pub symbols: Vec<(usize, TradeSymbol)>,
    pub elapsed_ms: u128,
}
// Currency template
#[derive(Template)]
#[template(path = "currencies.html")]
pub struct CurrenciesTemplate {
    pub currencies: Vec<(usize, Currency)>,
    pub elapsed_ms: u128,
}
// pg stats
#[derive(Template)]
#[template(path = "pg/pg.html")]
pub struct PgTemplate {
    pub pg_stats_connections: Vec<PgConnection>,
    pub pg_stats_table_info: Vec<PgTableInfo>,
    pub pg_stats_table_index: Vec<PgTableIndex>,
    pub pg_stat_statements: Vec<PgStatStatements>,
    pub pg_stat_table_size: Vec<PgStatTableSize>,
    pub elapsed_ms: u128,
}
#[derive(Template)]
#[template(path = "events/events.html")]
pub struct EventsTemplate {
    pub events: Vec<Event>,
    pub elapsed_ms: u128,
}
#[derive(Template)]
#[template(path = "errors/errors.html")]
pub struct ErrorsTemplate {
    pub errors: Vec<Error>,
    pub elapsed_ms: u128,
}
#[derive(Template)]
#[template(path = "events/msgevents.html")]
pub struct MsgEventTemplate {
    pub msgevents: Vec<MsgEvent>,
    pub elapsed_ms: u128,
}
#[derive(Template)]
#[template(path = "events/msgsend.html")]
pub struct MsgSendTemplate {
    pub msgsend: Vec<MsgSend>,
    pub elapsed_ms: u128,
}
#[derive(Template)]
#[template(path = "position/positionratio.html")]
pub struct PositinRatioTemplate {
    pub position_ratio: Vec<PositionRatio>,
    pub elapsed_ms: u128,
}
#[derive(Template)]
#[template(path = "position/positiondebt.html")]
pub struct PositionDebtTemplate {
    pub position_debt: Vec<PositionDebt>,
    pub elapsed_ms: u128,
}
#[derive(Template)]
#[template(path = "position/positionasset.html")]
pub struct PositionAssetTemplate {
    pub position_asset: Vec<PositionAsset>,
    pub elapsed_ms: u128,
}
#[derive(Template)]
#[template(path = "orders/activeorders.html")]
pub struct ActiveOrderTemplate {
    pub active_orders: Vec<ActiveOrder>,
    pub elapsed_ms: u128,
}
#[derive(Template)]
#[template(path = "orders/eventorders.html")]
pub struct EventOrderTemplate {
    pub event_orders: Vec<EventOrder>,
    pub elapsed_ms: u128,
}
#[derive(Template)]
#[template(path = "balance/balance.html")]
pub struct BalanceTemplate {
    pub balances: Vec<Balance>,
    pub elapsed_ms: u128,
}
// Index template
#[derive(Template)]
#[template(path = "index/index.html")]
pub struct IndexTemplate {}
