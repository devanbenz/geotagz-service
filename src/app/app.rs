use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use super::{
    database::{blob_store::BlobStore, data_store::DataStore},
    queue::message_queue::Queue,
    routes::{create_image::create_image_handler, healthcheck::healthcheck},
    state::AppState,
};

pub async fn run() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    dotenv::dotenv().ok();

    let queue = Queue::new();
    let data_store = DataStore::new();
    let blob_store = BlobStore::new().await;
    let shared_state = Arc::new(AppState {
        message_queue: queue,
        blob_store: blob_store,
        data_store: data_store,
    });

    let app = Router::new()
        .route("/", get(healthcheck))
        .route("/images", post(create_image_handler))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
