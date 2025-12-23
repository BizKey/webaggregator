use actix_web::{App, HttpServer, middleware, web};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
mod handlers;
mod models;
mod templates;
use crate::handlers::balance::balances;
use crate::handlers::currency::currencies;
use crate::handlers::errors::errors;
use crate::handlers::events::{events, msgevent, msgsend};
use crate::handlers::index::index;
use crate::handlers::orders::{activeorders, eventorders};
use crate::handlers::pg::pg;
use crate::handlers::position::{positionasset, positiondebt, positionratio};
use crate::handlers::symbol::{symbol_trade, symbols, tradeable};
use crate::handlers::system::{favicon, serve_css};
use crate::handlers::ticker::tickers;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv().ok();

    match env::var("DATABASE_URL") {
        Ok(database_url) => {
            match PgPoolOptions::new()
                .max_connections(5)
                .connect(&database_url)
                .await
            {
                Ok(pool) => {
                    HttpServer::new(move || {
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
                            // active orders
                            .route("/activeorder", web::get().to(activeorders))
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
                            // symbol_trade
                            .route("/symbol_trade", web::get().to(symbol_trade))
                            // Working with tickers
                            .route("/tickers", web::get().to(tickers))
                            // Working with currencies
                            .route("/currencies", web::get().to(currencies))
                            // Working with symbols
                            .route("/symbols", web::get().to(symbols))
                            // System links
                            .route("/static/style.css", web::get().to(serve_css))
                            .route("/favicon.png", web::get().to(favicon))
                    })
                    .bind("0.0.0.0:8080")?
                    .run()
                    .await
                }
                Err(e) => {
                    eprintln!("Failed to create database pool: {}", e);
                    return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
                }
            }
        }
        Err(e) => {
            eprintln!("DATABASE_URL not set: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, e));
        }
    }
}
