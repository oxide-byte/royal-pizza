use chrono::{DateTime, Utc};

/// Format currency amount as dollar string
pub fn format_currency(amount: f64) -> String {
    format!("${:.2}", amount)
}

/// Format datetime in a readable format
pub fn format_datetime(dt: &DateTime<Utc>) -> String {
    dt.format("%b %d, %Y at %I:%M %p").to_string()
}

/// Format date for input field (ISO format)
pub fn format_date(dt: &DateTime<Utc>) -> String {
    dt.format("%Y-%m-%d").to_string()
}

/// Format time for input field (HH:MM format)
pub fn format_time(dt: &DateTime<Utc>) -> String {
    dt.format("%H:%M").to_string()
}
