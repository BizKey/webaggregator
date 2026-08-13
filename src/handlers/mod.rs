use actix_web::{HttpResponse, Result as ActixResult, web};
use sqlx::PgPool;
use std::time::Instant;
use tracing::error;

use crate::api::page::Page;

pub async fn render_page<P: Page>(pool: web::Data<PgPool>) -> ActixResult<HttpResponse> {
    let start = Instant::now();

    match P::load_data(pool.as_ref()).await {
        Ok(data) => {
            let page = P::from_data(data, start.elapsed().as_millis());
            let html = page.render().map_err(|e| {
                error!("Template render error: {}", e);
                actix_web::error::ErrorInternalServerError("Template error")
            })?;

            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(html))
        }
        Err(e) => {
            error!("Database error: {}", e);
            Err(actix_web::error::ErrorInternalServerError("Database error"))
        }
    }
}

pub mod balance;
pub mod bots;
pub mod currency;
pub mod errors;
pub mod events;
pub mod index;
pub mod orders;
pub mod pg;
pub mod position;
pub mod symbol;
pub mod system;
pub mod ticker;

pub use index::index;
pub use system::{favicon, serve_css};
