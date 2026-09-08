use crate::definitions::text::ends_with as ends_with_definition;

pub fn ends_with(text: &str, suffix: &str) -> bool {
    text.ends_with(suffix)
}

pub const ENDS_WITH: ends_with_definition::EndsWith = ends_with;
