//! Data models for blog content

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Relationship type between posts
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RelationType {
    Related,
    Sequel,
    Prequel,
    Conversation,
}

impl Default for RelationType {
    fn default() -> Self {
        RelationType::Related
    }
}

impl RelationType {
    pub fn label(&self) -> &str {
        match self {
            RelationType::Related => "Related Post",
            RelationType::Sequel => "Sequel",
            RelationType::Prequel => "Prequel",
            RelationType::Conversation => "In Conversation",
        }
    }
}

/// Reference to a related post
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedPost {
    pub slug: String,
    #[serde(default)]
    pub relationship: RelationType,
}

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
    #[serde(default)]
    pub related_posts: Vec<RelatedPost>,
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

    /// Get the post's author
    pub fn author(&self) -> Option<&str> {
        self.frontmatter.author.as_deref()
    }

    /// Get explicitly related posts (from frontmatter)
    pub fn related_posts(&self) -> &[RelatedPost] {
        &self.frontmatter.related_posts
    }

    /// Find related posts by tags from all posts
    pub fn similar_posts_by_tags<'a>(
        &self,
        all_posts: &'a [Post],
        limit: usize,
    ) -> Vec<&'a Post> {
        use std::collections::HashSet;

        let post_tags: HashSet<_> = self.frontmatter.tags.iter().collect();

        let mut similar: Vec<_> = all_posts
            .iter()
            .filter(|p| p.slug() != self.slug() && !p.is_draft())
            .map(|p| {
                let matching_tags = p.frontmatter.tags.iter()
                    .filter(|tag| post_tags.contains(tag))
                    .count();
                (p, matching_tags)
            })
            .filter(|(_, count)| *count > 0)
            .collect();

        similar.sort_by(|a, b| b.1.cmp(&a.1));

        similar
            .into_iter()
            .take(limit)
            .map(|(p, _)| p)
            .collect()
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
