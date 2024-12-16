use crate::config::Config;
use axum::{
    routing::{get, post},
    Router,
};

pub async fn start_server(config: Config) {
    let app = Router::new()
        .route("/upload", post(upload_handler))
        .route("/search", get(search_handler));

    let listener = tokio::net::TcpListener::bind(config.listen_address)
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn upload_handler() {}

async fn search_handler() -> &'static str {
    "This is search handler"
}
