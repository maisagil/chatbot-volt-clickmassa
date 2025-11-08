use moka::future::Cache;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

#[derive(Clone, Debug, Serialize, Deserialize)] 
pub struct PropostaContexto {
    pub cpf: String,
    pub nome: String,
    pub email: String,
    pub simulation_id: String,
    pub valor_total: f64,
    pub parcelas: i32,
}

#[derive(Clone)]
pub struct CacheService {
    cache: Arc<Cache<String, String>>,
}

impl CacheService {
    pub fn new(ttl_seconds: u64) -> Self {
        let cache = Cache::builder()
            .time_to_live(Duration::from_secs(ttl_seconds))
            .build();

        Self {
            cache: Arc::new(cache),
        }
    }

    pub async fn set_contexto(
        &self,
        key: &str,
        contexto: &PropostaContexto,
    ) -> Result<(), String> {
        let json = serde_json::to_string(contexto)
            .map_err(|e| format!("Erro ao serializar: {}", e))?;
        self.cache.insert(key.to_string(), json).await;
        Ok(())
    }

    pub async fn get_contexto(&self, key: &str) -> Option<PropostaContexto> {
        self.cache
            .get(key)
            .await
            .and_then(|json| serde_json::from_str(&json).ok())
    }

    pub async fn invalidate(&self, key: &str) {
        self.cache.invalidate(key).await;
    }
}
