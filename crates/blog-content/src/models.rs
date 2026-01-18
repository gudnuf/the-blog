//! Data models for blog content

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Frontmatter metadata for blog posts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frontmatter {
    pub title: String,
    pub slug: String,
    pub date: NaiveDate,
    #[serde(default)]
    pub updated: Option<NaiveDate>,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default = "default_template")]
    pub template: String,
    #[serde(default)]
    pub draft: bool,
    #[serde(default)]
    pub toc: bool,
    #[serde(default)]
    pub featured_image: Option<String>,
}

fn default_template() -> String {
    "post".to_string()
}

/// A parsed blog post
#[derive(Debug, Clone, Serialize)]
pub struct Post {
    pub frontmatter: Frontmatter,
    pub raw_content: String,
    pub file_path: String,
}

impl Post {
    /// Get the post's title
    pub fn title(&self) -> &str {
        &self.frontmatter.title
    }

    /// Get the post's slug
    pub fn slug(&self) -> &str {
        &self.frontmatter.slug
    }

    /// Get the post's date
    pub fn date(&self) -> NaiveDate {
        self.frontmatter.date
    }

    /// Check if the post is a draft
    pub fn is_draft(&self) -> bool {
        self.frontmatter.draft
    }
}

/// Frontmatter for static pages (simpler than posts)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageFrontmatter {
    pub title: String,
    pub slug: String,
    #[serde(default = "default_page_template")]
    pub template: String,
}

fn default_page_template() -> String {
    "page".to_string()
}

/// A parsed static page
#[derive(Debug, Clone, Serialize)]
pub struct Page {
    pub title: String,
    pub slug: String,
    pub template: String,
    pub raw_content: String,
    pub file_path: String,
}

/// Rendered markdown content with optional table of contents
#[derive(Debug, Clone, Serialize)]
pub struct RenderedContent {
    pub html: String,
    pub toc: Option<String>,
}
