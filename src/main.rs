use actix_web::{App, HttpServer, web};

mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    println!("Sever run on http://0.0.0.0:8080");

    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(handlers::index))
            .route("/hello", web::get().to(handlers::hello))
            .route("/hello/{name}", web::get().to(handlers::hellodirect))
            .route("/static/style.css", web::get().to(handlers::serve_css))
            .route("/favicon.png", web::get().to(handlers::favicon))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
