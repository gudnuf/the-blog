//! Syntax highlighting using syntect

use once_cell::sync::Lazy;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

/// Global syntax set for code highlighting
static SYNTAX_SET: Lazy<SyntaxSet> = Lazy::new(SyntaxSet::load_defaults_newlines);

/// Global theme set for code highlighting
static THEME_SET: Lazy<ThemeSet> = Lazy::new(ThemeSet::load_defaults);

/// Default theme for syntax highlighting
const DEFAULT_THEME: &str = "base16-ocean.dark";

/// Highlight a code block with the given language
///
/// Returns HTML with inline styles for syntax highlighting.
/// Falls back to plain text if the language is not recognized.
pub fn highlight_code(code: &str, language: &str) -> String {
    let syntax = SYNTAX_SET
        .find_syntax_by_token(language)
        .or_else(|| SYNTAX_SET.find_syntax_by_extension(language))
        .unwrap_or_else(|| SYNTAX_SET.find_syntax_plain_text());

    let theme = THEME_SET
        .themes
        .get(DEFAULT_THEME)
        .expect("Default theme should exist");

    match highlighted_html_for_string(code, &SYNTAX_SET, syntax, theme) {
        Ok(html) => html,
        Err(_) => {
            // Fallback to escaped plain text
            format!(
                "<pre><code>{}</code></pre>",
                html_escape::encode_text(code)
            )
        }
    }
}

/// Get list of supported language names
pub fn supported_languages() -> Vec<&'static str> {
    SYNTAX_SET
        .syntaxes()
        .iter()
        .map(|s| s.name.as_str())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highlight_rust_code() {
        let code = r#"fn main() {
    println!("Hello, world!");
}"#;
        let html = highlight_code(code, "rust");

        // Should contain styled spans
        assert!(html.contains("<span"));
        assert!(html.contains("style="));
        // Should contain the code
        assert!(html.contains("main"));
        assert!(html.contains("println"));
    }

    #[test]
    fn test_unknown_language_fallback() {
        let code = "some unknown code";
        let html = highlight_code(code, "nonexistent_language_xyz");

        // Should still produce valid output
        assert!(html.contains("some unknown code"));
    }
}
