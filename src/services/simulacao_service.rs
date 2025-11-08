use crate::clients::v8_client::V8Client;
use crate::error::AppResult;
use crate::models::v8::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct SimulacaoService {
    v8_client: Arc<V8Client>,
}

impl SimulacaoService {
    pub fn new(v8_client: Arc<V8Client>) -> Self {
        Self { v8_client }
    }

    /// Gerar simulações para múltiplas parcelas
    pub async fn gerar_simulacoes(
        &self,
        consult_id: &str,
        valor_base: f64,
        min_parcelas: i32,
        max_parcelas: i32,
    ) -> AppResult<Vec<SimulationResponse>> {
        tracing::info!(
            "Gerando simulações de {} a {} parcelas para consult_id: {}",
            min_parcelas,
            max_parcelas,
            consult_id
        );

        let mut simulacoes = Vec::new();
        let config_id = self.v8_client.get_config_id().to_string();

        // Gerar simulações para: 6, 8, 10, 12, 18, 24 parcelas
        let parcelas_disponiveis = vec![6, 8, 10, 12, 18, 24];

        for parcelas in parcelas_disponiveis {
            // Verificar se está dentro dos limites
            if parcelas < min_parcelas || parcelas > max_parcelas {
                tracing::debug!("Pulando simulação de {} parcelas (fora dos limites)", parcelas);
                continue;
            }

            let request = CreateSimulationRequest {
                consult_id: consult_id.to_string(),
                number_of_installments: parcelas,
                installment_face_value: valor_base,
                config_id: config_id.clone(),
            };

            match self.v8_client.create_simulation(request).await {
                Ok(sim) => {
                    tracing::info!(
                        "Simulação de {}x criada: R$ {}",
                        parcelas,
                        sim.installment_value
                    );
                    simulacoes.push(sim);
                }
                Err(e) => {
                    tracing::warn!("Falha ao simular {}x: {}", parcelas, e);
                    // Continua com próxima parcela em caso de erro
                }
            }
        }

        if simulacoes.is_empty() {
            return Err(crate::error::AppError::V8Error(
                "Nenhuma simulação foi gerada".to_string(),
            ));
        }

        tracing::info!("Total de {} simulações geradas", simulacoes.len());
        Ok(simulacoes)
    }

    /// Gerar uma simulação específica
    pub async fn gerar_simulacao(
        &self,
        consult_id: &str,
        numero_parcelas: i32,
        valor_parcela: f64,
    ) -> AppResult<SimulationResponse> {
        tracing::info!(
            "Gerando simulação: {}x de R$ {}",
            numero_parcelas,
            valor_parcela
        );

        let request = CreateSimulationRequest {
            consult_id: consult_id.to_string(),
            number_of_installments: numero_parcelas,
            installment_face_value: valor_parcela,
            config_id: self.v8_client.get_config_id().to_string(),
        };

        self.v8_client.create_simulation(request).await
    }
}
