use axum::{Json, Router, routing::post};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration};
use std::collections::HashMap;
use utils::AppState;
use axum::extract::State;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Deserialize)]
struct LoginInput {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct TokenResponse {
    token: String,
}

pub fn routes() -> Router {
    Router::new().route("/login", post(login))
}

async fn login(State(state): State<Arc<Mutex<AppState>>>, Json(payload): Json<LoginInput>) -> Json<TokenResponse> {
    let token = encode(
        &Header::default(),
        &HashMap::from([("sub", payload.username)]),
        &EncodingKey::from_secret("secret".as_ref()),
    ).unwrap();
    Json(TokenResponse { token })
}