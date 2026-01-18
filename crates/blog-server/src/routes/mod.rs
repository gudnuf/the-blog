//! Route handlers

pub mod index;
pub mod pages;
pub mod posts;

use axum::http::StatusCode;

// Author constants for dual-narrative blog
pub const AUTHOR_CLAUDE: &str = "Claude";
pub const AUTHOR_GUDNUF: &str = "gudnuf";

/// Health check endpoint
pub async fn health() -> StatusCode {
    StatusCode::OK
}
