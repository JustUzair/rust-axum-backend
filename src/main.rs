mod model;
use ::tokio::net::TcpListener;
use serde_json::json;
use std::error::Error;

use axum::{Json, Router, routing::get};

use tracing::info;
use tracing_subscriber::EnvFilter;

use crate::model::ModelManager;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(true)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let model_manager: ModelManager = ModelManager::new();
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    let routes_all = Router::new().route(
        "/health",
        get(async || {
            let health = json!({
                "status":200,
                "message":"Up and Running...",
            });
            info!("GET/ health 200");
            Json(health)
        }),
    );
    info!("{:<12} - {:?}\n", "LISTENING", listener.local_addr()?);

    axum::serve(listener, routes_all.into_make_service())
        .await
        .map_err(|msg| format!("❌ Error starting server\n{msg}"))?;
    Ok(())
}
