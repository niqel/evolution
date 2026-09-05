use crate::definitions::text::contains as contains_definition;

pub fn contains(text: &str, pattern: &str) -> bool {
    text.contains(pattern)
}

pub const CONTAINS: contains_definition::Contains = contains;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_match() {
        assert!(contains("Hello World", "World"));
        assert!(contains("Hello World", "Hello"));
        assert!(contains("Hello World", "o W"));
    }

    #[test]
    fn contains_no_match() {
        assert!(!contains("Hello World", "world"));
        assert!(!contains("Hello World", "Foo"));
    }

    #[test]
    fn contains_empty_pattern() {
        assert!(contains("Hello World", ""));
        assert!(contains("", ""));
    }

    #[test]
    fn contains_case_sensitive() {
        assert!(!contains("México", "méxico"));
        assert!(contains("México", "México"));
    }

    #[test]
    fn contains_unicode() {
        assert!(contains("México lindo", "éxi"));
        assert!(contains("🦀 Rust", "🦀"));
    }

    #[test]
    fn contains_function_pointer() {
        let op: contains_definition::Contains = CONTAINS;
        assert!(op("Hello World", "World"));
    }
}
