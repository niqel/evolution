use crate::definitions::text::concat as concat_definition;
use alloc::borrow::Cow;
use alloc::string::String;

pub fn concat<'text>(parts: &[&'text str]) -> Cow<'text, str> {
    match parts {
        [] => Cow::Borrowed(""),
        [single] => Cow::Borrowed(*single),
        _ => {
            let mut total_len = 0usize;
            for part in parts {
                total_len = total_len.saturating_add(part.len());
            }
            let mut result = String::with_capacity(total_len);
            for part in parts {
                result.push_str(part);
            }
            Cow::Owned(result)
        }
    }
}

pub const CONCAT: concat_definition::Concat = concat;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concat_empty() {
        let res = concat(&[]);
        assert_eq!(res, "");
        assert!(matches!(res, Cow::Borrowed(_)));
    }

    #[test]
    fn concat_single() {
        let input = "Gustavo";
        let res = concat(&[input]);
        assert_eq!(res, "Gustavo");
        match res {
            Cow::Borrowed(slice) => {
                assert_eq!(slice.as_ptr(), input.as_ptr());
            }
            Cow::Owned(_) => panic!("expected Cow::Borrowed"),
        }
    }

    #[test]
    fn concat_multiple() {
        let res = concat(&["Gustavo", " ", "Melendez"]);
        assert_eq!(res, "Gustavo Melendez");
        assert!(matches!(res, Cow::Owned(_)));
    }

    #[test]
    fn concat_empty_elements() {
        let res = concat(&["a", "", "b"]);
        assert_eq!(res, "ab");
        assert!(matches!(res, Cow::Owned(_)));
    }

    #[test]
    fn concat_unicode() {
        let res = concat(&["Mé", "xi", "co"]);
        assert_eq!(res, "México");
        assert!(matches!(res, Cow::Owned(_)));
    }

    #[test]
    fn concat_function_pointer() {
        let op: concat_definition::Concat = CONCAT;
        let res = op(&["Hello", " ", "World"]);
        assert_eq!(res, "Hello World");
        assert!(matches!(res, Cow::Owned(_)));
    }
}
