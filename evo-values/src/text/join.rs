use crate::definitions::text::join as join_definition;
use alloc::borrow::Cow;
use alloc::string::String;

pub fn join<'text>(parts: &[&'text str], separator: &str) -> Cow<'text, str> {
    match parts {
        [] => Cow::Borrowed(""),
        [single] => Cow::Borrowed(*single),
        _ => {
            let mut total_len = 0usize;
            for (idx, part) in parts.iter().enumerate() {
                if idx > 0 {
                    total_len = total_len.saturating_add(separator.len());
                }
                total_len = total_len.saturating_add(part.len());
            }
            let mut result = String::with_capacity(total_len);
            for (idx, part) in parts.iter().enumerate() {
                if idx > 0 {
                    result.push_str(separator);
                }
                result.push_str(part);
            }
            Cow::Owned(result)
        }
    }
}

pub const JOIN: join_definition::Join = join;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn join_empty() {
        let res = join(&[], ",");
        assert_eq!(res, "");
        assert!(matches!(res, Cow::Borrowed(_)));
    }

    #[test]
    fn join_single() {
        let input = "Evo";
        let res = join(&[input], ",");
        assert_eq!(res, "Evo");
        match res {
            Cow::Borrowed(slice) => {
                assert_eq!(slice.as_ptr(), input.as_ptr());
            }
            Cow::Owned(_) => panic!("expected Cow::Borrowed"),
        }
    }

    #[test]
    fn join_two() {
        let res = join(&["a", "b"], ",");
        assert_eq!(res, "a,b");
        assert!(matches!(res, Cow::Owned(_)));
    }

    #[test]
    fn join_multiple() {
        let res = join(&["a", "b", "c"], ",");
        assert_eq!(res, "a,b,c");
        assert!(matches!(res, Cow::Owned(_)));
    }

    #[test]
    fn join_empty_elements() {
        let res1 = join(&["a", "", "b"], ",");
        assert_eq!(res1, "a,,b");
        assert!(matches!(res1, Cow::Owned(_)));

        let res2 = join(&["", "a", ""], ",");
        assert_eq!(res2, ",a,");
        assert!(matches!(res2, Cow::Owned(_)));
    }

    #[test]
    fn join_empty_separator() {
        let res = join(&["a", "b"], "");
        assert_eq!(res, "ab");
        assert!(matches!(res, Cow::Owned(_)));
    }

    #[test]
    fn join_multi_scalar_separator() {
        let res1 = join(&["a", "b"], "---");
        assert_eq!(res1, "a---b");

        let res2 = join(&["a", "b"], "🦀");
        assert_eq!(res2, "a🦀b");
    }

    #[test]
    fn join_unicode() {
        let res = join(&["México", "lindo"], " 🇲🇽 ");
        assert_eq!(res, "México 🇲🇽 lindo");
        assert!(matches!(res, Cow::Owned(_)));
    }

    #[test]
    fn join_function_pointer() {
        let op: join_definition::Join = JOIN;
        let res = op(&["x", "y", "z"], "-");
        assert_eq!(res, "x-y-z");
        assert!(matches!(res, Cow::Owned(_)));
    }
}
