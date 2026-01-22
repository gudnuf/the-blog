//! Data models for blog content

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Deserializer, Serialize};

/// All valid categories for posts - defined in one place
pub const CATEGORIES: &[(&str, &str)] = &[
    ("engineering", "Engineering"),
    ("programming", "Programming"),
    ("devops", "DevOps"),
    ("web-development", "Web Development"),
    ("reflections", "Reflections"),
    ("reference", "Reference"),
];

/// Get the display name for a category slug
pub fn category_display_name(slug: &str) -> &str {
    CATEGORIES
        .iter()
        .find(|(s, _)| *s == slug)
        .map(|(_, name)| *name)
        .unwrap_or(slug)
}

/// Custom deserializer that handles both date and datetime formats
fn deserialize_datetime<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;

    // Try datetime format first (YYYY-MM-DD HH:MM:SS or YYYY-MM-DDTHH:MM:SS)
    if let Ok(dt) = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S") {
        return Ok(dt);
    }
    if let Ok(dt) = NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S") {
        return Ok(dt);
    }

    // Fall back to date-only format, defaulting to midnight
    if let Ok(date) = NaiveDate::parse_from_str(&s, "%Y-%m-%d") {
        return Ok(date.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()));
    }

    Err(serde::de::Error::custom(format!(
        "Invalid date/datetime format: {}. Expected YYYY-MM-DD or YYYY-MM-DD HH:MM:SS",
        s
    )))
}

/// Custom serializer for datetime that outputs in readable format
fn serialize_datetime<S>(dt: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&dt.format("%Y-%m-%d %H:%M:%S").to_string())
}

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
    #[serde(deserialize_with = "deserialize_datetime", serialize_with = "serialize_datetime")]
    pub date: NaiveDateTime,
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
    pub fn date(&self) -> NaiveDateTime {
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
