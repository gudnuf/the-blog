//! Content parsing functionality

use std::fs;
use std::path::Path;

use gray_matter::{engine::YAML, Matter};
use thiserror::Error;
use walkdir::WalkDir;

use crate::models::{Frontmatter, Page, PageFrontmatter, Post};

/// Errors that can occur during content parsing
#[derive(Error, Debug)]
pub enum ContentError {
    #[error("Failed to read file: {0}")]
    FileRead(#[from] std::io::Error),

    #[error("Failed to parse frontmatter: {0}")]
    FrontmatterParse(String),

    #[error("Missing required frontmatter field: {0}")]
    MissingField(String),

    #[error("Post not found: {0}")]
    PostNotFound(String),

    #[error("Page not found: {0}")]
    PageNotFound(String),

    #[error("Invalid content path: {0}")]
    InvalidPath(String),
}

/// Parse a single post from a file path
pub fn load_post(path: &Path) -> Result<Post, ContentError> {
    let content = fs::read_to_string(path)?;
    let matter = Matter::<YAML>::new();

    let parsed = matter.parse(&content);

    let frontmatter: Frontmatter = parsed
        .data
        .ok_or_else(|| ContentError::MissingField("frontmatter".to_string()))?
        .deserialize()
        .map_err(|e| ContentError::FrontmatterParse(e.to_string()))?;

    Ok(Post {
        frontmatter,
        raw_content: parsed.content,
        file_path: path.to_string_lossy().to_string(),
    })
}

/// Load all posts from a content directory
///
/// Posts are expected to be in `content_dir/posts/` with filenames like
/// `YYYY-MM-DD-slug.md`
pub fn load_all_posts(content_dir: &Path) -> Result<Vec<Post>, ContentError> {
    let posts_dir = content_dir.join("posts");

    if !posts_dir.exists() {
        return Ok(Vec::new());
    }

    let mut posts = Vec::new();

    for entry in WalkDir::new(&posts_dir)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        if path.extension().map_or(false, |ext| ext == "md") {
            match load_post(path) {
                Ok(post) => posts.push(post),
                Err(e) => {
                    tracing::warn!("Failed to parse post {:?}: {}", path, e);
                }
            }
        }
    }

    // Sort by date, newest first
    posts.sort_by(|a, b| b.date().cmp(&a.date()));

    Ok(posts)
}

/// Load a specific post by slug
pub fn load_post_by_slug(slug: &str, content_dir: &Path) -> Result<Post, ContentError> {
    let posts = load_all_posts(content_dir)?;

    posts
        .into_iter()
        .find(|p| p.slug() == slug)
        .ok_or_else(|| ContentError::PostNotFound(slug.to_string()))
}

/// Load a static page by slug
pub fn load_page(slug: &str, content_dir: &Path) -> Result<Page, ContentError> {
    // Validate slug to prevent path traversal
    if slug.contains("..") || slug.contains('/') || slug.contains('\\') {
        return Err(ContentError::InvalidPath(slug.to_string()));
    }

    let pages_dir = content_dir.join("pages");
    let page_path = pages_dir.join(format!("{}.md", slug));

    if !page_path.exists() {
        return Err(ContentError::PageNotFound(slug.to_string()));
    }

    let content = fs::read_to_string(&page_path)?;
    let matter = Matter::<YAML>::new();
    let parsed = matter.parse(&content);

    let frontmatter: PageFrontmatter = parsed
        .data
        .ok_or_else(|| ContentError::MissingField("frontmatter".to_string()))?
        .deserialize()
        .map_err(|e| ContentError::FrontmatterParse(e.to_string()))?;

    Ok(Page {
        title: frontmatter.title,
        slug: frontmatter.slug,
        template: frontmatter.template,
        raw_content: parsed.content,
        file_path: page_path.to_string_lossy().to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_post(dir: &Path, filename: &str, content: &str) {
        let posts_dir = dir.join("posts");
        fs::create_dir_all(&posts_dir).unwrap();
        fs::write(posts_dir.join(filename), content).unwrap();
    }

    #[test]
    fn test_parse_post_with_valid_frontmatter() {
        let temp_dir = TempDir::new().unwrap();
        let content = r#"---
title: "Test Post"
slug: "test-post"
date: 2025-01-15
author: "Test Author"
tags: ["rust", "test"]
---

# Hello World

This is a test post.
"#;
        create_test_post(temp_dir.path(), "2025-01-15-test-post.md", content);

        let posts = load_all_posts(temp_dir.path()).unwrap();
        assert_eq!(posts.len(), 1);

        let post = &posts[0];
        assert_eq!(post.title(), "Test Post");
        assert_eq!(post.slug(), "test-post");
        assert_eq!(post.frontmatter.author, Some("Test Author".to_string()));
        assert_eq!(post.frontmatter.tags, vec!["rust", "test"]);
    }

    #[test]
    fn test_load_page() {
        let temp_dir = TempDir::new().unwrap();
        let pages_dir = temp_dir.path().join("pages");
        fs::create_dir_all(&pages_dir).unwrap();

        let content = r#"---
title: "About"
slug: "about"
---

# About this blog

Some content here.
"#;
        fs::write(pages_dir.join("about.md"), content).unwrap();

        let page = load_page("about", temp_dir.path()).unwrap();
        assert_eq!(page.title, "About");
        assert_eq!(page.slug, "about");
    }

    #[test]
    fn test_path_traversal_protection() {
        let temp_dir = TempDir::new().unwrap();

        let result = load_page("../etc/passwd", temp_dir.path());
        assert!(matches!(result, Err(ContentError::InvalidPath(_))));
    }
}
