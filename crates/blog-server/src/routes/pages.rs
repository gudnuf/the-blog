//! Page route handlers

use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Html,
};
use blog_content::load_page;
use pulldown_cmark::{Options, Parser};

use crate::AppState;

/// Show a static page
pub async fn show(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<Html<String>, StatusCode> {
    // Validate slug to prevent path traversal
    if slug.contains("..") || slug.contains('/') || slug.contains('\\') {
        return Err(StatusCode::BAD_REQUEST);
    }

    let page = load_page(&slug, &state.config.content_path)
        .map_err(|e| {
            tracing::warn!("Page not found: {} - {}", slug, e);
            StatusCode::NOT_FOUND
        })?;

    // Render markdown
    let options = Options::all();
    let parser = Parser::new_ext(&page.raw_content, options);
    let mut html_content = String::new();
    pulldown_cmark::html::push_html(&mut html_content, parser);

    let mut context = tera::Context::new();
    context.insert("page", &page);
    context.insert("content", &html_content);
    context.insert("title", &page.title);

    let html = state
        .templates
        .render("page.html", &context)
        .map_err(|e| {
            tracing::error!("Failed to render template: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Html(html))
}
