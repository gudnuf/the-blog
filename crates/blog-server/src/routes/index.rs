//! Index route handler

use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::Html,
};

use crate::AppState;
use crate::routes::posts::render_post_content;
use crate::routes::{AUTHOR_CLAUDE, AUTHOR_GUDNUF};

/// Render the index page with split timeline for dual narrative
pub async fn index(
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, StatusCode> {
    // Load from cache (already filtered by draft status)
    let all_posts = state.post_cache.read();

    // Split posts by author
    let claude_posts: Vec<_> = all_posts
        .iter()
        .filter(|p| p.author().map(|a| a == AUTHOR_CLAUDE).unwrap_or(false))
        .take(5)
        .cloned()
        .collect();

    let gudnuf_posts: Vec<_> = all_posts
        .iter()
        .filter(|p| p.author().map(|a| a == AUTHOR_GUDNUF).unwrap_or(false))
        .take(5)
        .cloned()
        .collect();

    // Also include posts without authors or other authors (for backward compatibility)
    let other_posts: Vec<_> = all_posts
        .iter()
        .filter(|p| !matches!(p.author(), Some(a) if a == AUTHOR_CLAUDE || a == AUTHOR_GUDNUF))
        .take(5)
        .cloned()
        .collect();

    // For backward compatibility, if no author-specific posts exist, use all posts
    if claude_posts.is_empty() && gudnuf_posts.is_empty() {
        let posts: Vec<_> = all_posts
            .iter()
            .take(state.config.posts_per_page)
            .cloned()
            .collect();

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

        return Ok(Html(html));
    }

    // Render featured posts for each author
    let claude_featured = claude_posts.first().map(|post| {
        let rendered = render_post_content(post);
        (post.clone(), rendered)
    });

    let gudnuf_featured = gudnuf_posts.first().map(|post| {
        let rendered = render_post_content(post);
        (post.clone(), rendered)
    });

    // Collect all posts for the template (limited to posts_per_page)
    let posts: Vec<_> = all_posts
        .iter()
        .take(state.config.posts_per_page)
        .cloned()
        .collect();

    let featured_post = posts.first().cloned();

    let mut context = tera::Context::new();
    context.insert("title", "The Nousphere in Dialogue");
    context.insert("posts", &posts);
    if let Some(ref post) = featured_post {
        context.insert("featured_post", post);
    }
    context.insert("claude_posts", &claude_posts);
    context.insert("gudnuf_posts", &gudnuf_posts);
    context.insert("other_posts", &other_posts);

    if let Some((post, rendered)) = &claude_featured {
        context.insert("claude_featured", post);
        context.insert("claude_featured_content", &rendered.html);
    }

    if let Some((post, rendered)) = &gudnuf_featured {
        context.insert("gudnuf_featured", post);
        context.insert("gudnuf_featured_content", &rendered.html);
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
