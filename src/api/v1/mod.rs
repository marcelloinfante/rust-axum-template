use axum::{Json, Router, routing::post};

use crate::AppState;

use crate::error::Result;
use crate::models::user::User;

pub fn routes() -> Router<AppState> {
    Router::new().route("/user", post(get_user))
}

async fn get_user(Json(user): Json<User>) -> Result<Json<User>> {
    let new_user = User {
        first_name: String::from("Test"),
        last_name: String::from("Test2"),
    };
    Ok(Json(new_user))
}
