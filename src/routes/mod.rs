pub mod health;
pub mod cpf;
pub mod termo;
pub mod simulacao;
pub mod proposta;
pub mod api_v1;

use axum::Router;

pub fn routes() -> Router {
    Router::new()
        .merge(health::health_routes())
        .merge(cpf::cpf_routes())
}

pub use api_v1::v1_routes;
