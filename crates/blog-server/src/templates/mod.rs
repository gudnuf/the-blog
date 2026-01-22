//! Tera template management

use std::path::Path;
use std::sync::RwLock;

use chrono::{Datelike, Local, NaiveDateTime};
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
        tera.register_filter("relative_time", relative_time_filter);

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

/// Custom filter for formatting dates (handles both date and datetime)
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

    // Try datetime formats first, then date-only
    if let Ok(dt) = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S") {
        Ok(tera::Value::String(dt.format(format).to_string()))
    } else if let Ok(dt) = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S") {
        Ok(tera::Value::String(dt.format(format).to_string()))
    } else if let Ok(date) = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
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

/// Custom filter for displaying relative time (e.g., "2 hours ago", "yesterday")
fn relative_time_filter(
    value: &tera::Value,
    _args: &std::collections::HashMap<String, tera::Value>,
) -> tera::Result<tera::Value> {
    let datetime_str = value
        .as_str()
        .ok_or_else(|| tera::Error::msg("relative_time filter expects a string"))?;

    // Try to parse as datetime first, then as date
    let datetime = if let Ok(dt) = NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S") {
        dt
    } else if let Ok(dt) = NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%dT%H:%M:%S") {
        dt
    } else if let Ok(date) = chrono::NaiveDate::parse_from_str(datetime_str, "%Y-%m-%d") {
        date.and_hms_opt(0, 0, 0).unwrap()
    } else {
        return Ok(value.clone());
    };

    let now = Local::now().naive_local();
    let duration = now.signed_duration_since(datetime);
    let seconds = duration.num_seconds();

    let result = if seconds < 0 {
        // Future date
        datetime.format("%B %d, %Y").to_string()
    } else if seconds < 60 {
        "just now".to_string()
    } else if seconds < 3600 {
        let mins = seconds / 60;
        if mins == 1 {
            "1 minute ago".to_string()
        } else {
            format!("{} minutes ago", mins)
        }
    } else if seconds < 86400 {
        let hours = seconds / 3600;
        if hours == 1 {
            "1 hour ago".to_string()
        } else {
            format!("{} hours ago", hours)
        }
    } else if seconds < 172800 {
        // Less than 2 days
        format!("yesterday at {}", datetime.format("%H:%M"))
    } else if seconds < 604800 {
        // Less than 7 days - show day name
        let days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
        let weekday = datetime.weekday().num_days_from_sunday() as usize;
        days[weekday].to_string()
    } else {
        // Older than a week - show full date
        datetime.format("%B %d, %Y").to_string()
    };

    Ok(tera::Value::String(result))
}
