use axum::Router;
use std::sync::Arc;

use crate::clients::{
    highconsult_client::HighConsultClient, viacep_client::ViaCepClient,
    v8_client::V8Client,
};
use crate::services::{
    enrichment_service::EnrichmentService, proposta_service::PropostaService,
    simulacao_service::SimulacaoService, termo_service::TermoService,
};

use super::{proposta, simulacao, termo, pix, cpf};

pub fn v1_routes(
    v8_client: Arc<V8Client>,
    highconsult_client: HighConsultClient,
    viacep_client: ViaCepClient,
) -> Router {
    let termo_service = Arc::new(TermoService::new(v8_client.clone()));
    let simulacao_service = Arc::new(SimulacaoService::new(v8_client.clone()));
    let proposta_service = Arc::new(PropostaService::new(v8_client.clone()));
    let enrichment_service = Arc::new(EnrichmentService::new(
        highconsult_client,
        viacep_client,
    ));

    Router::new()
        .merge(cpf::cpf_routes(cpf::CpfState {
            enrichment_service: enrichment_service.clone(),
        }))
        .merge(termo::termo_routes(termo::TermoState {
            termo_service: termo_service.clone(),  
            enrichment_service: enrichment_service.clone(),
        }))
        .merge(simulacao::simulacao_routes(simulacao::SimulacaoState {
            simulacao_service,
        }))
        .merge(proposta::proposta_routes(proposta::PropostaState {
            proposta_service,
            enrichment_service,
            termo_service: termo_service.clone(), 
        }))
        .merge(pix::pix_routes())
}
