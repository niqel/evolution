use crate::definitions::failures::TextOperationFailure;
use crate::definitions::text::replace as replace_definition;
use alloc::borrow::Cow;
use alloc::string::String;

pub fn replace<'text>(
    text: &'text str,
    pattern: &str,
    replacement: &str,
) -> Result<Cow<'text, str>, TextOperationFailure> {
    if pattern.is_empty() {
        return Err(TextOperationFailure::EmptyPattern);
    }

    if pattern == replacement {
        return Ok(Cow::Borrowed(text));
    }

    if !text.contains(pattern) {
        return Ok(Cow::Borrowed(text));
    }

    let mut result = String::new();
    let mut last_end = 0;
    let pattern_len = pattern.len();

    for (start, _) in text.match_indices(pattern) {
        if start >= last_end {
            result.push_str(&text[last_end..start]);
            result.push_str(replacement);
            last_end = start + pattern_len;
        }
    }

    result.push_str(&text[last_end..]);
    Ok(Cow::Owned(result))
}

pub const REPLACE: replace_definition::Replace = replace;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_one() {
        let res = replace("hello world", "world", "rust").unwrap();
        assert_eq!(res, "hello rust");
        assert!(matches!(res, Cow::Owned(_)));
    }

    #[test]
    fn replace_multiple() {
        let res = replace("one two one", "one", "1").unwrap();
        assert_eq!(res, "1 two 1");
        assert!(matches!(res, Cow::Owned(_)));
    }

    #[test]
    fn replace_no_match() {
        let input = "hello world";
        let res = replace(input, "abc", "123").unwrap();
        assert_eq!(res, "hello world");
        match res {
            Cow::Borrowed(slice) => {
                assert_eq!(slice.as_ptr(), input.as_ptr());
            }
            Cow::Owned(_) => panic!("expected Cow::Borrowed"),
        }
    }

    #[test]
    fn replace_pattern_equals_replacement() {
        let input = "hello world";
        let res = replace(input, "world", "world").unwrap();
        assert_eq!(res, "hello world");
        match res {
            Cow::Borrowed(slice) => {
                assert_eq!(slice.as_ptr(), input.as_ptr());
            }
            Cow::Owned(_) => panic!("expected Cow::Borrowed"),
        }
    }

    #[test]
    fn replace_unicode() {
        let res = replace("México lindo y querido", "México", "MÉXICO").unwrap();
        assert_eq!(res, "MÉXICO lindo y querido");
        assert!(matches!(res, Cow::Owned(_)));
    }

    #[test]
    fn replace_case_sensitive() {
        let res = replace("Case CASE case", "case", "word").unwrap();
        assert_eq!(res, "Case CASE word");
        assert!(matches!(res, Cow::Owned(_)));
    }

    #[test]
    fn replace_to_empty() {
        let res = replace("Gustavo", "avo", "").unwrap();
        assert_eq!(res, "Gust");
        assert!(matches!(res, Cow::Owned(_)));
    }

    #[test]
    fn replace_empty_pattern_error() {
        assert_eq!(
            replace("hello", "", "world"),
            Err(TextOperationFailure::EmptyPattern)
        );
    }

    #[test]
    fn replace_non_overlapping() {
        let res = replace("aaaa", "aa", "x").unwrap();
        assert_eq!(res, "xx");
        assert!(matches!(res, Cow::Owned(_)));
    }

    #[test]
    fn replace_function_pointer() {
        let op: replace_definition::Replace = REPLACE;
        let res = op("one two one", "one", "1").unwrap();
        assert_eq!(res, "1 two 1");
        assert!(matches!(res, Cow::Owned(_)));
    }
}
