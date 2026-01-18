//! Post route handlers

use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::{StatusCode, HeaderMap},
    response::Html,
};
use blog_content::{
    load_all_posts,
    parser::load_post_by_slug,
    Post, RenderedContent,
    highlighter::highlight_code,
    toc::{extract_toc, render_toc},
};
use pulldown_cmark::{CodeBlockKind, CowStr, Event, Options, Parser, Tag, TagEnd};
use serde::Deserialize;

use crate::AppState;

#[derive(Deserialize)]
pub struct ListQuery {
    pub page: Option<usize>,
}

/// List posts with pagination
pub async fn list(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ListQuery>,
) -> Result<Html<String>, StatusCode> {
    let page = query.page.unwrap_or(1).max(1);
    render_post_list(state, page, false).await
}

/// List posts for a specific page (HTMX partial)
pub async fn list_page(
    State(state): State<Arc<AppState>>,
    Path(page): Path<usize>,
    headers: HeaderMap,
) -> Result<Html<String>, StatusCode> {
    let is_htmx = headers.contains_key("hx-request");
    render_post_list(state, page.max(1), is_htmx).await
}

async fn render_post_list(
    state: Arc<AppState>,
    page: usize,
    partial: bool,
) -> Result<Html<String>, StatusCode> {
    let all_posts = load_all_posts(&state.config.content_path)
        .map_err(|e| {
            tracing::error!("Failed to load posts: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Filter drafts
    let all_posts: Vec<_> = all_posts
        .into_iter()
        .filter(|p| state.config.enable_drafts || !p.is_draft())
        .collect();

    let per_page = state.config.posts_per_page;
    let total_pages = (all_posts.len() + per_page - 1) / per_page;
    let skip = (page - 1) * per_page;

    let posts: Vec<_> = all_posts
        .into_iter()
        .skip(skip)
        .take(per_page)
        .collect();

    let mut context = tera::Context::new();
    context.insert("posts", &posts);
    context.insert("page", &page);
    context.insert("total_pages", &total_pages);
    context.insert("has_next", &(page < total_pages));
    context.insert("has_prev", &(page > 1));
    context.insert("next_page", &(page + 1));
    context.insert("prev_page", &(page - 1));
    context.insert("title", "All Posts");

    let template = if partial {
        "partials/post_list_items.html"
    } else {
        "post_list.html"
    };

    let html = state
        .templates
        .render(template, &context)
        .map_err(|e| {
            tracing::error!("Failed to render template: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Html(html))
}

/// Show a single post
pub async fn show(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<Html<String>, StatusCode> {
    // Validate slug to prevent path traversal
    if slug.contains("..") || slug.contains('/') || slug.contains('\\') {
        return Err(StatusCode::BAD_REQUEST);
    }

    let post = load_post_by_slug(&slug, &state.config.content_path)
        .map_err(|e| {
            tracing::warn!("Post not found: {} - {}", slug, e);
            StatusCode::NOT_FOUND
        })?;

    // Check draft status
    if post.is_draft() && !state.config.enable_drafts {
        return Err(StatusCode::NOT_FOUND);
    }

    let rendered = render_post_content(&post);

    let mut context = tera::Context::new();
    context.insert("post", &post);
    context.insert("content", &rendered.html);
    context.insert("title", post.title());

    if let Some(ref toc_html) = rendered.toc {
        context.insert("toc", toc_html);
        context.insert("has_toc", &true);
    } else {
        context.insert("has_toc", &false);
    }

    let html = state
        .templates
        .render("post.html", &context)
        .map_err(|e| {
            tracing::error!("Failed to render template: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Html(html))
}

/// Render markdown content with syntax highlighting and optional TOC
pub fn render_post_content(post: &Post) -> RenderedContent {
    let content = &post.raw_content;

    // Extract TOC if enabled
    let toc = if post.frontmatter.toc {
        let entries = extract_toc(content);
        Some(render_toc(&entries))
    } else {
        None
    };

    // Parse and render markdown with syntax highlighting
    let html = render_markdown_with_highlighting(content);

    RenderedContent { html, toc }
}

fn render_markdown_with_highlighting(content: &str) -> String {
    let options = Options::all();
    let parser = Parser::new_ext(content, options);

    let mut in_code_block = false;
    let mut code_lang = String::new();
    let mut code_content = String::new();
    let mut heading_id = String::new();

    let events: Vec<Event> = parser
        .flat_map(|event| {
            match event {
                Event::Start(Tag::CodeBlock(kind)) => {
                    in_code_block = true;
                    code_content.clear();
                    code_lang = match kind {
                        CodeBlockKind::Fenced(lang) => lang.to_string(),
                        CodeBlockKind::Indented => String::new(),
                    };
                    vec![]
                }
                Event::End(TagEnd::CodeBlock) => {
                    in_code_block = false;
                    let highlighted = highlight_code(&code_content, &code_lang);
                    vec![Event::Html(CowStr::from(highlighted))]
                }
                Event::Text(text) if in_code_block => {
                    code_content.push_str(&text);
                    vec![]
                }
                Event::Start(Tag::Heading { level, id, .. }) => {
                    // Generate heading ID for anchor links
                    heading_id = id.map(|s: CowStr| s.to_string()).unwrap_or_default();
                    vec![Event::Start(Tag::Heading { level, id: None, classes: vec![], attrs: vec![] })]
                }
                Event::End(TagEnd::Heading(level)) => {
                    if heading_id.is_empty() {
                        vec![Event::End(TagEnd::Heading(level))]
                    } else {
                        // Add ID to heading for anchor links
                        let id = std::mem::take(&mut heading_id);
                        vec![
                            Event::Html(CowStr::from(format!("<a id=\"{}\"></a>", id))),
                            Event::End(TagEnd::Heading(level)),
                        ]
                    }
                }
                _ => vec![event],
            }
        })
        .collect();

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, events.into_iter());

    html_output
}
