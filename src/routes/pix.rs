use axum::{
    extract::Json,
    routing::post,
    Router,
};
use crate::error::AppResult;
use crate::models::chatbot::{ValidarPixRequest, ValidarPixResponse};
use crate::utils::pix_validator;

pub fn pix_routes() -> Router {
    Router::new().route("/pix/validar", post(validar_pix))
}

/// Validar chave PIX
/// 
/// Valida formato de chave PIX conforme tipo (CPF, telefone, email, aleatória)
/// 
/// **Tipos suportados:**
/// - `cpf`: CPF do titular (11 dígitos)
/// - `phone`: Telefone com DDD (11 dígitos)
/// - `email`: Email válido
/// - `random`: Chave aleatória (UUID)
#[utoipa::path(
    post,
    path = "/pix/validar",
    context_path = "/api/v1", 
    request_body = ValidarPixRequest,
    responses(
        (
            status = 200,
            description = "Chave PIX validada",
            body = ValidarPixResponse,
            content_type = "application/json"
        ),
        (
            status = 400,
            description = "Chave PIX inválida"
        )
    ),
    tag = "pix"
)]
pub async fn validar_pix(
    Json(payload): Json<ValidarPixRequest>,
) -> AppResult<Json<ValidarPixResponse>> {
    tracing::info!(
        "🔑 Validando chave PIX tipo: {} para CPF: {}",
        payload.tipo_chave,
        payload.cpf
    );

    // 1. Validar CPF do titular
    let _cpf_valido = crate::utils::cpf_validator::validate_cpf(&payload.cpf)?;

    // 2. Validar chave PIX
    match pix_validator::validate_pix_key(&payload.chave_pix, &payload.tipo_chave) {
        Ok(chave_formatada) => {
            tracing::info!("Chave PIX válida: {}", chave_formatada);

            // TODO: Aqui você pode adicionar chamada para API do Banco Central
            // para verificar se a chave realmente existe e pertence ao CPF

            Ok(Json(ValidarPixResponse {
                valida: true,
                tipo_chave: payload.tipo_chave.clone(),
                chave_formatada: Some(chave_formatada),
                mensagem: "Chave PIX válida".to_string(),
            }))
        }
        Err(e) => {
            tracing::warn!("Chave PIX inválida: {}", e);
            Ok(Json(ValidarPixResponse {
                valida: false,
                tipo_chave: payload.tipo_chave.clone(),
                chave_formatada: None,
                mensagem: format!("{}", e),
            }))
        }
    }
}
