use crate::clients::v8_client::V8Client;
use crate::error::AppResult;
use crate::models::v8::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct PropostaService {
    v8_client: Arc<V8Client>,
}

impl PropostaService {
    pub fn new(v8_client: Arc<V8Client>) -> Self {
        Self { v8_client }
    }

    /// Criar operação/proposta
    pub async fn criar_operacao(
        &self,
        request: CreateOperationRequest,
    ) -> AppResult<CreateOperationResponse> {
        tracing::info!(
            "Criando operação para: {}",
            request.borrower.individual_document_number
        );

        let response = self.v8_client.create_operation(request).await?;

        tracing::info!("Operação criada com ID: {}", response.id);
        tracing::info!("Link de formalização: {}", response.formalization_url);

        Ok(response)
    }

    /// Consultar status de operação
    pub async fn consultar_operacao(&self, operation_id: &str) -> AppResult<OperationResponse> {
        tracing::info!("Consultando operação: {}", operation_id);
        self.v8_client.get_operation(operation_id).await
    }
}
