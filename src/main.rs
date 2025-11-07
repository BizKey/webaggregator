use actix_web::{App, HttpServer, middleware, web};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
mod handlers;
mod models;
mod templates;
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
                            .route("/", web::get().to(handlers::index))
                            //
                            // Working with tickers
                            .route("/tickers", web::get().to(handlers::tickers))
                            //
                            // Working with currencies
                            .route("/currencies", web::get().to(handlers::currencies))
                            //
                            // Working with symbols
                            .route("/symbols", web::get().to(handlers::symbols))
                            //
                            // Working with lend
                            .route("/lend", web::get().to(handlers::lends))
                            .route("/lend/{currency}", web::get().to(handlers::lend))
                            //
                            // Working with borrow
                            .route("/borrow", web::get().to(handlers::borrows))
                            .route("/borrow/{currency}", web::get().to(handlers::borrow))
                            //
                            // Working with candles
                            .route("/candle", web::get().to(handlers::candles))
                            .route("/candle/{ticker}", web::get().to(handlers::candle))
                            //
                            //
                            .route("/strategy", web::get().to(handlers::strategy))
                            .route(
                                "/strategy/{ticker}",
                                web::get().to(handlers::tickerstrategy),
                            )
                            .route("/smastrategy", web::get().to(handlers::smastrategy))
                            //
                            // System links
                            .route("/static/style.css", web::get().to(handlers::serve_css))
                            .route("/favicon.png", web::get().to(handlers::favicon))
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
