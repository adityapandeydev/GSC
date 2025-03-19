use axum::routing::post;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct UserInput {
    investment_type: String,
    risk_level: String,
}

#[derive(Serialize)]
struct ModelResponse {
    suggestions: Vec<String>,
}

pub fn routes() -> Router {
    Router::new().route("/advice", post(handle_request))
}

async fn handle_request(Json(_payload): Json<UserInput>) -> Json<ModelResponse> {
    let response = ModelResponse {
        suggestions: vec![
            "Diversify your portfolio".to_string(),
            "Consider long-term stability".to_string(),
        ],
    };
    Json(response)
}