use crate::definitions::text::starts_with as starts_with_definition;

pub fn starts_with(text: &str, prefix: &str) -> bool {
    text.starts_with(prefix)
}

pub const STARTS_WITH: starts_with_definition::StartsWith = starts_with;
