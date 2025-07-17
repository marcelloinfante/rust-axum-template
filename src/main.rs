mod api;
mod error;
mod interactors;
mod models;
mod utils;

use axum::{Router, http::StatusCode, response::IntoResponse};

#[tokio::main]
async fn main() {
    utils::trace::add_tracing().await;

    let app: Router = Router::new().nest("/api/v1", api::v1::routes());

    let app = app.fallback(handler_404);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(utils::shutdown::shutdown_signal())
        .await
        .unwrap();
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not Found")
}
