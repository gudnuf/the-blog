//! Table of contents generation

use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};

/// A table of contents entry
#[derive(Debug, Clone)]
pub struct TocEntry {
    pub level: u8,
    pub text: String,
    pub id: String,
}

/// Extract table of contents from markdown content
pub fn extract_toc(markdown: &str) -> Vec<TocEntry> {
    let parser = Parser::new(markdown);
    let mut entries = Vec::new();
    let mut current_heading: Option<(u8, String)> = None;

    for event in parser {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                let level_num = match level {
                    HeadingLevel::H1 => 1,
                    HeadingLevel::H2 => 2,
                    HeadingLevel::H3 => 3,
                    HeadingLevel::H4 => 4,
                    HeadingLevel::H5 => 5,
                    HeadingLevel::H6 => 6,
                };
                current_heading = Some((level_num, String::new()));
            }
            Event::Text(text) => {
                if let Some((_, ref mut heading_text)) = current_heading {
                    heading_text.push_str(&text);
                }
            }
            Event::Code(code) => {
                if let Some((_, ref mut heading_text)) = current_heading {
                    heading_text.push_str(&code);
                }
            }
            Event::End(TagEnd::Heading(_)) => {
                if let Some((level, text)) = current_heading.take() {
                    let id = slugify(&text);
                    entries.push(TocEntry { level, text, id });
                }
            }
            _ => {}
        }
    }

    entries
}

/// Generate HTML for table of contents
pub fn render_toc(entries: &[TocEntry]) -> String {
    if entries.is_empty() {
        return String::new();
    }

    let mut html = String::from("<nav class=\"toc\" aria-label=\"Table of contents\">\n");
    html.push_str("<h2 class=\"toc-title\">Contents</h2>\n");
    html.push_str("<ul class=\"toc-list\">\n");

    for entry in entries {
        // Only include h2 and h3 in TOC
        if entry.level >= 2 && entry.level <= 3 {
            let indent = if entry.level == 3 { "  " } else { "" };
            html.push_str(&format!(
                "{}<li class=\"toc-item toc-level-{}\"><a href=\"#{}\">{}</a></li>\n",
                indent,
                entry.level,
                entry.id,
                html_escape::encode_text(&entry.text)
            ));
        }
    }

    html.push_str("</ul>\n");
    html.push_str("</nav>\n");

    html
}

/// Convert text to a URL-safe slug
fn slugify(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c
            } else if c.is_whitespace() || c == '-' || c == '_' {
                '-'
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_toc() {
        let markdown = r#"
# Main Title
## Introduction
Some text here.
## Getting Started
### Prerequisites
More text.
### Installation
## Conclusion
"#;
        let toc = extract_toc(markdown);

        assert_eq!(toc.len(), 6);
        assert_eq!(toc[0].text, "Main Title");
        assert_eq!(toc[0].level, 1);
        assert_eq!(toc[1].text, "Introduction");
        assert_eq!(toc[1].level, 2);
        assert_eq!(toc[3].text, "Prerequisites");
        assert_eq!(toc[3].level, 3);
    }

    #[test]
    fn test_slugify() {
        assert_eq!(slugify("Hello World"), "hello-world");
        assert_eq!(slugify("Getting Started with Rust"), "getting-started-with-rust");
        assert_eq!(slugify("API v2.0"), "api-v2-0");
    }

    #[test]
    fn test_render_toc() {
        let entries = vec![
            TocEntry {
                level: 2,
                text: "Introduction".to_string(),
                id: "introduction".to_string(),
            },
            TocEntry {
                level: 3,
                text: "Background".to_string(),
                id: "background".to_string(),
            },
        ];

        let html = render_toc(&entries);
        assert!(html.contains("Introduction"));
        assert!(html.contains("href=\"#introduction\""));
        assert!(html.contains("toc-level-2"));
        assert!(html.contains("toc-level-3"));
    }
}
