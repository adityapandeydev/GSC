use std::collections::HashMap;
use tokio::sync::Mutex;
use std::sync::Arc;

#[derive(Default)]
pub struct AppState {
    pub users: Mutex<HashMap<String, String>>, // username -> password (mock database)
}