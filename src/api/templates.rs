use crate::api::models::{
    Balance, Bots, Currency, Error, Event, EventOrder, MsgEvent, MsgSend, PgConnection,
    PgStatStatements, PgStatTableSize, PgTableIndex, PgTableInfo, PositionAsset, PositionDebt,
    PositionRatio, Symbol, Ticker,
};
use crate::api::page::Page;
use askama::Template;
use futures::try_join;
use sqlx::PgPool;

// ====== TICKERS ======
#[derive(Template)]
#[template(path = "tickers.html")]
pub struct TickersContent {
    pub tickers: Vec<(usize, Ticker)>,
    pub elapsed_ms: u128,
}

impl TickersContent {
    pub fn new(tickers: Vec<Ticker>, elapsed_ms: u128) -> Self {
        let tickers = tickers
            .into_iter()
            .enumerate()
            .map(|(i, v)| (i + 1, v))
            .collect();
        Self {
            tickers,
            elapsed_ms,
        }
    }
}

impl Page for TickersContent {
    type Data = Vec<Ticker>;

    async fn load_data(pool: &PgPool) -> Result<Self::Data, sqlx::Error> {
        sqlx::query_as::<_, Ticker>(
            r#"
            SELECT exchange, symbol, symbol_name, taker_fee_rate, maker_fee_rate, 
                   taker_coefficient, maker_coefficient, updated_at
            FROM ticker
            ORDER BY updated_at DESC;
            "#,
        )
        .fetch_all(pool)
        .await
    }

    fn from_data(data: Self::Data, elapsed_ms: u128) -> Self {
        Self::new(data, elapsed_ms)
    }
}

// ====== CURRENCIES ======
#[derive(Template)]
#[template(path = "currencies.html")]
pub struct CurrenciesContent {
    pub currencies: Vec<(usize, Currency)>,
    pub elapsed_ms: u128,
}

impl CurrenciesContent {
    pub fn new(currencies: Vec<Currency>, elapsed_ms: u128) -> Self {
        let currencies = currencies
            .into_iter()
            .enumerate()
            .map(|(i, v)| (i + 1, v))
            .collect();
        Self {
            currencies,
            elapsed_ms,
        }
    }
}

impl Page for CurrenciesContent {
    type Data = Vec<Currency>;

    async fn load_data(pool: &PgPool) -> Result<Self::Data, sqlx::Error> {
        sqlx::query_as::<_, Currency>(
            r#"
            SELECT exchange, currency, currency_name, full_name, precision, 
                   is_margin_enabled, is_debit_enabled, updated_at
            FROM currency
            ORDER BY updated_at DESC;
            "#,
        )
        .fetch_all(pool)
        .await
    }

    fn from_data(data: Self::Data, elapsed_ms: u128) -> Self {
        Self::new(data, elapsed_ms)
    }
}

// ====== BOTS ======
#[derive(Template)]
#[template(path = "bots/bots.html")]
pub struct BotsContent {
    pub bots: Vec<(usize, Bots)>,
    pub init_balance: f64,
    pub final_balance: f64,
    pub elapsed_ms: u128,
}

impl BotsContent {
    pub fn new(bots_list: Vec<Bots>, elapsed_ms: u128) -> Self {
        let bots: Vec<(usize, Bots)> = bots_list
            .into_iter()
            .enumerate()
            .map(|(i, v)| (i + 1, v))
            .collect();

        let final_balance = bots
            .iter()
            .filter_map(|(_, bot)| bot.balance.as_ref().and_then(|s| s.parse::<f64>().ok()))
            .sum();

        let init_balance = (20 * bots.len()) as f64;

        Self {
            bots,
            init_balance,
            final_balance,
            elapsed_ms,
        }
    }
}

impl Page for BotsContent {
    type Data = Vec<Bots>;

    async fn load_data(pool: &PgPool) -> Result<Self::Data, sqlx::Error> {
        sqlx::query_as::<_, Bots>(
            r#"
            SELECT exchange, entry_price, entry_client_oid, exit_tp_price, exit_tp_order_id,
                   exit_tp_client_oid, exit_sl_price, exit_sl_order_id, exit_sl_client_oid,
                   symbol, balance, updated_at
            FROM bots
            ORDER BY updated_at DESC;
            "#,
        )
        .fetch_all(pool)
        .await
    }

    fn from_data(data: Self::Data, elapsed_ms: u128) -> Self {
        Self::new(data, elapsed_ms)
    }
}

// ====== PG ======
pub struct PgData {
    pub connections: Vec<PgConnection>,
    pub table_info: Vec<PgTableInfo>,
    pub table_index: Vec<PgTableIndex>,
    pub statements: Vec<PgStatStatements>,
    pub table_size: Vec<PgStatTableSize>,
}

#[derive(Template)]
#[template(path = "pg/pg.html")]
pub struct PgContent {
    pub pg_stats_connections: Vec<PgConnection>,
    pub pg_stats_table_info: Vec<PgTableInfo>,
    pub pg_stats_table_index: Vec<PgTableIndex>,
    pub pg_stat_statements: Vec<PgStatStatements>,
    pub pg_stat_table_size: Vec<PgStatTableSize>,
    pub elapsed_ms: u128,
}

impl Page for PgContent {
    type Data = PgData;

    async fn load_data(pool: &PgPool) -> Result<Self::Data, sqlx::Error> {
        let (connections, table_info, table_index, statements, table_size) = try_join!(
            sqlx::query_as::<_, PgConnection>(
                "SELECT count(*) AS total_connections, count(*) FILTER (WHERE state = 'active') AS active_connections FROM pg_stat_activity;"
            ).fetch_all(pool),
            sqlx::query_as::<_, PgTableInfo>(
                "SELECT schemaname, relname, seq_scan, seq_tup_read, idx_scan, idx_tup_fetch, n_tup_ins, n_tup_upd, n_tup_del, n_live_tup, n_dead_tup FROM pg_stat_user_tables;"
            ).fetch_all(pool),
            sqlx::query_as::<_, PgTableIndex>(
                "SELECT schemaname, relname, idx_scan, idx_tup_read, idx_tup_fetch FROM pg_stat_user_indexes;"
            ).fetch_all(pool),
            sqlx::query_as::<_, PgStatStatements>(
                "SELECT query, calls, total_exec_time, mean_exec_time, rows FROM pg_stat_statements ORDER BY total_exec_time DESC LIMIT 100;"
            ).fetch_all(pool),
            sqlx::query_as::<_, PgStatTableSize>(
                "SELECT schemaname, relname, pg_size_pretty(pg_total_relation_size(schemaname || '.' || relname)) AS total_size, pg_size_pretty(pg_relation_size(schemaname || '.' || relname)) AS table_size, pg_size_pretty(pg_indexes_size(schemaname || '.' || relname)) AS indexes_size FROM pg_stat_user_tables;"
            ).fetch_all(pool)
        )?;

        Ok(PgData {
            connections,
            table_info,
            table_index,
            statements,
            table_size,
        })
    }

    fn from_data(data: Self::Data, elapsed_ms: u128) -> Self {
        Self {
            pg_stats_connections: data.connections,
            pg_stats_table_info: data.table_info,
            pg_stats_table_index: data.table_index,
            pg_stat_statements: data.statements,
            pg_stat_table_size: data.table_size,
            elapsed_ms,
        }
    }
}

// ====== INDEX TEMPLATE ======
#[derive(Template)]
#[template(path = "index/index.html")]
pub struct IndexTemplate {}

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
#[template(path = "bots/bots.html")]
pub struct BotsTemplate {
    pub bots: Vec<(usize, Bots)>,
    pub init_balance: f64,
    pub final_balance: f64,
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
