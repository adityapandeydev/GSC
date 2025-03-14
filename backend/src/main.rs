use axum::{
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/investment-advice", post(handle_request));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Backend running on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize)]
struct UserInput {
    investment_type: String,
    risk_level: String,
}

#[derive(Serialize)]
struct ModelResponse {
    suggestions: Vec<String>,
}

async fn handle_request(Json(payload): Json<UserInput>) -> Json<ModelResponse> {
    // Here, we would call the Python model using HTTP (to be implemented)
    let response = ModelResponse {
        suggestions: vec![
            "Diversify your portfolio".to_string(),
            "Consider long-term stability".to_string(),
        ],
    };

    Json(response)
}
