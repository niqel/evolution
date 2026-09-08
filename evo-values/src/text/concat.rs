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
