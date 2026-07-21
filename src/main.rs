mod api {
    pub mod models;
    pub mod templates;
    pub mod tools;
}
mod handlers;
use crate::api::tools::get_env;
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

use actix_web::{App, HttpServer, middleware, web};
use anyhow::{Context, Result};
use dotenvy::dotenv;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;
use tracing::info;

#[actix_web::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_target(true)
        .with_thread_ids(true)
        .init();
    dotenv().ok();

    let database_url: String = get_env("DATABASE_URL").context("DATABASE_URL not set")?;

    let pool: PgPool = PgPoolOptions::new()
        .max_connections(10)
        .min_connections(5)
        .acquire_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Duration::from_secs(1800))
        .connect(&database_url)
        .await
        .context("Failed to connect to PostgreSQL")?;

    info!("Database connected");

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Compress::default())
            .configure(routes)
    })
    .bind("0.0.0.0:8080")
    .context("Failed to bind server to 0.0.0.0:8080")?;

    info!("Server running on http://0.0.0.0:8080");

    server.run().await.context("Server crashed")?;

    Ok(())
}

fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index))
        .route("/pg", web::get().to(pg))
        .route("/events", web::get().to(events))
        .route("/errors", web::get().to(errors))
        .route("/balance", web::get().to(balances))
        .route("/eventorder", web::get().to(eventorders))
        .route("/positiondebt", web::get().to(positiondebt))
        .route("/msgevent", web::get().to(msgevent))
        .route("/msgsend", web::get().to(msgsend))
        .route("/positionasset", web::get().to(positionasset))
        .route("/positionratio", web::get().to(positionratio))
        .route("/tradeable", web::get().to(tradeable))
        .route("/tickers", web::get().to(tickers))
        .route("/currencies", web::get().to(currencies))
        .route("/symbols", web::get().to(symbols))
        .route("/bots", web::get().to(bots))
        .route("/static/style.css", web::get().to(serve_css))
        .route("/favicon.png", web::get().to(favicon));
}
