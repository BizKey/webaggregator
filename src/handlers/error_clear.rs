use crate::api::models::ClearErrorsResponse;
use crate::core::app_state::AppState;
use actix_web::{HttpResponse, Result as ActixResult, web};
use serde_json::json;
use tracing::{error, info};

pub async fn clear_errors(state: web::Data<AppState>) -> ActixResult<HttpResponse> {
    match state.error_service.clear_errors().await {
        Ok(deleted_count) => {
            info!("Cleared {} errors from table", deleted_count);
            Ok(HttpResponse::Ok().json(ClearErrorsResponse {
                deleted_count,
                message: format!("Successfully deleted {} error records", deleted_count),
            }))
        }
        Err(e) => {
            error!("Failed to clear errors: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Failed to clear errors"
            })))
        }
    }
}
