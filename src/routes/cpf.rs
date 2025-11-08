use axum::{
    extract::{Json, State},
    routing::post,
    Router,
};
use std::sync::Arc;

use crate::error::AppResult;
use crate::models::chatbot::{
    ConsultaCpfRequest, ConsultaCpfResponse,
    ValidarCpfRequest, ValidarCpfResponse,
};
use crate::services::enrichment_service::EnrichmentService;
use crate::utils::cpf_validator;

// ← Adicione o State
#[derive(Clone)]
pub struct CpfState {
    pub enrichment_service: Arc<EnrichmentService>,
}

pub fn cpf_routes(state: CpfState) -> Router {
    Router::new()
        .route("/cpf/validar", post(validar_cpf))
        .route("/cpf/consultar", post(consultar_cpf))
        .with_state(state)
}

/// Validar CPF
/// 
/// Valida um CPF verificando:
/// - Formato (11 dígitos)
/// - Dígitos verificadores (algoritmo oficial)
/// - CPFs conhecidos inválidos
/// 
/// **Exemplo de CPF válido:** `111.444.777-35`
#[utoipa::path(
    post,
    path = "/cpf/validar",
    request_body = ValidarCpfRequest,
    responses(
        (
            status = 200,
            description = "CPF validado com sucesso",
            body = ValidarCpfResponse,
            content_type = "application/json"
        ),
        (
            status = 400,
            description = "CPF inválido - formato incorreto ou dígitos verificadores errados"
        ),
        (
            status = 500,
            description = "Erro interno do servidor"
        )
    ),
    tag = "cpf"
)]
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

/// Consultar dados de CPF
/// 
/// Consulta dados reais de CPF usando API externa HighConsult
/// Retorna: nome, data de nascimento, nome da mãe, endereço completo
#[utoipa::path(
    post,
    path = "/cpf/consultar",
    request_body = ConsultaCpfRequest,
    responses(
        (
            status = 200,
            description = "Dados do CPF consultados com sucesso",
            body = ConsultaCpfResponse,
            content_type = "application/json"
        ),
        (
            status = 400,
            description = "CPF inválido"
        ),
        (
            status = 502,
            description = "Erro ao consultar API externa"
        )
    ),
    tag = "cpf"
)]
pub async fn consultar_cpf(
    State(state): State<CpfState>,
    Json(payload): Json<ConsultaCpfRequest>,
) -> AppResult<Json<ConsultaCpfResponse>> {
    tracing::info!("Consultando dados do CPF: {}", payload.cpf);

    // 1. Validar CPF
    let cpf_valido = cpf_validator::validate_cpf(&payload.cpf)?;

    // 2. Buscar dados reais na API HighConsult
    let dados_pessoa = state
        .enrichment_service
        .get_person_data(&cpf_valido)
        .await?;

    tracing::info!("Dados obtidos: {}", dados_pessoa.nome);

    // 3. Retornar dados formatados
    Ok(Json(ConsultaCpfResponse {
        cpf: cpf_validator::format_cpf(&cpf_valido),
        nome: dados_pessoa.nome,
        status: "ativo".to_string(), // TODO: determinar status real
    }))
}
