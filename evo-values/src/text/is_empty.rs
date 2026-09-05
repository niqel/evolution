use crate::definitions::text::is_empty as is_empty_definition;

pub fn is_empty(text: &str) -> bool {
    text.is_empty()
}

pub const IS_EMPTY: is_empty_definition::IsEmpty = is_empty;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_empty_empty() {
        assert!(is_empty(""));
    }

    #[test]
    fn is_empty_whitespace() {
        assert!(!is_empty(" "));
        assert!(!is_empty("\n"));
        assert!(!is_empty("\t"));
    }

    #[test]
    fn is_empty_ascii() {
        assert!(!is_empty("abc"));
    }

    #[test]
    fn is_empty_unicode() {
        assert!(!is_empty("México"));
    }

    #[test]
    fn is_empty_function_pointer() {
        let op: is_empty_definition::IsEmpty = IS_EMPTY;
        assert!(op(""));
        assert!(!op("abc"));
    }
}
