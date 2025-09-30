use crate::models::{Borrow, Currency, Lend, Symbol, Ticker};
use askama::Template;
use chrono::{DateTime, Utc};

pub trait DateTimeFormat {
    fn format_date(&self) -> String;
}

impl DateTimeFormat for DateTime<Utc> {
    fn format_date(&self) -> String {
        self.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}

#[derive(Debug)]
pub struct DvaResult {
    pub commission_rate: f64,
    pub periods: usize,
    pub total_gross_spent: String,
    pub total_gross_received: String,
    pub net_invested: String,
    pub final_asset_amount: String,
    pub bnh: String,
    pub final_price: String,
    pub start_price: String,
    pub final_value: String,
    pub target_increment: String,
    pub profit: String,
    pub roi: String,
}

// Tickers template
#[derive(Template)]
#[template(path = "dva/dvas.html")]
pub struct DvasTemplate {
    pub tickers: Vec<(usize, Ticker)>,
    pub elapsed_ms: u128,
}
#[derive(Template)]
#[template(path = "dva/dva.html")]
pub struct DvaTemplate {
    pub elapsed_ms: u128,
    pub ticker: String,
    pub data: DvaResult,
}
#[derive(Template)]
#[template(path = "ticker/ticker.html")]
pub struct TickerTemplate {
    pub tickers: Vec<(usize, Ticker)>,
    pub chart_series: Vec<f64>,
    pub elapsed_ms: u128,
}
#[derive(Template)]
#[template(path = "ticker/tickers.html")]
pub struct TickersTemplate {
    pub tickers: Vec<(usize, Ticker)>,
    pub elapsed_ms: u128,
}
//
// Symbols template
#[derive(Template)]
#[template(path = "symbol/symbol.html")]
pub struct SymbolTemplate {
    pub symbols: Vec<Symbol>,
    pub elapsed_ms: u128,
}
#[derive(Template)]
#[template(path = "symbol/symbols.html")]
pub struct SymbolsTemplate {
    pub symbols: Vec<(usize, Symbol)>,
    pub elapsed_ms: u128,
}
//
// Currency template
#[derive(Template)]
#[template(path = "currency/currency.html")]
pub struct CurrencyTemplate {
    pub current_currency: Vec<(usize, Currency)>,
    pub elapsed_ms: u128,
}
#[derive(Template)]
#[template(path = "currency/currencies.html")]
pub struct CurrenciesTemplate {
    pub currencies: Vec<(usize, Currency)>,
    pub elapsed_ms: u128,
}
//
// Borrow template
#[derive(Template)]
#[template(path = "borrow/borrow.html")]
pub struct BorrowTemplate {
    pub borrows: Vec<(usize, Borrow)>,
    pub elapsed_ms: u128,
}
#[derive(Template)]
#[template(path = "borrow/borrows.html")]
pub struct BorrowsTemplate {
    pub borrows: Vec<(usize, Borrow)>,
    pub elapsed_ms: u128,
}
//
// Lend template
#[derive(Template)]
#[template(path = "lend/lend.html")]
pub struct LendTemplate {
    pub lends: Vec<(usize, Lend)>,
    pub elapsed_ms: u128,
}
#[derive(Template)]
#[template(path = "lend/lends.html")]
pub struct LendsTemplate {
    pub lends: Vec<(usize, Lend)>,
    pub elapsed_ms: u128,
}
//
// Index template
#[derive(Template)]
#[template(path = "index/index.html")]
pub struct IndexTemplate {}
