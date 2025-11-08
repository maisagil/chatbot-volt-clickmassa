use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub environment: String,
    pub host: String,
    pub port: u16,
    
    // V8
    pub v8_auth_url: String,
    pub v8_base_url: String,
    pub v8_client_id: String,
    pub v8_client_secret: Option<String>,
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
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let environment = env::var("ENVIRONMENT").unwrap_or_else(|_| "staging".to_string());
        
        Ok(Config {
            environment: environment.clone(),
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .map_err(|_| "PORT deve ser um número".to_string())?,
            
            // V8
            v8_auth_url: env::var("V8_AUTH_URL")
                .map_err(|_| "V8_AUTH_URL não configurada".to_string())?,
            v8_base_url: env::var("V8_BASE_URL")
                .map_err(|_| "V8_BASE_URL não configurada".to_string())?,
            v8_client_id: env::var("V8_CLIENT_ID")
                .map_err(|_| "V8_CLIENT_ID não configurada".to_string())?,
            v8_client_secret: env::var("V8_CLIENT_SECRET").ok(),
            v8_username: env::var("V8_USERNAME")
                .map_err(|_| "V8_USERNAME não configurada".to_string())?,
            v8_password: env::var("V8_PASSWORD")
                .map_err(|_| "V8_PASSWORD não configurada".to_string())?,
            v8_audience: env::var("V8_AUDIENCE")
                .map_err(|_| "V8_AUDIENCE não configurada".to_string())?,
            v8_config_id: env::var("V8_CONFIG_ID")
                .map_err(|_| "V8_CONFIG_ID não configurada".to_string())?,
            v8_provider: env::var("V8_PROVIDER")
                .map_err(|_| "V8_PROVIDER não configurada".to_string())?,
            
            // APIs Externas
            highconsult_api_url: env::var("HIGHCONSULT_API_URL")
                .unwrap_or_else(|_| "https://telefone.highconsult.net".to_string()),
            viacep_api_url: env::var("VIACEP_API_URL")
                .unwrap_or_else(|_| "https://viacep.com.br/ws".to_string()),
            
            // Cache
            token_cache_ttl_seconds: env::var("TOKEN_CACHE_TTL_SECONDS")
                .unwrap_or_else(|_| "3600".to_string())
                .parse()
                .unwrap_or(3600),
            
            // Logging
            rust_log: env::var("RUST_LOG")
                .unwrap_or_else(|_| "info".to_string()),
        })
    }
}
