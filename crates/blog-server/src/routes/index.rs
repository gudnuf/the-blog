//! Index route handler

use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::Html,
};

use crate::AppState;
use crate::routes::posts::render_post_content;

/// Render the index page with recent posts
pub async fn index(
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, StatusCode> {
    // Load from cache (already filtered by draft status)
    let all_posts = state.post_cache.read();
    let posts: Vec<_> = all_posts
        .iter()
        .take(state.config.posts_per_page)
        .cloned()
        .collect();

    // Render the first post content for the featured section
    // No need to reload - we already have the post!
    let featured_post = posts.first().map(|post| {
        let rendered = render_post_content(post);
        (post.clone(), rendered)
    });

    let mut context = tera::Context::new();
    context.insert("posts", &posts);
    context.insert("title", "Home");

    if let Some((post, rendered)) = &featured_post {
        context.insert("featured_post", post);
        context.insert("featured_content", &rendered.html);
    }

    let html = state
        .templates
        .render("index.html", &context)
        .map_err(|e| {
            tracing::error!("Failed to render template: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Html(html))
}
