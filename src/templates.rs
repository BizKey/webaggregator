use crate::models::{
    Currency, Error, Event, PgConnection, PgStatStatements, PgStatTableSize, PgTableIndex,
    PgTableInfo, Symbol, Ticker,
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
// Index template
#[derive(Template)]
#[template(path = "index/index.html")]
pub struct IndexTemplate {}
