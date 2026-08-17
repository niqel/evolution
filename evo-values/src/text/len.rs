use crate::definitions::text::len as len_definition;

pub fn len(text: &str) -> usize {
    text.chars().count()
}

pub const LEN: len_definition::Len = len;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len_empty() {
        assert_eq!(len(""), 0);
    }

    #[test]
    fn len_ascii() {
        assert_eq!(len("Gustavo"), 7);
    }

    #[test]
    fn len_unicode() {
        assert_eq!(len("México"), 6);
        assert_ne!(len("México"), "México".len());
    }

    #[test]
    fn len_function_pointer() {
        let op: len_definition::Len = LEN;
        assert_eq!(op("México"), 6);
    }
}
