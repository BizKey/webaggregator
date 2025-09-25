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

// Tickers template
#[derive(Template)]
#[template(path = "dvas.html")]
pub struct DvasTemplate {
    pub tickers: Vec<(usize, Ticker)>,
    pub elapsed_ms: u128,
}
#[derive(Template)]
#[template(path = "dva.html")]
pub struct DvaTemplate {
    pub elapsed_ms: u128,
    pub data: DvaResult,
}
#[derive(Template)]
#[template(path = "ticker.html")]
pub struct TickerTemplate {
    pub tickers: Vec<(usize, Ticker)>,
    pub chart_series: Vec<f64>,
    pub elapsed_ms: u128,
}
#[derive(Template)]
#[template(path = "tickers.html")]
pub struct TickersTemplate {
    pub tickers: Vec<(usize, Ticker)>,
    pub elapsed_ms: u128,
}
//
// Symbols template
#[derive(Template)]
#[template(path = "symbol.html")]
pub struct SymbolTemplate {
    pub symbols: Vec<Symbol>,
    pub elapsed_ms: u128,
}
#[derive(Template)]
#[template(path = "symbols.html")]
pub struct SymbolsTemplate {
    pub symbols: Vec<(usize, Symbol)>,
    pub elapsed_ms: u128,
}
//
// Currency template
#[derive(Template)]
#[template(path = "currency.html")]
pub struct CurrencyTemplate {
    pub current_currency: Vec<(usize, Currency)>,
    pub elapsed_ms: u128,
}
#[derive(Template)]
#[template(path = "currencies.html")]
pub struct CurrenciesTemplate {
    pub currencies: Vec<(usize, Currency)>,
    pub elapsed_ms: u128,
}
//
// Borrow template
#[derive(Template)]
#[template(path = "borrow.html")]
pub struct BorrowTemplate {
    pub borrows: Vec<(usize, Borrow)>,
    pub elapsed_ms: u128,
}
#[derive(Template)]
#[template(path = "borrows.html")]
pub struct BorrowsTemplate {
    pub borrows: Vec<(usize, Borrow)>,
    pub elapsed_ms: u128,
}
//
// Lend template
#[derive(Template)]
#[template(path = "lend.html")]
pub struct LendTemplate {
    pub lends: Vec<(usize, Lend)>,
    pub elapsed_ms: u128,
}
#[derive(Template)]
#[template(path = "lends.html")]
pub struct LendsTemplate {
    pub lends: Vec<(usize, Lend)>,
    pub elapsed_ms: u128,
}
//
// Index template
#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}
