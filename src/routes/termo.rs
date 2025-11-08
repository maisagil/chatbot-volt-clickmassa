use axum::{
    extract::{Json, State},
    routing::post,
    Router,
};
use std::sync::Arc;
use utoipa::path;

use crate::error::AppResult;
use crate::models::chatbot::{
    AutorizarTermoRequest, AutorizarTermoResponse, CriarTermoRequest, CriarTermoResponse,
};
use crate::models::v8::PhoneNumber;
use crate::services::enrichment_service::EnrichmentService;
use crate::services::termo_service::TermoService;
use crate::utils::cpf_validator;

#[derive(Clone)]
pub struct TermoState {
    pub termo_service: Arc<TermoService>,
    pub enrichment_service: Arc<EnrichmentService>,
}

pub fn termo_routes(state: TermoState) -> Router {
    Router::new()
        .route("/termo/criar", post(criar_termo))
        .route("/termo/autorizar", post(autorizar_termo))
        .with_state(state)
}

/// Criar termo de autorização
#[utoipa::path(
    post,
    path = "/termo/criar",
    context_path = "/api/v1", 
    request_body = CriarTermoRequest,
    responses(
        (status = 200, description = "Termo criado com sucesso", body = CriarTermoResponse),
        (status = 400, description = "Erro na validação"),
        (status = 502, description = "Erro na API V8")
    ),
    tag = "termo"
)]
async fn criar_termo(
    State(state): State<TermoState>,
    Json(payload): Json<CriarTermoRequest>,
) -> AppResult<Json<CriarTermoResponse>> {
    tracing::info!("📝 Criando termo para CPF: {}", payload.cpf);

    let cpf_limpo = cpf_validator::validate_cpf(&payload.cpf)?;
    let dados_pessoa = state.enrichment_service.get_person_data(&cpf_limpo).await?;

    tracing::info!("✅ Dados obtidos: {}", dados_pessoa.nome);

    let telefone_limpo = payload.telefone.chars().filter(|c| c.is_ascii_digit()).collect::<String>();
    
    let (ddd, numero) = if telefone_limpo.len() == 11 {
        (&telefone_limpo[0..2], &telefone_limpo[2..11])
    } else if telefone_limpo.len() == 10 {
        (&telefone_limpo[0..2], &telefone_limpo[2..10])
    } else {
        return Err(crate::error::AppError::ValidationError(
            "Telefone inválido. Use formato: 11984353470".to_string(),
        ));
    };

    let termo_request = crate::models::v8::CreateTermoRequest {
        borrower_document_number: cpf_limpo.clone(),
        signer_name: dados_pessoa.nome.clone(),
        signer_email: payload.email.clone(),
        signer_phone: PhoneNumber {
            country_code: "55".to_string(),
            area_code: ddd.to_string(),
            phone_number: numero.to_string(),
        },
        birth_date: format!(
            "{}-{}-{}",
            &dados_pessoa.nasc[0..4],
            &dados_pessoa.nasc[4..6],
            &dados_pessoa.nasc[6..8]
        ),
        gender: "male".to_string(),
        provider: "QI".to_string(),
    };

    let termo_response = state.termo_service.criar_termo(termo_request).await?;

    tracing::info!("✅ Termo criado com ID: {}", termo_response.id);

    Ok(Json(CriarTermoResponse {
        termo_id: termo_response.id,
        status: "sucesso".to_string(),
        mensagem: format!(
            "Termo criado com sucesso para {}. Aguardando autorização.",
            dados_pessoa.nome
        ),
    }))
}

/// Autorizar termo após assinatura
#[utoipa::path(
    post,
    path = "/termo/autorizar",
    context_path = "/api/v1", 
    request_body = AutorizarTermoRequest,
    responses(
        (status = 200, description = "Termo autorizado com sucesso", body = AutorizarTermoResponse),
        (status = 400, description = "Termo ID inválido"),
        (status = 502, description = "Erro na API V8")
    ),
    tag = "termo"
)]
async fn autorizar_termo(
    State(state): State<TermoState>,
    Json(payload): Json<AutorizarTermoRequest>,
) -> AppResult<Json<AutorizarTermoResponse>> {
    tracing::info!("🔐 Autorizando termo: {}", payload.termo_id);

    state.termo_service.autorizar_termo(&payload.termo_id).await?;

    let consult_data = state
        .termo_service
        .get_consult_data(&payload.termo_id)
        .await?;

    tracing::info!(
        "✅ Termo autorizado! Margem disponível: R$ {}",
        consult_data.margin_base_value
    );

    Ok(Json(AutorizarTermoResponse {
        consult_id: consult_data.id,
        nome: consult_data.name,
        margem_disponivel: consult_data.margin_base_value.clone(),
        parcelas_min: consult_data.simulation_limit.installments_min,
        parcelas_max: consult_data.simulation_limit.installments_max,
        status: consult_data.status,
        mensagem: format!(
            "Termo autorizado! Margem disponível: R$ {}",
            consult_data.margin_base_value
        ),
    }))
}
