use axum::{
    response::Json,
    routing::get,
    Router,
};
use serde_json::json;

pub fn health_routes() -> Router {
    Router::new().route("/health", get(health_check))
}

/// Verifica o status de operação do serviço
/// 
/// Retorna informações básicas sobre o middleware incluindo versão e status
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (
            status = 200,
            description = "Serviço está operacional",
            content_type = "application/json"
        )
    ),
    tag = "health"
)]
pub async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "ok",
        "service": "chatbot-volt-clickmassa",
        "version": "0.1.0"
    }))
}
