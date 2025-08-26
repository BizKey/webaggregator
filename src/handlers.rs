use actix_web::{HttpResponse, Result, web};
use askama::Template;
use sqlx::PgPool;

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Ticket {
    pub id: i32,
    pub name: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    name: String,
    time: String,
}

pub async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body("Welcome to rr")
}

pub async fn hello() -> HttpResponse {
    let template = IndexTemplate {
        name: "User".to_string(),
        time: chrono::Local::now().format("%H:%M:%S").to_string(),
    };

    match template.render() {
        Ok(html) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html),
        Err(_) => HttpResponse::InternalServerError().body("Error template render"),
    }
}

pub async fn tickers(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let tickets = sqlx::query_as::<_, Ticket>("SELECT id, name FROM Ticket")
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    Ok(HttpResponse::Ok().json(tickets))
}

pub async fn hellodirect(path: web::Path<String>, pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let name = path.into_inner();

    let _ = sqlx::query_as::<_, Ticket>("INSERT INTO Ticket (name) VALUES ($1) RETURNING id, name")
        .bind(&name)
        .fetch_one(pool.get_ref())
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Failed to create ticket")
        })?;

    let template = IndexTemplate {
        name: name,
        time: chrono::Local::now().format("%H:%M:%S").to_string(),
    };

    match template.render() {
        Ok(html) => Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html)),
        Err(_) => Ok(HttpResponse::InternalServerError().body("Error template render")),
    }
}

pub async fn serve_css() -> Result<HttpResponse, std::io::Error> {
    let content = std::fs::read_to_string("./static/style.css")?;

    Ok(HttpResponse::Ok()
        .content_type("text/css; charset=utf-8")
        .insert_header(("Cache-Control", "public, max-age=3600"))
        .body(content))
}

pub async fn favicon() -> Result<HttpResponse, std::io::Error> {
    let bytes = std::fs::read("./static/favicon.png")?;

    Ok(HttpResponse::Ok()
        .content_type("image/png")
        .insert_header(("Cache-Control", "public, max-age=3600"))
        .body(bytes))
}
