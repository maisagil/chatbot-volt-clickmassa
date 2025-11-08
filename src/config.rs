use crate::error::{AppError, AppResult};
use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    // Servidor
    pub host: String,
    pub port: u16,
    pub environment: String,

    // V8 Sistema - URLs e credenciais (dinâmicas conforme ambiente)
    pub v8_auth_url: String,
    pub v8_base_url: String,
    pub v8_client_id: String,
    pub v8_username: String,
    pub v8_password: String,
    pub v8_audience: String,
    pub v8_config_id: String,
    pub v8_provider: String,

    // APIs Externas
    pub highconsult_api_url: String,
    pub viacep_api_url: String,

    // Cache
    pub token_cache_ttl_seconds: u64,

    // Logging
    pub rust_log: String,

    // API Key (opcional)
    pub api_key: Option<String>,
}

impl Config {
    pub fn from_env() -> AppResult<Self> {
        // Carregar variáveis de ambiente
        dotenvy::dotenv().ok();

        let environment = env::var("ENVIRONMENT")
            .unwrap_or_else(|_| "staging".to_string())
            .to_lowercase();

        // Construir sufixo das variáveis conforme ambiente
        let suffix = if environment == "production" {
            "_PROD".to_string()
        } else {
            "_STAGING".to_string()
        };

        // Função auxiliar para buscar variáveis com erro customizado
        let get_var = |key: &str| -> AppResult<String> {
            env::var(key).map_err(|_| {
                AppError::ConfigError(format!("Variável de ambiente não encontrada: {}", key))
            })
        };

        let config = Config {
            // Servidor
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .map_err(|_| AppError::ConfigError("PORT deve ser um número".to_string()))?,
            environment: environment.clone(),

            // V8 Sistema
            v8_auth_url: get_var(&format!("V8_AUTH_URL{}", suffix))?,
            v8_base_url: get_var(&format!("V8_BASE_URL{}", suffix))?,
            v8_client_id: get_var(&format!("V8_CLIENT_ID{}", suffix))?,
            v8_username: get_var(&format!("V8_USERNAME{}", suffix))?,
            v8_password: get_var(&format!("V8_PASSWORD{}", suffix))?,
            v8_audience: get_var(&format!("V8_AUDIENCE{}", suffix))?,
            v8_config_id: get_var("V8_CONFIG_ID")?,
            v8_provider: env::var("V8_PROVIDER").unwrap_or_else(|_| "QI".to_string()),

            // APIs Externas
            highconsult_api_url: get_var("HIGHCONSULT_API_URL")?,
            viacep_api_url: get_var("VIACEP_API_URL")?,

            // Cache
            token_cache_ttl_seconds: env::var("TOKEN_CACHE_TTL_SECONDS")
                .unwrap_or_else(|_| "3600".to_string())
                .parse()
                .map_err(|_| {
                    AppError::ConfigError(
                        "TOKEN_CACHE_TTL_SECONDS deve ser um número".to_string(),
                    )
                })?,

            // Logging
            rust_log: env::var("RUST_LOG")
                .unwrap_or_else(|_| "info,chatbot_volt_clickmassa=debug".to_string()),

            // API Key
            api_key: env::var("API_KEY").ok(),
        };

        Ok(config)
    }
}
