use axum::{Json, Router, routing::post};

use crate::error::Result;
use crate::models::user::User;

pub fn routes() -> Router {
    Router::new().route("/user", post(get_user))
}

async fn get_user(Json(user): Json<User>) -> Result<Json<User>> {
    Ok(Json(user))
}
