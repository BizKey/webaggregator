mod application;
mod domain;
mod infrastructure;
use crate::application::services::{
    BalanceService, BotService, CurrencyService, ErrorService, EventService, OrderService,
    PgStatService, PositionService, SymbolService, TickerService,
};
use crate::infrastructure::config::AppConfig;
use crate::infrastructure::db::postgres::repositories::{
    PostgresBalanceRepository, PostgresBotRepository, PostgresCurrencyRepository,
    PostgresErrorRepository, PostgresEventRepository, PostgresOrderRepository,
    PostgresPgStatRepository, PostgresPositionRepository, PostgresSymbolRepository,
    PostgresTickerRepository, Repositories,
};
use crate::infrastructure::logging::init_tracing;
use crate::infrastructure::web::handlers::{
    favicon, get_balances, get_bots, get_currencies, get_errors, get_events, get_index,
    get_msg_events, get_msg_sends, get_orders, get_pg_stats, get_position_assets,
    get_position_debts, get_position_ratios, get_symbols, get_tickers, serve_css,
};
use actix_web::{App, HttpServer, middleware, web};
use anyhow::{Context, Result};
use dotenvy::dotenv;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;
use tokio::signal;
use tracing::info;

async fn create_db_pool(config: &AppConfig) -> Result<PgPool> {
    Ok(PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Duration::from_secs(1800))
        .connect(&config.database_url)
        .await
        .context("Failed to connect to PostgreSQL")?)
}

fn routes(cfg: &mut web::ServiceConfig) {
    use web::get;
    cfg.route("/", get().to(get_index))
        .route(
            "/tickers",
            get().to(get_tickers::<PostgresTickerRepository>),
        )
        .route(
            "/symbols",
            get().to(get_symbols::<PostgresSymbolRepository>),
        )
        .route(
            "/currencies",
            get().to(get_currencies::<PostgresCurrencyRepository>),
        )
        .route(
            "/balance",
            get().to(get_balances::<PostgresBalanceRepository>),
        )
        .route(
            "/positionasset",
            get().to(get_position_assets::<PostgresPositionRepository>),
        )
        .route(
            "/positiondebt",
            get().to(get_position_debts::<PostgresPositionRepository>),
        )
        .route(
            "/positionratio",
            get().to(get_position_ratios::<PostgresPositionRepository>),
        )
        .route(
            "/eventorder",
            get().to(get_orders::<PostgresOrderRepository>),
        )
        .route("/bots", get().to(get_bots::<PostgresBotRepository>))
        .route("/events", get().to(get_events::<PostgresEventRepository>))
        .route(
            "/msgevent",
            get().to(get_msg_events::<PostgresEventRepository>),
        )
        .route(
            "/msgsend",
            get().to(get_msg_sends::<PostgresEventRepository>),
        )
        .route("/errors", get().to(get_errors::<PostgresErrorRepository>))
        .route("/pg", get().to(get_pg_stats::<PostgresPgStatRepository>))
        .route("/static/style.css", get().to(serve_css))
        .route("/favicon.png", get().to(favicon));
}

async fn shutdown_signal() {
    let sigint = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
        info!("Received SIGINT (Ctrl+C)");
    };

    #[cfg(unix)]
    let sigterm = async {
        use tokio::signal::unix::{SignalKind, signal};
        let mut sig = signal(SignalKind::terminate()).expect("Failed to install SIGTERM handler");
        sig.recv().await;
        info!("Received SIGTERM");
    };

    #[cfg(not(unix))]
    let sigterm = std::future::pending::<()>();

    tokio::select! {
        _ = sigint => {},
        _ = sigterm => {},
    }
}

#[actix_web::main]
async fn main() -> Result<()> {
    init_tracing();
    dotenv().ok();

    let config = AppConfig::from_env().map_err(|e| anyhow::anyhow!(e))?;

    info!("Configuration loaded");
    info!("Server address: {}", config.server_addr);

    let pool = create_db_pool(&config).await?;
    info!("Database connected");

    // Create services
    let repos = Repositories::new(pool.clone());
    let ticker_service = TickerService::new(repos.ticker);
    let symbol_service = SymbolService::new(repos.symbol);
    let currency_service = CurrencyService::new(repos.currency);
    let balance_service = BalanceService::new(repos.balance);
    let position_service = PositionService::new(repos.position);
    let order_service = OrderService::new(repos.order);
    let bot_service = BotService::new(repos.bot);
    let event_service = EventService::new(repos.event);
    let error_service = ErrorService::new(repos.error);
    let pg_stat_service = PgStatService::new(repos.pg_stat);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(ticker_service.clone()))
            .app_data(web::Data::new(symbol_service.clone()))
            .app_data(web::Data::new(currency_service.clone()))
            .app_data(web::Data::new(balance_service.clone()))
            .app_data(web::Data::new(position_service.clone()))
            .app_data(web::Data::new(order_service.clone()))
            .app_data(web::Data::new(bot_service.clone()))
            .app_data(web::Data::new(event_service.clone()))
            .app_data(web::Data::new(error_service.clone()))
            .app_data(web::Data::new(pg_stat_service.clone()))
            .wrap(middleware::Compress::default())
            .configure(routes)
    })
    .bind(&config.server_addr)
    .with_context(|| format!("Failed to bind server to {}", config.server_addr))?;

    info!("Server running on http://{}", config.server_addr);

    let server_handle = server.run();
    let server_task = tokio::spawn(server_handle);

    shutdown_signal().await;
    info!("Initiating graceful shutdown...");

    server_task.abort();
    info!("HTTP server stopped");

    pool.close().await;
    info!("Database pool closed");

    info!("Shutdown complete");
    Ok(())
}
