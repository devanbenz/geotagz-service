pub mod app;
pub mod interfaces;

use app::routes::healthcheck::healthcheck;
use axum::{Router, routing::get};

pub async fn run() {
    let app = Router::new()
        .route("/", get(healthcheck));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}