use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Erro de configuração: {0}")]
    ConfigError(String),

    #[error("Erro de autenticação: {0}")]
    AuthError(String),

    #[error("Erro na requisição V8: {0}")]
    V8Error(String),

    #[error("Erro na API externa: {0}")]
    ExternalApiError(String),

    #[error("Erro de validação: {0}")]
    ValidationError(String),

    #[error("Recurso não encontrado")]
    NotFound,

    #[error("Erro interno do servidor: {0}")]
    InternalError(String),

    #[error("{0}")]
    Other(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::ConfigError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Erro de configuração: {}", msg),
            ),
            AppError::AuthError(msg) => (
                StatusCode::UNAUTHORIZED,
                format!("Erro de autenticação: {}", msg),
            ),
            AppError::V8Error(msg) => (
                StatusCode::BAD_GATEWAY,
                format!("Erro na API V8: {}", msg),
            ),
            AppError::ExternalApiError(msg) => (
                StatusCode::BAD_GATEWAY,
                format!("Erro na API externa: {}", msg),
            ),
            AppError::ValidationError(msg) => (
                StatusCode::BAD_REQUEST,
                format!("Erro de validação: {}", msg),
            ),
            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                "Recurso não encontrado".to_string(),
            ),
            AppError::InternalError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Erro interno: {}", msg),
            ),
            AppError::Other(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Erro: {}", msg),
            ),
        };

        let body = Json(json!({
            "error": error_message,
            "status": status.as_u16(),
        }));

        (status, body).into_response()
    }
}

// Type alias para Result com AppError
pub type AppResult<T> = Result<T, AppError>;
