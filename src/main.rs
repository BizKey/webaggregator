use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    println!("Sever run on http://0.0.0.0:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(handlers::index))
            .route("/tickers", web::get().to(handlers::tickers))
            .route("/ticker/{ticker}", web::get().to(handlers::ticker))
            .route("/currencies", web::get().to(handlers::currencies))
            .route("/symbols", web::get().to(handlers::symbols))
            .route("/hello/{name}", web::get().to(handlers::hellodirect))
            .route("/static/style.css", web::get().to(handlers::serve_css))
            .route("/favicon.png", web::get().to(handlers::favicon))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
