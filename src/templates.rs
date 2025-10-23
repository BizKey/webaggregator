use crate::models::{
    Borrow, Candle, CandleWithAtr, CandleWithProfit, Currency, Lend, Strategy, Symbol, Ticker,
    Total,
};
use askama::Template;

//
//
// Tickers template
//
//
#[derive(Template)]
#[template(path = "tickers.html")]
pub struct TickersTemplate {
    pub tickers: Vec<(usize, Ticker)>,
    pub elapsed_ms: u128,
}
//
//
// Symbols template
//
//
#[derive(Template)]
#[template(path = "symbols.html")]
pub struct SymbolsTemplate {
    pub symbols: Vec<(usize, Symbol)>,
    pub elapsed_ms: u128,
}
//
//
// Currency template
//
//
#[derive(Template)]
#[template(path = "currencies.html")]
pub struct CurrenciesTemplate {
    pub currencies: Vec<(usize, Currency)>,
    pub elapsed_ms: u128,
}
//
//
// Borrow template
//
//
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
//
//
//
//
#[derive(Template)]
#[template(path = "strategy/strategy.html")]
pub struct StrategyTemplate {
    pub candles: Vec<(usize, CandleWithProfit)>,
    pub total_profit: f64,
    pub elapsed_ms: u128,
}
#[derive(Template)]
#[template(path = "strategy/onestrategy.html")]
pub struct OneStrategyTemplate {
    pub candles: Vec<Strategy>,
    pub total: Total,
    pub elapsed_ms: u128,
}
//
//
// Candle template
//
//
#[derive(Template)]
#[template(path = "candle/candles.html")]
pub struct CandlesTemplate {
    pub candles: Vec<(usize, Candle)>,
    pub elapsed_ms: u128,
}
#[derive(Template)]
#[template(path = "candle/candle.html")]
pub struct CandleTemplate {
    pub candles: Vec<CandleWithAtr>,
    pub elapsed_ms: u128,
}
//
//
// Lend template
//
//
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
//
// Index template
//
//
#[derive(Template)]
#[template(path = "index/index.html")]
pub struct IndexTemplate {}
