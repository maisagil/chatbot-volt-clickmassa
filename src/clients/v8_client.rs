use crate::auth::token_manager::TokenManager;
use crate::error::{AppError, AppResult};
use crate::models::v8::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct V8Client {
    client: reqwest::Client,
    base_url: String,
    token_manager: Arc<TokenManager>,
    config_id: String,
    provider: String,
}

impl V8Client {
    pub fn new(
        base_url: String,
        token_manager: Arc<TokenManager>,
        config_id: String,
        provider: String,
    ) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url,
            token_manager,
            config_id,
            provider,
        }
    }

    async fn get_auth_header(&self) -> AppResult<String> {
        let token = self.token_manager.get_token().await?;
        Ok(format!("Bearer {}", token))
    }

    // 1. CRIAR TERMO
    pub async fn create_termo(&self, request: CreateTermoRequest) -> AppResult<CreateTermoResponse> {
        let url = format!("{}/private-consignment/consult", self.base_url);
        let auth = self.get_auth_header().await?;

        tracing::info!("Criando termo para CPF: {}", request.borrower_document_number);

        let response = self
            .client
            .post(&url)
            .header("Authorization", auth)
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::V8Error(format!("Falha na requisição: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            tracing::error!("Erro ao criar termo: status={}, body={}", status, error_text);
            return Err(AppError::V8Error(format!(
                "Falha ao criar termo: status={}, body={}",
                status, error_text
            )));
        }

        let result: CreateTermoResponse = response.json().await.map_err(|e| {
            AppError::V8Error(format!("Falha ao parsear resposta: {}", e))
        })?;

        tracing::info!("Termo criado com ID: {}", result.id);
        Ok(result)
    }

    // 2. GET TERMO

    pub async fn get_termo(&self, termo_id: &str) -> AppResult<String> {
        let url = format!("{}/termos-de-autorizacao/{}", self.base_url, termo_id);
        let auth = self.get_auth_header().await?;

        tracing::debug!("Buscando termo: {}", termo_id);

        let response = self
            .client
            .get(&url)
            .header("Authorization", auth)
            .send()
            .await
            .map_err(|e| AppError::V8Error(format!("Falha na requisição: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::V8Error(format!(
                "Falha ao buscar termo: {}",
                response.status()
            )));
        }

        let html = response.text().await.map_err(|e| {
            AppError::V8Error(format!("Falha ao ler resposta: {}", e))
        })?;

        Ok(html)
    }

    // 3. ACEITAR TERMO (GET)

    pub async fn accept_termo(&self, termo_id: &str, cpf: &str) -> AppResult<String> {
        let url = format!(
            "{}/private-consignment/consult/{}/unprotected/{}",
            self.base_url, termo_id, cpf
        );
        let auth = self.get_auth_header().await?;

        tracing::debug!("Aceitando termo: {} para CPF: {}", termo_id, cpf);

        let response = self
            .client
            .get(&url)
            .header("Authorization", auth)
            .send()
            .await
            .map_err(|e| AppError::V8Error(format!("Falha na requisição: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::V8Error(format!(
                "Falha ao aceitar termo: {}",
                response.status()
            )));
        }

        let html = response.text().await.map_err(|e| {
            AppError::V8Error(format!("Falha ao ler resposta: {}", e))
        })?;

        Ok(html)
    }

    // 4. AUTORIZAR TERMO (POST)
    
    pub async fn authorize_termo(&self, termo_id: &str) -> AppResult<String> {
        let url = format!(
            "{}/private-consignment/consult/{}/authorize",
            self.base_url, termo_id
        );
        let auth = self.get_auth_header().await?;

        tracing::info!("Autorizando termo: {}", termo_id);

        let response = self
            .client
            .post(&url)
            .header("Authorization", auth)
            .send()
            .await
            .map_err(|e| AppError::V8Error(format!("Falha na requisição: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::V8Error(format!(
                "Falha ao autorizar termo: {}",
                response.status()
            )));
        }

        let html = response.text().await.map_err(|e| {
            AppError::V8Error(format!("Falha ao ler resposta: {}", e))
        })?;

        Ok(html)
    }

    // 5. CONSULTAR DADOS (GET)
    
    pub async fn get_consult_data(&self, consult_id: &str) -> AppResult<ConsultDataResponse> {
        let url = format!("{}/private-consignment/consult/{}", self.base_url, consult_id);
        let auth = self.get_auth_header().await?;

        tracing::info!("📊 Consultando dados: {}", consult_id);

        let response = self
            .client
            .get(&url)
            .header("Authorization", auth)
            .send()
            .await
            .map_err(|e| AppError::V8Error(format!("Falha na requisição: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            tracing::error!("Erro ao consultar dados: status={}, body={}", status, error_text);
            return Err(AppError::V8Error(format!(
                "Falha ao consultar dados: status={}, body={}",
                status, error_text
            )));
        }

        let result: ConsultDataResponse = response.json().await.map_err(|e| {
            AppError::V8Error(format!("Falha ao parsear resposta: {}", e))
        })?;

        tracing::info!("Dados consultados com sucesso");
        tracing::debug!("Margem disponível: R$ {}", result.margin_base_value);

        Ok(result)
    }

    // 6. CRIAR SIMULAÇÃO
    
    pub async fn create_simulation(
        &self,
        request: CreateSimulationRequest,
    ) -> AppResult<SimulationResponse> {
        let url = format!("{}/private-consignment/simulation", self.base_url);
        let auth = self.get_auth_header().await?;

        tracing::info!(
            "Criando simulação: {} parcelas de R$ {}",
            request.number_of_installments,
            request.installment_face_value
        );

        let response = self
            .client
            .post(&url)
            .header("Authorization", auth)
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::V8Error(format!("Falha na requisição: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            tracing::error!(
                "Erro ao criar simulação: status={}, body={}",
                status,
                error_text
            );
            return Err(AppError::V8Error(format!(
                "Falha ao criar simulação: status={}, body={}",
                status, error_text
            )));
        }

        let result: SimulationResponse = response.json().await.map_err(|e| {
            AppError::V8Error(format!("Falha ao parsear resposta: {}", e))
        })?;

        tracing::info!(
            "Simulação criada: R$ {} em {} parcelas",
            result.operation_amount,
            result.number_of_installments
        );

        Ok(result)
    }

    // 7. CRIAR OPERAÇÃO

    pub async fn create_operation(
        &self,
        request: CreateOperationRequest,
    ) -> AppResult<CreateOperationResponse> {
        let url = format!("{}/private-consignment/operation", self.base_url);
        let auth = self.get_auth_header().await?;

        tracing::info!(
            "Criando operação para: {}",
            request.borrower.individual_document_number
        );

        let response = self
            .client
            .post(&url)
            .header("Authorization", auth)
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::V8Error(format!("Falha na requisição: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            tracing::error!(
                "Erro ao criar operação: status={}, body={}",
                status,
                error_text
            );
            return Err(AppError::V8Error(format!(
                "Falha ao criar operação: status={}, body={}",
                status, error_text
            )));
        }

        let result: CreateOperationResponse = response.json().await.map_err(|e| {
            AppError::V8Error(format!("Falha ao parsear resposta: {}", e))
        })?;

        tracing::info!("Operação criada com ID: {}", result.id);
        tracing::info!("Link de formalização: {}", result.formalization_url);

        Ok(result)
    }

    // 8. CONSULTAR OPERAÇÃO
    
    pub async fn get_operation(&self, operation_id: &str) -> AppResult<OperationResponse> {
        let url = format!(
            "{}/private-consignment/operation/{}?provider={}",
            self.base_url, operation_id, self.provider
        );
        let auth = self.get_auth_header().await?;

        tracing::debug!("🔍 Consultando operação: {}", operation_id);

        let response = self
            .client
            .get(&url)
            .header("Authorization", auth)
            .send()
            .await
            .map_err(|e| AppError::V8Error(format!("Falha na requisição: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::V8Error(format!(
                "Falha ao consultar operação: {}",
                response.status()
            )));
        }

        let result: OperationResponse = response.json().await.map_err(|e| {
            AppError::V8Error(format!("Falha ao parsear resposta: {}", e))
        })?;

        tracing::info!("Operação consultada com status: {}", result.status);

        Ok(result)
    }

    pub fn get_config_id(&self) -> &str {
        &self.config_id
    }
}
