use axum::{
    extract::State,
    routing::post,
    Json, Router,
};
use std::sync::Arc;

use crate::error::AppResult;
use crate::models::chatbot::{GerarSimulacoesRequest, GerarSimulacoesResponse, SimulacaoResumo};
use crate::services::simulacao_service::SimulacaoService;

#[derive(Clone)]
pub struct SimulacaoState {
    pub simulacao_service: Arc<SimulacaoService>,
}

pub fn simulacao_routes(state: SimulacaoState) -> Router {
    Router::new()
        .route("/simulacao/gerar", post(gerar_simulacoes))
        .with_state(state)
}

/// POST /simulacao/gerar
/// Gera múltiplas simulações com diferentes parcelamentos
async fn gerar_simulacoes(
    State(state): State<SimulacaoState>,
    Json(payload): Json<GerarSimulacoesRequest>,
) -> AppResult<Json<GerarSimulacoesResponse>> {
    tracing::info!(
        "💰 Gerando simulações para consult_id: {}",
        payload.consult_id
    );

    // 1. Buscar dados da consulta para pegar limites
    // TODO: Implementar busca de dados se necessário

    // 2. Gerar simulações (6, 8, 10, 12, 18, 24 parcelas)
    let valor_base = 1000.0; // TODO: receber como parâmetro

    let simulacoes_v8 = state
        .simulacao_service
        .gerar_simulacoes(&payload.consult_id, valor_base, 6, 24)
        .await?;

    tracing::info!(
        "✅ {} simulações geradas com sucesso",
        simulacoes_v8.len()
    );

    // 3. Formatar resposta para o chatbot
    let simulacoes_resumo: Vec<SimulacaoResumo> = simulacoes_v8
        .into_iter()
        .map(|sim| SimulacaoResumo {
            parcelas: sim.number_of_installments,
            valor_parcela: sim.installment_value,
            valor_total: sim.operation_amount,
            valor_liberado: sim.disbursement_amount,
            taxa_juros_mensal: sim.monthly_interest_rate,
            primeira_parcela: sim.first_installment_date,
            simulation_id: sim.id_simulation,
        })
        .collect();

    Ok(Json(GerarSimulacoesResponse {
        simulacoes: simulacoes_resumo,
        status: "sucesso".to_string(),
        mensagem: "Simulações geradas com sucesso".to_string(),
    }))
}
