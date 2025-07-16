mod api;
mod interactors;
mod models;

use axum::Router;

#[tokio::main]
async fn main() {
    let app: Router = Router::new().nest("/api/v1", api::v1::routes());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
