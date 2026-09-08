use crate::definitions::text::contains as contains_definition;

pub fn contains(text: &str, pattern: &str) -> bool {
    text.contains(pattern)
}

pub const CONTAINS: contains_definition::Contains = contains;
