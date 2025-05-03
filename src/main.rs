/* Main application entry point */

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
// FIXME: The code above should only be used in development.

use gandalf::{
    app,
    config::{database, get_config, telemetry},
};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();
    // Initialize tracing
    telemetry::init_tracing();

    let config = get_config();
    let db_connection_pool = database::get_db_connection_pool().await;
    let listener = TcpListener::bind(format!("0.0.0.0:{}", config.app_port))
        .await
        .expect("Failed to bind to address");
    let app = app::build_app(Arc::new(db_connection_pool.clone()));

    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
    Ok(())
}
