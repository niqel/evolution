use crate::definitions::text::ends_with as ends_with_definition;

pub fn ends_with(text: &str, suffix: &str) -> bool {
    text.ends_with(suffix)
}

pub const ENDS_WITH: ends_with_definition::EndsWith = ends_with;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ends_with_match() {
        assert!(ends_with("Hello World", "World"));
        assert!(ends_with("Hello World", "d"));
    }

    #[test]
    fn ends_with_no_match() {
        assert!(!ends_with("Hello World", "Hello"));
        assert!(!ends_with("Hello World", "world"));
    }

    #[test]
    fn ends_with_empty_suffix() {
        assert!(ends_with("Hello World", ""));
        assert!(ends_with("", ""));
    }

    #[test]
    fn ends_with_case_sensitive() {
        assert!(!ends_with("México", "XICO"));
        assert!(ends_with("México", "ico"));
    }

    #[test]
    fn ends_with_unicode() {
        assert!(ends_with("¡Hola México!", "México!"));
        assert!(ends_with("Rust 🦀", "🦀"));
    }

    #[test]
    fn ends_with_function_pointer() {
        let op: ends_with_definition::EndsWith = ENDS_WITH;
        assert!(op("Hello World", "World"));
    }
}
