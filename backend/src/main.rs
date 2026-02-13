use axum::Router;
use std::sync::Arc;
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod handlers;
mod middleware;
mod repository;
mod routes;
mod services;
mod utils;

use config::{AppState, Config};
use repository::db::create_db_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing subscriber for structured logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration from environment
    dotenv::dotenv().ok();
    let config = Config::from_env()
        .map_err(|e| format!("Failed to load configuration: {}", e))?;

    tracing::info!(
        "Starting Royal Pizza Backend on {}:{}",
        config.server.host,
        config.server.port
    );

    // Create database connection
    let db = create_db_client(&config.database).await?;
    tracing::info!("Connected to SurrealDB");

    // Seed database if enabled
    let should_seed = std::env::var("DATABASE_SEED")
        .unwrap_or_else(|_| "true".to_string())
        .parse::<bool>()
        .unwrap_or(true);

    if should_seed {
        tracing::info!("Database seeding enabled");
        match repository::seed::seed_database(&db, false).await {
            Ok(_) => tracing::info!("Database initialization completed successfully"),
            Err(e) => {
                tracing::error!("Failed to seed database: {}", e);
                return Err(e);
            }
        }
    } else {
        tracing::info!("Database seeding disabled via DATABASE_SEED=false");
    }

    // Build AppState with Arc-wrapped dependencies
    let app_state = AppState::new(Arc::new(db), Arc::new(config.clone()));

    // Create Axum router with routes and middleware
    let app = Router::new()
        .nest("/api", routes::api::create_router())
        .layer(axum::middleware::from_fn(
            middleware::logging::log_request,
        ))
        .layer(TraceLayer::new_for_http())
        .layer(middleware::cors::create_cors_layer(
            &config.server.cors_allow_origin,
        ))
        .with_state(app_state);

    // Start server with graceful shutdown
    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Royal Pizza Backend listening on {}", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    tracing::info!("Server shutdown complete");
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("Shutdown signal received (Ctrl+C)");
        },
        _ = terminate => {
            tracing::info!("Shutdown signal received (SIGTERM)");
        },
    }
}
