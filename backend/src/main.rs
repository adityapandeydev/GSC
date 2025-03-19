use axum::{
    extract::State,
    middleware,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;
use tower_http::trace::TraceLayer;
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration};
use std::collections::HashMap;

mod middleware_layer;
mod routes;
mod utils;

use routes::{investment, auth};
use utils::AppState;

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(AppState::default()));
    let app = Router::new()
        .nest("/investment", investment::routes())
        .nest("/auth", auth::routes())
        .layer(middleware::from_fn(middleware_layer::log_request))
        .layer(TraceLayer::new_for_http())
        .with_state(state.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Backend running on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}