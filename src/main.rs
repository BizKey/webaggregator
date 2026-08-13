mod api {
    pub mod models;
    pub mod page;
    pub mod templates;
    pub mod tools;
}
mod handlers;

use crate::api::templates::TickersContent;
use crate::api::templates::{BotsContent, CurrenciesContent, PgContent, TickersContent};
use crate::api::tools::get_env;
use crate::api::tools::get_env;
use crate::handlers::render_page;
use crate::handlers::{
    balance::balances,
    bots::bots,
    currency::currencies,
    errors::errors,
    events::{events, msgevent, msgsend},
    index::index,
    orders::eventorders,
    pg::pg,
    position::{positionasset, positiondebt, positionratio},
    symbol::{symbols, tradeable},
    system::{favicon, serve_css},
    ticker::tickers,
};
use crate::handlers::{favicon, index, render_page, serve_css};
use actix_web::{App, HttpServer, middleware, web};
use actix_web::{App, HttpServer, middleware, web};
use anyhow::{Context, Result};
use anyhow::{Context, Result};
use dotenvy::dotenv;
use dotenvy::dotenv;
use sqlx::{PgPool, postgres::PgPoolOptions};
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;
use std::time::Duration;
use tracing::info;
use tracing::info;

fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_target(true)
        .with_thread_ids(true)
        .init();
}

async fn create_db_pool() -> Result<PgPool> {
    let database_url = get_env("DATABASE_URL")?;

    Ok(PgPoolOptions::new()
        .max_connections(10)
        .min_connections(1)
        .acquire_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Duration::from_secs(1800))
        .connect(&database_url)
        .await
        .context("Failed to connect to PostgreSQL")?)
}

fn routes(cfg: &mut web::ServiceConfig) {
    use web::get;
    cfg.route("/", get().to(index::index))
        .route("/tickers", get().to(render_page::<TickersContent>))
        .route("/currencies", get().to(render_page::<CurrenciesContent>))
        .route("/bots", get().to(render_page::<BotsContent>))
        .route("/pg", get().to(render_page::<PgContent>))
        .route("/static/style.css", get().to(serve_css))
        .route("/favicon.png", get().to(favicon));
}

const SERVER_ADDR: &str = "0.0.0.0:8080";

#[actix_web::main]
async fn main() -> Result<()> {
    init_tracing();
    dotenv().ok();

    let pool = create_db_pool().await?;
    info!("Database connected");

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Compress::default())
            .configure(routes)
    })
    .bind(SERVER_ADDR)
    .with_context(|| format!("Failed to bind server to {SERVER_ADDR}"))?;

    info!("Server running on http://0.0.0.0:8080");

    server.run().await.context("Server crashed")?;

    Ok(())
}
