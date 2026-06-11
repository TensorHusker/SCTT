//! # Web Interface for SCTT
//!
//! Stub implementation for web interface.

use axum::{Router, response::Json};
use serde_json::{json, Value};

/// Create web app router
pub fn create_app() -> Router {
    Router::new()
}

/// Health check endpoint
pub async fn health() -> Json<Value> {
    Json(json!({"status": "ok", "service": "sctt-web"}))
}