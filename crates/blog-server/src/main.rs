//! Blog server - SSR blog with Axum

mod config;
mod routes;
mod templates;

use std::net::SocketAddr;
use std::path::Path;
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
use blog_content::Post;
use parking_lot::RwLock;

/// Application state shared across handlers
pub struct AppState {
    pub config: Config,
    pub templates: Templates,
    pub post_cache: Arc<RwLock<Vec<Post>>>,
}

/// Load all posts into memory cache
fn load_posts_into_cache(
    content_path: &Path,
    enable_drafts: bool,
) -> Result<Vec<Post>, blog_content::ContentError> {
    let all_posts = blog_content::load_all_posts(content_path)?;

    // Pre-filter drafts during cache load
    let posts: Vec<_> = all_posts
        .into_iter()
        .filter(|p| enable_drafts || !p.is_draft())
        .collect();

    tracing::info!("Loaded {} posts into cache", posts.len());
    Ok(posts)
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

    // Initialize post cache
    let initial_posts = load_posts_into_cache(&config.content_path, config.enable_drafts)?;
    let post_cache = Arc::new(RwLock::new(initial_posts));

    // Create shared state
    let state = Arc::new(AppState {
        config: config.clone(),
        templates,
        post_cache,
    });

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
        .with_state(state.clone());

    // Spawn SIGHUP handler for cache reload
    spawn_sighup_handler(state);

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

/// Spawn a task to handle SIGHUP signals for cache reload
fn spawn_sighup_handler(state: Arc<AppState>) {
    #[cfg(unix)]
    {
        tokio::spawn(async move {
            use tokio::signal::unix::{signal, SignalKind};

            let mut sighup = match signal(SignalKind::hangup()) {
                Ok(s) => s,
                Err(e) => {
                    tracing::error!("Failed to install SIGHUP handler: {}", e);
                    return;
                }
            };

            tracing::info!("SIGHUP handler installed");

            loop {
                sighup.recv().await;
                tracing::info!("SIGHUP received, reloading post cache");

                match load_posts_into_cache(&state.config.content_path, state.config.enable_drafts) {
                    Ok(new_posts) => {
                        *state.post_cache.write() = new_posts;
                        tracing::info!("Post cache reloaded successfully");
                    }
                    Err(e) => {
                        tracing::error!("Failed to reload post cache: {}", e);
                        // Keep old cache on error
                    }
                }
            }
        });
    }

    #[cfg(not(unix))]
    tracing::warn!("SIGHUP handler not available on non-Unix systems");
}
