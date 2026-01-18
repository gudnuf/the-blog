//! Blog server - SSR blog with Axum

mod config;
mod routes;
mod templates;

use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    routing::get,
    Router,
};
use tower_http::{
    compression::CompressionLayer,
    services::ServeDir,
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::Config;
use crate::templates::Templates;

/// Application state shared across handlers
pub struct AppState {
    pub config: Config,
    pub templates: Templates,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,blog_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::load()?;
    tracing::info!("Configuration loaded: {:?}", config);

    // Initialize templates
    let templates = Templates::new(&config.templates_path)?;
    tracing::info!("Templates loaded from {:?}", config.templates_path);

    // Create shared state
    let state = Arc::new(AppState { config: config.clone(), templates });

    // Build router
    let app = Router::new()
        .route("/", get(routes::index::index))
        .route("/health", get(routes::health))
        .route("/posts", get(routes::posts::list))
        .route("/posts/page/:page", get(routes::posts::list_page))
        .route("/posts/:slug", get(routes::posts::show))
        .route("/pages/:slug", get(routes::pages::show))
        .nest_service("/static", ServeDir::new(&config.static_path))
        .nest_service("/images", ServeDir::new(config.content_path.join("images")))
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // Start server
    let addr = SocketAddr::new(config.host.parse()?, config.port);
    tracing::info!("Starting server on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    tracing::info!("Server shut down gracefully");
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("Shutdown signal received");
}
