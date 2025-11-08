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
use crate::utils::cpf_validator;

#[derive(Clone)]
pub struct PropostaState {
    pub proposta_service: Arc<PropostaService>,
    pub enrichment_service: Arc<EnrichmentService>,
}

pub fn proposta_routes(state: PropostaState) -> Router {
    Router::new()
        .route("/proposta/criar", post(criar_proposta))
        .route("/operacao/{id}", get(consultar_operacao))
        .with_state(state)
}

/// Criar proposta completa
#[utoipa::path(
    post,
    path = "/proposta/criar",
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

    tracing::info!("CPF validado: {}", cpf_limpo);

    // 2. Buscar dados de endereço via CEP (usando HighConsult)
    let dados_pessoa = state.enrichment_service.get_person_data(&cpf_limpo).await?;

    tracing::info!("Dados da pessoa obtidos: {}", dados_pessoa.nome);

    // 3. Buscar dados de endereço via CEP
    let dados_endereco = state
        .enrichment_service
        .get_address_data(&dados_pessoa.cep)
        .await?;

    tracing::info!(
        "Endereço obtido: {}, {}",
        dados_endereco.logradouro,
        dados_endereco.localidade
    );

    // 4. Parsear telefone
    let telefone_limpo = payload
        .telefone
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>();

    let (ddd, numero) = if telefone_limpo.len() == 11 {
        (&telefone_limpo[0..2], &telefone_limpo[2..11])
    } else if telefone_limpo.len() == 10 {
        (&telefone_limpo[0..2], &telefone_limpo[2..10])
    } else {
        return Err(crate::error::AppError::ValidationError(
            "Telefone inválido. Use formato: 11984353470".to_string(),
        ));
    };

    // 5. Determinar tipo de documento
    let doc_type = if payload.cpf.len() == 14 || payload.cpf.len() == 11 {
        "rg" // Simplificado - em produção seria CNPJ/CPF
    } else {
        "rg"
    };

    // 6. Montar estrutura completa da proposta
    let operation_request = CreateOperationRequest {
        borrower: Borrower {
            name: payload.nome.clone(),
            email: payload.email.clone(),
            phone: BorrowerPhone {
                country_code: "55".to_string(),
                area_code: ddd.to_string(),
                number: numero.to_string(),
            },
            political_exposition: false,
            address: BorrowerAddress {
                postal_code: dados_endereco.cep.replace("-", ""),
                city: dados_endereco.localidade.clone(),
                state: dados_endereco.uf.clone(),
                number: "0".to_string(), // TODO: receber no request
                street: dados_endereco.logradouro.clone(),
                complement: Some(dados_endereco.complemento.clone()),
                neighborhood: dados_endereco.bairro.clone(),
            },
            birth_date: payload.data_nascimento.clone(),
            mother_name: payload.mae.clone(),
            nationality: "Brasileiro".to_string(),
            gender: payload.genero.clone(),
            person_type: "natural".to_string(),
            marital_status: "single".to_string(), // TODO: receber no request
            individual_document_number: cpf_limpo.clone(),
            document_identification_date: "2010-10-10".to_string(), // TODO: receber
            document_issuer: "SSP".to_string(),
            document_identification_type: doc_type.to_string(),
            document_identification_number: cpf_limpo.clone(),
            bank: BorrowerBank {
                transfer_method: "pix".to_string(),
                pix_key: payload.chave_pix.clone(),
                pix_key_type: payload.tipo_chave_pix.clone(),
            },
            work_data: WorkData {
                employer_name: "Empresa".to_string(),      // TODO: receber
                employer_document_number: "00000000000000".to_string(), // TODO: receber
                registration_number: "123456789".to_string(), // TODO: receber
            },
        },
        simulation_id: payload.simulation_id.clone(),
    };

    // 7. Criar operação na API V8
    tracing::info!("Enviando operação para V8...");
    let operation_response = state
        .proposta_service
        .criar_operacao(operation_request)
        .await?;

    tracing::info!(
        "Operação criada com ID: {}",
        operation_response.id
    );
    tracing::info!("Link de formalização: {}", operation_response.formalization_url);

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

    tracing::info!("Operação consultada com status: {}", operation.status);

    Ok(Json(ConsultarOperacaoResponse {
        operation_id: operation.id,
        status: operation.status,
        provider: operation.provider,
        mensagem: "Operação consultada com sucesso".to_string(),
    }))
}
