//! Tera template management

use std::path::Path;
use std::sync::RwLock;

use chrono::Datelike;
use tera::Tera;

/// Template manager wrapping Tera
pub struct Templates {
    tera: RwLock<Tera>,
}

impl Templates {
    /// Create a new template manager loading templates from the given path
    pub fn new(templates_path: &Path) -> anyhow::Result<Self> {
        let pattern = templates_path.join("**/*.html");
        let pattern_str = pattern.to_string_lossy();

        let mut tera = Tera::new(&pattern_str)?;

        // Register custom filters
        tera.register_filter("date_format", date_format_filter);

        // Register custom functions
        tera.register_function("current_year", current_year_function);

        Ok(Self {
            tera: RwLock::new(tera),
        })
    }

    /// Render a template with the given context
    pub fn render(&self, template: &str, context: &tera::Context) -> anyhow::Result<String> {
        let tera = self.tera.read().unwrap();
        Ok(tera.render(template, context)?)
    }
}

/// Custom filter for formatting dates
fn date_format_filter(
    value: &tera::Value,
    args: &std::collections::HashMap<String, tera::Value>,
) -> tera::Result<tera::Value> {
    let date_str = value
        .as_str()
        .ok_or_else(|| tera::Error::msg("date_format filter expects a string"))?;

    let format = args
        .get("format")
        .and_then(|v| v.as_str())
        .unwrap_or("%B %d, %Y");

    // Parse the date and format it
    if let Ok(date) = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        Ok(tera::Value::String(date.format(format).to_string()))
    } else {
        // Return original if parsing fails
        Ok(value.clone())
    }
}

/// Custom function to get the current year
fn current_year_function(
    _args: &std::collections::HashMap<String, tera::Value>,
) -> tera::Result<tera::Value> {
    let year = chrono::Local::now().year();
    Ok(tera::Value::Number(year.into()))
}
