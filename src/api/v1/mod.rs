use axum::{Router, routing::get};

pub fn routes() -> Router {
    Router::new().route("/hello", get(|| async { "Hello, World!" }))
}
