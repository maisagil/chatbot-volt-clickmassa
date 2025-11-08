pub mod health;
pub mod cpf;
pub mod termo;
pub mod simulacao;
pub mod proposta;
pub mod api_v1;
pub mod pix;

use axum::Router;

pub fn routes() -> Router {
    Router::new()
        .merge(health::health_routes())
}

pub use api_v1::v1_routes;
