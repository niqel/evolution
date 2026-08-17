use crate::definitions::text::replace as replace_definition;
use alloc::string::String;

pub fn replace(text: &str, from: &str, to: &str) -> Result<String, replace_definition::Error> {
    if from.is_empty() {
        return Err(replace_definition::Error::EmptyPattern);
    }

    let mut result = String::new();
    let mut last_end = 0;
    let from_len = from.len();

    for (start, _) in text.match_indices(from) {
        if start >= last_end {
            result.push_str(&text[last_end..start]);
            result.push_str(to);
            last_end = start + from_len;
        }
    }

    result.push_str(&text[last_end..]);
    Ok(result)
}

pub const REPLACE: replace_definition::Replace = replace;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_one() {
        assert_eq!(
            replace("hello world", "world", "rust"),
            Ok(String::from("hello rust"))
        );
    }

    #[test]
    fn replace_multiple() {
        assert_eq!(
            replace("one two one", "one", "1"),
            Ok(String::from("1 two 1"))
        );
    }

    #[test]
    fn replace_no_match() {
        assert_eq!(
            replace("hello world", "abc", "123"),
            Ok(String::from("hello world"))
        );
    }

    #[test]
    fn replace_unicode() {
        assert_eq!(
            replace("México lindo y querido", "México", "MÉXICO"),
            Ok(String::from("MÉXICO lindo y querido"))
        );
    }

    #[test]
    fn replace_case_sensitive() {
        assert_eq!(
            replace("Case CASE case", "case", "word"),
            Ok(String::from("Case CASE word"))
        );
    }

    #[test]
    fn replace_to_empty() {
        assert_eq!(replace("Gustavo", "avo", ""), Ok(String::from("Gust")));
    }

    #[test]
    fn replace_empty_pattern_error() {
        assert_eq!(
            replace("hello", "", "world"),
            Err(replace_definition::Error::EmptyPattern)
        );
    }

    #[test]
    fn replace_function_pointer() {
        let op: replace_definition::Replace = REPLACE;
        assert_eq!(op("one two one", "one", "1"), Ok(String::from("1 two 1")));
    }
}
