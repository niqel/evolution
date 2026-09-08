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
