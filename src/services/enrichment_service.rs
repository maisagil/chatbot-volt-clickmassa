use crate::clients::highconsult_client::HighConsultClient;
use crate::clients::viacep_client::ViaCepClient;
use crate::error::AppResult;
use crate::models::external::{HighConsultResponse, ViaCepResponse};

#[derive(Clone)]
pub struct EnrichmentService {
    highconsult_client: HighConsultClient,
    viacep_client: ViaCepClient,
}

impl EnrichmentService {
    pub fn new(highconsult_client: HighConsultClient, viacep_client: ViaCepClient) -> Self {
        Self {
            highconsult_client,
            viacep_client,
        }
    }

    /// Buscar dados de pessoa física pelo CPF
    pub async fn get_person_data(&self, cpf: &str) -> AppResult<HighConsultResponse> {
        tracing::info!("Enriquecendo dados do CPF: {}", cpf);
        self.highconsult_client.get_person_data(cpf).await
    }

    /// Buscar dados de endereço pelo CEP
    pub async fn get_address_data(&self, cep: &str) -> AppResult<ViaCepResponse> {
        tracing::info!("Enriquecendo dados do CEP: {}", cep);
        self.viacep_client.get_address(cep).await
    }
}
