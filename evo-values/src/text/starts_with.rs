use crate::definitions::text::starts_with as starts_with_definition;

pub fn starts_with(text: &str, prefix: &str) -> bool {
    text.starts_with(prefix)
}

pub const STARTS_WITH: starts_with_definition::StartsWith = starts_with;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_with_match() {
        assert!(starts_with("Hello World", "Hello"));
        assert!(starts_with("Hello World", "H"));
    }

    #[test]
    fn starts_with_no_match() {
        assert!(!starts_with("Hello World", "World"));
        assert!(!starts_with("Hello World", "hello"));
    }

    #[test]
    fn starts_with_empty_prefix() {
        assert!(starts_with("Hello World", ""));
        assert!(starts_with("", ""));
    }

    #[test]
    fn starts_with_case_sensitive() {
        assert!(!starts_with("México", "méx"));
        assert!(starts_with("México", "Méx"));
    }

    #[test]
    fn starts_with_unicode() {
        assert!(starts_with("México", "Mé"));
        assert!(starts_with("🦀 Rust", "🦀"));
    }

    #[test]
    fn starts_with_function_pointer() {
        let op: starts_with_definition::StartsWith = STARTS_WITH;
        assert!(op("Hello World", "Hello"));
    }
}
