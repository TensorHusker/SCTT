//! # SCTT Language Server
//!
//! Language Server Protocol implementation for SCTT.

use sctt_core::prelude::*;
use sctt_checker::TypeChecker;
use sctt_web::create_app;
use axum::Router;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    info!("Starting SCTT Language Server...");
    
    // Create web application
    let app = create_server_app().await;
    
    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn create_server_app() -> Router {
    create_app()
        .layer(CorsLayer::permissive())
        .layer(tower_http::trace::TraceLayer::new_for_http())
}