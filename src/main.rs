mod api {
    pub mod models;
    pub mod templates;
    pub mod tools;
}
mod handlers;
use crate::api::tools::get_env;
use crate::handlers::balance::balances;
use crate::handlers::bots::bots;
use crate::handlers::currency::currencies;
use crate::handlers::errors::errors;
use crate::handlers::events::{events, msgevent, msgsend};
use crate::handlers::index::index;
use crate::handlers::orders::eventorders;
use crate::handlers::pg::pg;
use crate::handlers::position::{positionasset, positiondebt, positionratio};
use crate::handlers::symbol::{symbols, tradeable};
use crate::handlers::system::{favicon, serve_css};
use crate::handlers::ticker::tickers;
use actix_web::{App, HttpServer, middleware, web};
use anyhow::{Context, Result};
use dotenvy::dotenv;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init();
    dotenv().ok();

    let database_url: String = get_env("DATABASE_URL").context("Don't find ENV")?;

    let pool: PgPool = PgPoolOptions::new()
        .max_connections(10)
        .min_connections(5)
        .acquire_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Duration::from_secs(1800))
        .connect(&database_url)
        .await
        .context("Failed to create pg pool")?;

    log::info!("DB connected");

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Compress::default())
            .route("/", web::get().to(index))
            // Work with pg
            .route("/pg", web::get().to(pg))
            // events
            .route("/events", web::get().to(events))
            // errors
            .route("/errors", web::get().to(errors))
            // balance
            .route("/balance", web::get().to(balances))
            // event orders
            .route("/eventorder", web::get().to(eventorders))
            // position debt
            .route("/positiondebt", web::get().to(positiondebt))
            // msgevent
            .route("/msgevent", web::get().to(msgevent))
            // msgsend
            .route("/msgsend", web::get().to(msgsend))
            // position asset
            .route("/positionasset", web::get().to(positionasset))
            // position ratio
            .route("/positionratio", web::get().to(positionratio))
            // tradeable
            .route("/tradeable", web::get().to(tradeable))
            // Working with tickers
            .route("/tickers", web::get().to(tickers))
            // Working with currencies
            .route("/currencies", web::get().to(currencies))
            // Working with symbols
            .route("/symbols", web::get().to(symbols))
            // bots
            .route("/bots", web::get().to(bots))
            // System links
            .route("/static/style.css", web::get().to(serve_css))
            .route("/favicon.png", web::get().to(favicon))
    })
    .bind("0.0.0.0:8080")
    .context("Failed start server 0.0.0.0:8080")?;

    log::info!("Server started: http://0.0.0.0:8080");

    server.run().await.context("Fail http server")?;

    Ok(())
}
