mod config;
mod error;
mod routes;
mod models;
mod cache;
mod auth;
mod clients;
mod services;
mod utils;
mod docs;

use axum::{
    body::Body,
    http::Request,
    middleware::Next,
    response::Response,
    Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use docs::ApiDoc;

#[tokio::main]
async fn main() {
    let config = match config::Config::from_env() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Erro ao carregar configuração: {}", e);
            std::process::exit(1);
        }
    };

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::new(&config.rust_log),
        )
        .init();

    tracing::info!("Iniciando Chatbot Volt Crédito Middleware");
    tracing::info!("Ambiente: {}", config.environment);
    tracing::info!("V8 Base URL: {}", config.v8_base_url);

    let token_manager = auth::token_manager::TokenManager::new(
        config.v8_auth_url.clone(),
        config.v8_client_id.clone(),
        config.v8_username.clone(),
        config.v8_password.clone(),
        config.v8_audience.clone(),
        config.token_cache_ttl_seconds,
    );

    tracing::info!("Testando autenticação com V8...");
    match token_manager.get_token().await {
        Ok(_) => tracing::info!("Autenticação V8 funcionando!"),
        Err(e) => {
            tracing::error!("Falha na autenticação V8: {}", e);
            tracing::warn!("Servidor continuará, mas chamadas à API V8 falharão");
        }
    }

    let token_manager = Arc::new(token_manager);
    let v8_client = Arc::new(clients::v8_client::V8Client::new(
        config.v8_base_url.clone(),
        token_manager.clone(),
        config.v8_config_id.clone(),
        config.v8_provider.clone(),
    ));

    let highconsult_client =
        clients::highconsult_client::HighConsultClient::new(config.highconsult_api_url.clone());

    let viacep_client =
        clients::viacep_client::ViaCepClient::new(config.viacep_api_url.clone());

    // Construir aplicação - SwaggerUi JÁ serve o JSON automaticamente
    let app = Router::new()
        // SwaggerUi registra AMBOS: /swagger-ui E /api-docs/openapi.json
        .merge(
            SwaggerUi::new("/swagger-ui")
                .url("/api-docs/openapi.json", ApiDoc::openapi())
        )
        .merge(routes::routes())
        .nest("/api/v1", routes::v1_routes(v8_client, highconsult_client, viacep_client))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .layer(axum::middleware::from_fn(logging_middleware));

    let addr = SocketAddr::from((
        config.host.parse::<std::net::IpAddr>()
            .expect("Host inválido"),
        config.port,
    ));

    tracing::info!("   Servidor rodando em http://{}", addr);
    tracing::info!("   Endpoints disponíveis:");
    tracing::info!("   GET  /health");
    tracing::info!("   POST /cpf/validar");
    tracing::info!("   POST /api/v1/termo/criar");
    tracing::info!("   POST /api/v1/termo/autorizar");
    tracing::info!("   POST /api/v1/simulacao/gerar");
    tracing::info!("   POST /api/v1/proposta/criar");
    tracing::info!("   GET  /api/v1/operacao/{{id}}");
    tracing::info!("   SWAGGER JSON: /api-docs/openapi.json");
    tracing::info!("   SWAGGER UI: /swagger-ui");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Falha ao vincular porta");

    axum::serve(listener, app)
        .await
        .expect("Erro ao iniciar servidor");
}

async fn logging_middleware(
    req: Request<Body>,
    next: Next,
) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();

    tracing::debug!("{} {}", method, uri);

    next.run(req).await
}
