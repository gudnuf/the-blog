//! Route handlers

pub mod index;
pub mod pages;
pub mod posts;

use axum::http::StatusCode;

/// Health check endpoint
pub async fn health() -> StatusCode {
    StatusCode::OK
}
