use crate::api::models::ClearBalancesResponse;
use crate::core::app_state::AppState;
use actix_web::{HttpResponse, Result as ActixResult, web};
use serde_json::json;
use tracing::{error, info};

pub async fn clear_balances(state: web::Data<AppState>) -> ActixResult<HttpResponse> {
    match state.balance_service.clear_balances().await {
        Ok(deleted_count) => {
            info!("Cleared {} records from balance table", deleted_count);
            Ok(HttpResponse::Ok().json(ClearBalancesResponse {
                deleted_count,
                message: format!("Successfully deleted {} balance records", deleted_count),
            }))
        }
        Err(e) => {
            error!("Failed to clear balances: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Failed to clear balances"
            })))
        }
    }
}
