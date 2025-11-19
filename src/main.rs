use actix_web::{App, HttpServer, middleware, web};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
mod handlers;
mod models;
mod templates;
use crate::handlers::borrow::{borrow, borrows};
use crate::handlers::candle::{candle, candles};
use crate::handlers::currency::currencies;
use crate::handlers::index::index;
use crate::handlers::lend::{lend, lends};
use crate::handlers::pg::pg;
use crate::handlers::smastrategy::{smastrategy, smastrategy_by_symbol};
use crate::handlers::strategy::{strategy, tickerstrategy};
use crate::handlers::symbol::symbols;
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
                            //
                            // Working with tickers
                            .route("/tickers", web::get().to(tickers))
                            //
                            // Work with pg
                            .route("/pg", web::get().to(pg))
                            //
                            // Working with currencies
                            .route("/currencies", web::get().to(currencies))
                            //
                            // Working with symbols
                            .route("/symbols", web::get().to(symbols))
                            //
                            // Working with lend
                            .route("/lend", web::get().to(lends))
                            .route("/lend/{currency}", web::get().to(lend))
                            //
                            // Working with borrow
                            .route("/borrow", web::get().to(borrows))
                            .route("/borrow/{currency}", web::get().to(borrow))
                            //
                            // Working with candles
                            .route("/candle", web::get().to(candles))
                            .route("/candle/{ticker}", web::get().to(candle))
                            //
                            //
                            .route("/strategy", web::get().to(strategy))
                            .route("/strategy/{ticker}", web::get().to(tickerstrategy))
                            .route("/sma", web::get().to(smastrategy))
                            .route("/sma/{symbol}", web::get().to(smastrategy_by_symbol))
                            //
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
