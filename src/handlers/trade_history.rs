use crate::core::app_state::AppState;
use actix_web::{HttpResponse, Result as ActixResult, web};
use serde::Serialize;
use std::time::Instant;
use tracing::error;

#[derive(Debug, Serialize)]
pub struct TradeHistoryResponse {
    pub symbol: String,
    pub trades: Vec<TradeDetail>,
    pub elapsed_ms: u128,
}

#[derive(Debug, Serialize)]
pub struct TradeDetail {
    pub order_id: String,
    pub client_oid: Option<String>,
    pub side: String,
    pub price: Option<String>,
    pub size: Option<String>,
    pub filled_size: Option<String>,
    pub status: String,
    pub event_time: chrono::DateTime<chrono::Utc>,
    pub stop_order: Option<StopDetail>,
    pub direction: String, // "UP" или "DOWN"
}

#[derive(Debug, Serialize)]
pub struct StopDetail {
    pub client_oid: String,
    pub side: String,
    pub stop_type: String,
    pub stop_price: String,
    pub size: Option<String>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub async fn trade_history(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    let start = Instant::now();
    let symbol = path.into_inner();

    // Получаем сделки с stop orders
    let trades_with_stops = state
        .order_service
        .get_trades_with_stops(&symbol, 3)
        .await
        .map_err(|e| {
            error!("Service error: {}", e);
            actix_web::error::ErrorInternalServerError("Service error")
        })?;

    if trades_with_stops.is_empty() {
        return Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("No trades found for symbol: {}", symbol)
        })));
    }

    let trades: Vec<TradeDetail> = trades_with_stops
        .into_iter()
        .map(|t| {
            // Определяем направление по side и stop_type
            let direction = if let Some(stop) = t.stop_client_oid.as_ref() {
                let stop_type = t.stop_type.as_ref().unwrap();
                let stop_side = t.stop_side.as_ref().unwrap();

                // Стоп-лосс: цена пошла против позиции
                if stop_type == "loss" {
                    if stop_side == "buy" {
                        "DOWN".to_string() // Цена упала, сработал buy stop-loss
                    } else {
                        "UP".to_string() // Цена выросла, сработал sell stop-loss
                    }
                }
                // Стоп-лимит/вход: цена пошла в направлении входа
                else if stop_type == "entry" {
                    if stop_side == "buy" {
                        "UP".to_string() // Цена выросла, вход в buy
                    } else {
                        "DOWN".to_string() // Цена упала, вход в sell
                    }
                } else {
                    // Если тип не распознан, используем side
                    match t.side.as_str() {
                        "buy" | "BUY" => "UP".to_string(),
                        "sell" | "SELL" => "DOWN".to_string(),
                        _ => "UNKNOWN".to_string(),
                    }
                }
            } else {
                // Нет stop_order, используем side
                match t.side.as_str() {
                    "buy" | "BUY" => "UP".to_string(),
                    "sell" | "SELL" => "DOWN".to_string(),
                    _ => "UNKNOWN".to_string(),
                }
            };

            TradeDetail {
                order_id: t.order_id,
                client_oid: t.client_oid,
                side: t.side,
                price: t.price,
                size: t.size,
                filled_size: t.filled_size,
                status: t.status,
                event_time: t.event_updated_at,
                stop_order: t.stop_client_oid.as_ref().map(|_| StopDetail {
                    client_oid: t.stop_client_oid.clone().unwrap(),
                    side: t.stop_side.clone().unwrap(),
                    stop_type: t.stop_type.clone().unwrap(),
                    stop_price: t.stop_price.clone().unwrap(),
                    size: t.stop_size.clone(),
                    updated_at: t.stop_updated_at.unwrap(),
                }),
                direction,
            }
        })
        .collect();

    Ok(HttpResponse::Ok().json(TradeHistoryResponse {
        symbol,
        trades,
        elapsed_ms: start.elapsed().as_millis(),
    }))
}
