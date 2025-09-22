use actix_web::{App, HttpServer, middleware, web};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
mod handlers;

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
                            // Working with tickers
                            .route("/tickers", web::get().to(handlers::tickers))
                            .route("/ticker/{ticker}", web::get().to(handlers::ticker))
                            // Working with currencies
                            .route("/currencies", web::get().to(handlers::currencies))
                            .route("/currency/{currency}", web::get().to(handlers::currency))
                            // Working with symbols
                            .route("/symbols", web::get().to(handlers::symbols))
                            .route("/symbol/{symbol}", web::get().to(handlers::symbol))
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
