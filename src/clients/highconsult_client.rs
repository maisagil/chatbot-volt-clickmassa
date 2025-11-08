use crate::error::{AppError, AppResult};
use crate::models::external::HighConsultResponse;

#[derive(Clone)]
pub struct HighConsultClient {
    client: reqwest::Client,
    base_url: String,
}

impl HighConsultClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url,
        }
    }

    pub async fn get_person_data(&self, cpf: &str) -> AppResult<HighConsultResponse> {
        let url = format!("{}/dados.php?cpf={}", self.base_url, cpf);

        tracing::debug!("Buscando dados do CPF: {}", cpf);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| {
                AppError::ExternalApiError(format!("Falha ao consultar HighConsult: {}", e))
            })?;

        if !response.status().is_success() {
            let status = response.status();
            tracing::error!("Erro ao buscar dados: status={}", status);
            return Err(AppError::ExternalApiError(format!(
                "Falha ao buscar dados: status={}",
                status
            )));
        }

        let result: HighConsultResponse = response.json().await.map_err(|e| {
            AppError::ExternalApiError(format!("Falha ao parsear resposta: {}", e))
        })?;

        tracing::info!("✅ Dados do CPF obtidos: {}", result.nome);

        Ok(result)
    }
}
