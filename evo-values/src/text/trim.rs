use crate::definitions::text::trim as trim_definition;

pub fn trim(text: &str) -> &str {
    text.trim()
}

pub const TRIM: trim_definition::Trim = trim;
