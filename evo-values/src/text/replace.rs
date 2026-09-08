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
