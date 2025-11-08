use crate::error::{AppError, AppResult};
use crate::models::external::ViaCepResponse;

#[derive(Clone)]
pub struct ViaCepClient {
    client: reqwest::Client,
    base_url: String,
}

impl ViaCepClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url,
        }
    }

    pub async fn get_address(&self, cep: &str) -> AppResult<ViaCepResponse> {
        // Remover caracteres especiais do CEP
        let cep_clean = cep.replace("-", "").replace(".", "");
        let url = format!("{}/{}/json/", self.base_url, cep_clean);

        tracing::debug!("Buscando endereço para CEP: {}", cep);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| AppError::ExternalApiError(format!("Falha ao consultar ViaCEP: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            tracing::error!("Erro ao buscar endereço: status={}", status);
            return Err(AppError::ExternalApiError(format!(
                "Falha ao buscar endereço: status={}",
                status
            )));
        }

        let result: ViaCepResponse = response.json().await.map_err(|e| {
            AppError::ExternalApiError(format!("Falha ao parsear resposta: {}", e))
        })?;

        tracing::info!(
            "✅ Endereço obtido: {}, {}",
            result.logradouro,
            result.localidade
        );

        Ok(result)
    }
}
