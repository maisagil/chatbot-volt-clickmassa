use crate::cache::token_cache::TokenCache;
use crate::error::{AppError, AppResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub expires_in: u64,
    pub token_type: String,
}

#[derive(Clone)]
pub struct TokenManager {
    cache: TokenCache,
    client: reqwest::Client,
    auth_url: String,
    client_id: String,
    username: String,
    password: String,
    audience: String,
}

impl TokenManager {
    pub fn new(
        auth_url: String,
        client_id: String,
        username: String,
        password: String,
        audience: String,
        cache_ttl_seconds: u64,
    ) -> Self {
        Self {
            cache: TokenCache::new(cache_ttl_seconds),
            client: reqwest::Client::new(),
            auth_url,
            client_id,
            username,
            password,
            audience,
        }
    }

    pub async fn get_token(&self) -> AppResult<String> {
        let cache_key = "v8_access_token";

        // Tenta buscar do cache
        if let Some(token) = self.cache.get(cache_key).await {
            tracing::debug!("Token encontrado no cache");
            return Ok(token);
        }

        // Se não estiver no cache, autentica
        tracing::info!("Token não encontrado no cache, autenticando...");
        let token = self.authenticate().await?;

        // Armazena no cache
        self.cache
            .set(cache_key.to_string(), token.clone())
            .await;

        Ok(token)
    }

    async fn authenticate(&self) -> AppResult<String> {
        let params = [
            ("grant_type", "password"),
            ("username", &self.username),
            ("password", &self.password),
            ("audience", &self.audience),
            ("scope", "offline_access"),
            ("client_id", &self.client_id),
        ];

        tracing::debug!("Autenticando com V8 Sistema...");
        tracing::debug!("Auth URL: {}", self.auth_url);

        let response = self
            .client
            .post(&self.auth_url)
            .form(&params)
            .send()
            .await
            .map_err(|e| AppError::AuthError(format!("Falha na requisição: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            tracing::error!("Falha na autenticação: status={}, body={}", status, error_text);
            return Err(AppError::AuthError(format!(
                "Falha na autenticação: status={}, body={}",
                status, error_text
            )));
        }

        let token_response: TokenResponse = response.json().await.map_err(|e| {
            AppError::AuthError(format!("Falha ao parsear resposta de token: {}", e))
        })?;

        tracing::info!("Autenticação V8 bem-sucedida");
        tracing::debug!("Token expira em {} segundos", token_response.expires_in);

        Ok(token_response.access_token)
    }

    pub async fn invalidate_cache(&self) {
        self.cache.invalidate_all().await;
        tracing::info!("Cache de token invalidado");
    }
}
