use crate::clients::v8_client::V8Client;
use crate::error::{AppError, AppResult};
use crate::models::v8::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct TermoService {
    v8_client: Arc<V8Client>,
}

impl TermoService {
    pub fn new(v8_client: Arc<V8Client>) -> Self {
        Self { v8_client }
    }

    /// Criar novo termo de autorização
    pub async fn criar_termo(&self, request: CreateTermoRequest) -> AppResult<CreateTermoResponse> {
        tracing::info!("Iniciando criação de termo para CPF: {}", request.borrower_document_number);
        
        let response = self.v8_client.create_termo(request).await?;
        
        tracing::info!("Termo criado com sucesso! ID: {}", response.id);
        Ok(response)
    }

    /// Obter URL do termo para assinatura
    pub async fn get_termo_url(&self, termo_id: &str) -> AppResult<String> {
        tracing::debug!("Obtendo URL do termo: {}", termo_id);
        self.v8_client.get_termo(termo_id).await
    }

    /// Autorizar termo (após assinatura)
    pub async fn autorizar_termo(&self, termo_id: &str) -> AppResult<String> {
        tracing::info!("Autorizando termo: {}", termo_id);
        self.v8_client.authorize_termo(termo_id).await
    }

    /// Buscar dados da consulta após autorização
    pub async fn get_consult_data(&self, consult_id: &str) -> AppResult<ConsultDataResponse> {
        tracing::info!("Buscando dados da consulta: {}", consult_id);
        self.v8_client.get_consult_data(consult_id).await
    }
}
