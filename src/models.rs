use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Ticker {
    pub exchange: String,
    pub symbol: String,
    pub symbol_name: String,
    pub taker_fee_rate: Option<String>,
    pub maker_fee_rate: Option<String>,
    pub taker_coefficient: Option<String>,
    pub maker_coefficient: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Symbol {
    pub exchange: String,
    pub symbol: String,
    pub name: String,
    pub base_currency: String,
    pub quote_currency: String,
    pub fee_currency: String,
    pub market: String,
    pub base_min_size: String,
    pub quote_min_size: String,
    pub base_max_size: String,
    pub quote_max_size: String,
    pub base_increment: String,
    pub quote_increment: String,
    pub price_increment: String,
    pub price_limit_rate: String,
    pub min_funds: Option<String>,
    pub is_margin_enabled: bool,
    pub enable_trading: bool,
    pub fee_category: i16,
    pub maker_fee_coefficient: String,
    pub taker_fee_coefficient: String,
    pub st: bool,
    pub callauction_is_enabled: bool,
    pub callauction_price_floor: Option<String>,
    pub callauction_price_ceiling: Option<String>,
    pub callauction_first_stage_start_time: Option<i64>,
    pub callauction_second_stage_start_time: Option<i64>,
    pub callauction_third_stage_start_time: Option<i64>,
    pub trading_start_time: Option<i64>,
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Currency {
    pub exchange: String,
    pub currency: String,
    pub name: String,
    pub full_name: String,
    pub precision: i16,
    pub confirms: Option<i16>,
    pub contract_address: Option<String>,
    pub is_margin_enabled: bool,
    pub is_debit_enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct PgConnection {
    pub total_connections: i64,
    pub active_connections: i64,
}
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct PgTableInfo {
    pub schemaname: String,
    pub relname: String,
    pub seq_scan: Option<i64>,
    pub seq_tup_read: Option<i64>,
    pub idx_scan: Option<i64>,
    pub idx_tup_fetch: Option<i64>,
    pub n_tup_ins: Option<i64>,
    pub n_tup_upd: Option<i64>,
    pub n_tup_del: Option<i64>,
    pub n_live_tup: Option<i64>,
    pub n_dead_tup: Option<i64>,
}
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct PgTableIndex {
    pub schemaname: String,
    pub relname: String,
    pub idx_scan: Option<i64>,
    pub idx_tup_read: Option<i64>,
    pub idx_tup_fetch: Option<i64>,
}
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct PgStatStatements {
    pub query: String,
    pub calls: Option<i64>,
    pub total_exec_time: Option<f64>,
    pub mean_exec_time: Option<f64>,
    pub rows: Option<i64>,
}
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct PgStatTableSize {
    pub schemaname: String,
    pub relname: String,
    pub total_size: String,
    pub table_size: String,
    pub indexes_size: String,
}
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Event {
    pub exchange: String,
    pub msg: String,
}
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Error {
    pub exchange: String,
    pub msg: String,
}
