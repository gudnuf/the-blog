//! Blog content parsing library
//!
//! This crate provides functionality for parsing markdown blog posts and pages
//! with YAML frontmatter, syntax highlighting, and table of contents generation.

pub mod highlighter;
pub mod models;
pub mod parser;
pub mod toc;

pub use models::{Frontmatter, Page, Post, RenderedContent};
pub use parser::{load_all_posts, load_page, load_post, ContentError};
