use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;

use crate::error::AppResult;
use crate::models::chatbot::{
    CriarPropostaRequestCompleta, CriarPropostaResponse, ConsultarOperacaoResponse,
};
use crate::models::v8::*;
use crate::services::enrichment_service::EnrichmentService;
use crate::services::proposta_service::PropostaService;
use crate::services::termo_service::TermoService;
use crate::utils::cpf_validator;

#[derive(Clone)]
pub struct PropostaState {
    pub proposta_service: Arc<PropostaService>,
    pub enrichment_service: Arc<EnrichmentService>,
    pub termo_service: Arc<TermoService>,
}

pub fn proposta_routes(state: PropostaState) -> Router {
    Router::new()
        .route("/proposta/criar", post(criar_proposta))
        .route("/operacao/{id}", get(consultar_operacao))
        .with_state(state)
}

#[utoipa::path(
    post,
    path = "/proposta/criar",
    context_path = "/api/v1",
    request_body = CriarPropostaRequestCompleta,
    responses(
        (status = 200, description = "Proposta criada", body = CriarPropostaResponse),
        (status = 400, description = "Dados inválidos"),
        (status = 502, description = "Erro na API V8")
    ),
    tag = "proposta"
)]
async fn criar_proposta(
    State(state): State<PropostaState>,
    Json(payload): Json<CriarPropostaRequestCompleta>,
) -> AppResult<Json<CriarPropostaResponse>> {
    tracing::info!("Criando proposta completa para CPF: {}", payload.cpf);

    // 1. Validar CPF
    let cpf_limpo = cpf_validator::validate_cpf(&payload.cpf)?;

    // 2. Buscar dados completos do consult_id
    let consult_data = state
        .termo_service
        .get_consult_data(&payload.consult_id)
        .await?;

    // 3. Buscar dados do HighConsult
    let dados_pessoa = state.enrichment_service.get_person_data(&cpf_limpo).await?;

    // 4. Buscar endereço detalhado ViaCEP
    let dados_endereco = state
        .enrichment_service
        .get_address_data(&dados_pessoa.cep)
        .await?;

    // 5. Parsear telefone
    let telefone_limpo: String = consult_data
        .phone_number
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect();

    let (country_code, ddd, numero) = if telefone_limpo.len() >= 13 {
        (&telefone_limpo[0..2], &telefone_limpo[2..4], &telefone_limpo[4..])
    } else if telefone_limpo.len() == 11 {
        ("55", &telefone_limpo[0..2], &telefone_limpo[2..])
    } else {
        return Err(crate::error::AppError::ValidationError(
            "Telefone da consulta está em formato inválido.".to_string(),
        ));
    };

    // 6. Preparar datas - usar valores padrão por enquanto
    // TODO: Corrigir tipos de birth_date e admission_date do consult_data
    let birth_date = "1990-01-01".to_string();
    let document_date = "2010-10-10".to_string();

    // 7. Montar estrutura do request
    let operation_request = CreateOperationRequest {
        borrower: Borrower {
            name: consult_data.name.clone(),
            email: payload.email.clone(),
            phone: BorrowerPhone {
                country_code: country_code.to_string(),
                area_code: ddd.to_string(),
                number: numero.to_string(),
            },
            political_exposition: false,
            address: BorrowerAddress {
                postal_code: dados_endereco.cep.replace("-", ""),
                city: dados_endereco.localidade.clone(),
                state: dados_endereco.uf.clone(),
                number: payload.numero_endereco.clone(),
                street: dados_endereco.logradouro.clone(),
                complement: Some(dados_endereco.complemento.clone()),
                neighborhood: dados_endereco.bairro.clone(),
            },
            birth_date,
            mother_name: dados_pessoa.mae,
            nationality: "Brasileiro".to_string(),
            gender: consult_data.gender.clone(),
            person_type: "natural".to_string(),
            marital_status: "single".to_string(),
            individual_document_number: cpf_limpo.clone(),
            document_identification_date: document_date,
            document_issuer: "SSP".to_string(),
            document_identification_type: "rg".to_string(),
            document_identification_number: cpf_limpo.clone(),
            bank: BorrowerBank {
                transfer_method: "pix".to_string(),
                pix_key: payload.chave_pix.clone(),
                pix_key_type: payload.tipo_chave_pix.clone(),
            },
            work_data: WorkData {
                employer_name: consult_data.employer_name.clone(),
                employer_document_number: consult_data.employer_document_number.clone(),
                registration_number: consult_data.registration_number.clone(),
            },
        },
        simulation_id: payload.simulation_id.clone(),
    };

    let operation_response = state
        .proposta_service
        .criar_operacao(operation_request)
        .await?;

    Ok(Json(CriarPropostaResponse {
        operation_id: operation_response.id,
        formalization_url: operation_response.formalization_url,
        status: "sucesso".to_string(),
        mensagem: "Proposta criada com sucesso! Acesse o link para formalizar.".to_string(),
    }))
}

#[utoipa::path(
    get,
    path = "/operacao/{id}",
    context_path = "/api/v1",
    params(
        ("id" = String, Path, description = "ID da operação")
    ),
    responses(
        (status = 200, description = "Operação consultada", body = ConsultarOperacaoResponse),
        (status = 404, description = "Operação não encontrada")
    ),
    tag = "proposta"
)]
async fn consultar_operacao(
    State(state): State<PropostaState>,
    Path(operation_id): Path<String>,
) -> AppResult<Json<ConsultarOperacaoResponse>> {
    tracing::info!("Consultando operação: {}", operation_id);

    let operation = state
        .proposta_service
        .consultar_operacao(&operation_id)
        .await?;

    Ok(Json(ConsultarOperacaoResponse {
        operation_id: operation.id,
        status: operation.status,
        provider: operation.provider,
        mensagem: "Operação consultada com sucesso".to_string(),
    }))
}
