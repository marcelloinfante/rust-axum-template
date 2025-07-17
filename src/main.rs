mod api;
mod error;
mod interactors;
mod models;
mod utils;

use axum::{Router, http::StatusCode, response::IntoResponse};
use sea_orm::{Database, DatabaseConnection};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    utils::trace::add_tracing().await;

    let db: DatabaseConnection = Database::connect("sqlite::memory:")
        .await
        .expect("Database connection failed");

    let state: AppState = AppState { db };

    let app: Router = Router::new()
        .nest("/api/v1", api::v1::routes())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

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

#[derive(Clone)]
struct AppState {
    pub db: DatabaseConnection,
}
