//! Index route handler

use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::Html,
};
use blog_content::{load_all_posts, parser::load_post_by_slug};

use crate::AppState;
use crate::routes::posts::render_post_content;

/// Render the index page with recent posts
pub async fn index(
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, StatusCode> {
    let posts = load_all_posts(&state.config.content_path)
        .map_err(|e| {
            tracing::error!("Failed to load posts: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Filter out drafts unless enabled
    let posts: Vec<_> = posts
        .into_iter()
        .filter(|p| state.config.enable_drafts || !p.is_draft())
        .take(state.config.posts_per_page)
        .collect();

    // Render the first post content for the featured section
    let featured_post = if let Some(first) = posts.first() {
        match load_post_by_slug(first.slug(), &state.config.content_path) {
            Ok(post) => {
                let rendered = render_post_content(&post);
                Some((post, rendered))
            }
            Err(_) => None,
        }
    } else {
        None
    };

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
