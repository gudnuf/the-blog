//! Configuration management with environment variable support

use std::env;
use std::path::PathBuf;

/// Blog server configuration
#[derive(Debug, Clone)]
pub struct Config {
    /// Host to bind to
    pub host: String,
    /// Port to listen on
    pub port: u16,
    /// Path to content directory
    pub content_path: PathBuf,
    /// Path to templates directory
    pub templates_path: PathBuf,
    /// Path to static assets directory
    pub static_path: PathBuf,
    /// Number of posts per page
    pub posts_per_page: usize,
    /// Whether to show draft posts
    pub enable_drafts: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 3311,
            content_path: PathBuf::from("./content"),
            templates_path: PathBuf::from("./templates"),
            static_path: PathBuf::from("./static"),
            posts_per_page: 10,
            enable_drafts: false,
        }
    }
}

impl Config {
    /// Load configuration from environment variables with defaults
    pub fn load() -> anyhow::Result<Self> {
        let mut config = Self::default();

        if let Ok(host) = env::var("BLOG_HOST") {
            config.host = host;
        }

        if let Ok(port) = env::var("BLOG_PORT") {
            config.port = port.parse()?;
        }

        if let Ok(path) = env::var("BLOG_CONTENT_PATH") {
            config.content_path = PathBuf::from(path);
        }

        if let Ok(path) = env::var("BLOG_TEMPLATES_PATH") {
            config.templates_path = PathBuf::from(path);
        }

        if let Ok(path) = env::var("BLOG_STATIC_PATH") {
            config.static_path = PathBuf::from(path);
        }

        if let Ok(count) = env::var("BLOG_POSTS_PER_PAGE") {
            config.posts_per_page = count.parse()?;
        }

        if let Ok(enable) = env::var("BLOG_ENABLE_DRAFTS") {
            config.enable_drafts = enable.parse().unwrap_or(false);
        }

        // Validate paths exist
        config.validate()?;

        Ok(config)
    }

    /// Validate that required paths exist
    fn validate(&self) -> anyhow::Result<()> {
        if !self.content_path.exists() {
            tracing::warn!(
                "Content path {:?} does not exist, creating...",
                self.content_path
            );
            std::fs::create_dir_all(&self.content_path)?;
            std::fs::create_dir_all(self.content_path.join("posts"))?;
            std::fs::create_dir_all(self.content_path.join("pages"))?;
        }

        if !self.templates_path.exists() {
            anyhow::bail!(
                "Templates path {:?} does not exist",
                self.templates_path
            );
        }

        if !self.static_path.exists() {
            tracing::warn!(
                "Static path {:?} does not exist, creating...",
                self.static_path
            );
            std::fs::create_dir_all(&self.static_path)?;
        }

        Ok(())
    }
}
