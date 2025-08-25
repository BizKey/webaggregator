use actix_web::{App, HttpServer, web};
use dotenv::dotenv;

mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    println!("Sever run on http://0.0.0.0:8080");

    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(handlers::index))
            .route("/users", web::get().to(handlers::hello))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
