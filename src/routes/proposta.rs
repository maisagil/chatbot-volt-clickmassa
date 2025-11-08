use axum::{
    extract::{Path, State}, 
    routing::{get, post},
    Json, Router,
};

use std::sync::Arc;

use crate::error::AppResult;
use crate::models::chatbot::{
    CriarPropostaRequest, CriarPropostaResponse, ConsultarOperacaoResponse,
};
use crate::models::v8::*;
use crate::services::enrichment_service::EnrichmentService;
use crate::services::proposta_service::PropostaService;
use crate::utils::cpf_validator;

#[derive(Clone)]
pub struct PropostaState {
    pub proposta_service: Arc<PropostaService>,
    pub enrichment_service: Arc<EnrichmentService>,
}

pub fn proposta_routes(state: PropostaState) -> Router {
    Router::new()
        .route("/proposta/criar", post(criar_proposta))
        .route("/operacao/:id", get(consultar_operacao))
        .with_state(state)
}

/// POST /proposta/criar
/// Cria operação/proposta com dados completos
async fn criar_proposta(
    State(state): State<PropostaState>,
    Json(payload): Json<CriarPropostaRequest>,
) -> AppResult<Json<CriarPropostaResponse>> {
    tracing::info!(
        "📋 Criando proposta com simulation_id: {}",
        payload.simulation_id
    );

    // TODO: Implementar fluxo completo
    // 1. Buscar dados da simulação
    // 2. Buscar dados do CPF e enriquecê-los
    // 3. Buscar dados do CEP
    // 4. Montar estrutura completa da proposta
    // 5. Chamar API V8 para criar operação

    // Por enquanto, retorna mock
    Ok(Json(CriarPropostaResponse {
        operation_id: "OP-2025-001".to_string(),
        formalization_url: "https://v8digital.com/proposta/abc123".to_string(),
        status: "sucesso".to_string(),
        mensagem: "Proposta criada com sucesso. Acesse o link para formalizar.".to_string(),
    }))
}

/// GET /operacao/:id
/// Consulta status de uma operação
async fn consultar_operacao(
    State(state): State<PropostaState>,
    Path(operation_id): Path<String>,
) -> AppResult<Json<ConsultarOperacaoResponse>> {
    tracing::info!("🔍 Consultando operação: {}", operation_id);

    let operation = state
        .proposta_service
        .consultar_operacao(&operation_id)
        .await?;

    tracing::info!("Operação consultada com status: {}", operation.status);

    Ok(Json(ConsultarOperacaoResponse {
        operation_id: operation.id,
        status: operation.status,
        provider: operation.provider,
        mensagem: "Operação consultada com sucesso".to_string(),
    }))
}
