use axum::{
    extract::Json,
    routing::post,
    Router,
};
use crate::error::AppResult;
use crate::models::chatbot::{
    ConsultaCpfRequest, ConsultaCpfResponse,
    ValidarCpfRequest, ValidarCpfResponse,
};
use crate::utils::cpf_validator;

pub fn cpf_routes() -> Router {
    Router::new()
        .route("/cpf/consultar", post(consultar_cpf))
        .route("/cpf/validar", post(validar_cpf))
}

pub async fn validar_cpf(
    Json(payload): Json<ValidarCpfRequest>,
) -> Json<ValidarCpfResponse> {
    match cpf_validator::validate_cpf(&payload.cpf) {
        Ok(cpf_valido) => {
            tracing::info!("CPF válido: {}", cpf_valido);
            Json(ValidarCpfResponse {
                valido: true,
                cpf_formatado: Some(cpf_validator::format_cpf(&cpf_valido)),
                mensagem: "CPF válido".to_string(),
            })
        }
        Err(e) => {
            tracing::warn!("CPF inválido: {}", e);
            Json(ValidarCpfResponse {
                valido: false,
                cpf_formatado: None,
                mensagem: format!("{}", e),
            })
        }
    }
}

pub async fn consultar_cpf(
    Json(payload): Json<ConsultaCpfRequest>,
) -> AppResult<Json<ConsultaCpfResponse>> {
    // Validar CPF
    let cpf_valido = cpf_validator::validate_cpf(&payload.cpf)?;

    tracing::info!("CPF válido recebido: {}", cpf_valido);

    // Por enquanto, retorna dados mockados
    Ok(Json(ConsultaCpfResponse {
        cpf: cpf_validator::format_cpf(&cpf_valido),
        nome: "João da Silva (MOCK)".to_string(),
        status: "ativo".to_string(),
    }))
}
