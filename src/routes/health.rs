use axum::{
    response::Json,
    routing::get,
    Router,
};
use serde_json::json;

pub fn health_routes() -> Router {
    Router::new().route("/health", get(health_check))
}

pub async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "ok",
        "service": "chatbot-volt-clickmassa",
        "version": "0.1.0"
    }))
}
