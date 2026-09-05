use crate::definitions::scalars::TextLength;
use crate::definitions::text::len as len_definition;

pub fn len(text: &str) -> TextLength {
    TextLength(text.chars().count())
}

pub const LEN: len_definition::Len = len;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len_empty() {
        assert_eq!(len(""), TextLength(0));
    }

    #[test]
    fn len_ascii() {
        assert_eq!(len("Gustavo"), TextLength(7));
    }

    #[test]
    fn len_unicode() {
        assert_eq!(len("México"), TextLength(6));
        assert_ne!(len("México").0, "México".len());
    }

    #[test]
    fn len_function_pointer() {
        let op: len_definition::Len = LEN;
        assert_eq!(op("México"), TextLength(6));
    }
}
