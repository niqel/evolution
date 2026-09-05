use crate::definitions::scalars::TextPosition;
use crate::definitions::text::find as find_definition;

pub fn find(text: &str, pattern: &str) -> Option<TextPosition> {
    if pattern.is_empty() {
        return Some(TextPosition(0));
    }
    text.find(pattern).map(|byte_idx| {
        let char_pos = text[..byte_idx].chars().count();
        TextPosition(char_pos)
    })
}

pub const FIND: find_definition::Find = find;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_match_at_start() {
        assert_eq!(find("Hello World", "Hello"), Some(TextPosition(0)));
    }

    #[test]
    fn find_match_in_middle() {
        assert_eq!(find("Hello World", "World"), Some(TextPosition(6)));
    }

    #[test]
    fn find_first_of_multiple_matches() {
        assert_eq!(find("ab ab ab", "ab"), Some(TextPosition(0)));
        assert_eq!(find("banana", "an"), Some(TextPosition(1)));
    }

    #[test]
    fn find_no_match() {
        assert_eq!(find("Hello World", "404"), None);
    }

    #[test]
    fn find_empty_pattern() {
        assert_eq!(find("Hello World", ""), Some(TextPosition(0)));
        assert_eq!(find("", ""), Some(TextPosition(0)));
    }

    #[test]
    fn find_case_sensitive() {
        assert_eq!(find("Hello World", "world"), None);
        assert_eq!(find("Hello World", "World"), Some(TextPosition(6)));
    }

    #[test]
    fn find_unicode_multibyte_before_match() {
        // Mandatory test case: find("México", "x") -> Some(TextPosition(2))
        // In UTF-8, 'M' is 1 byte, 'é' is 2 bytes, so 'x' is at byte offset 3, but scalar position 2.
        assert_eq!(find("México", "x"), Some(TextPosition(2)));

        // Multi-byte emoji before match
        assert_eq!(find("🦀🦀Rust", "Rust"), Some(TextPosition(2)));
    }

    #[test]
    fn find_function_pointer() {
        let op: find_definition::Find = FIND;
        assert_eq!(op("México", "x"), Some(TextPosition(2)));
    }
}
