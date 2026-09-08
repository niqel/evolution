use crate::definitions::failures::TextOperationFailure;
use crate::definitions::scalars::{TextLength, TextPosition};
use crate::definitions::text::substring as substring_definition;

pub fn substring(
    text: &str,
    start: TextPosition,
    length: TextLength,
) -> Result<&str, TextOperationFailure> {
    let start_idx = start.0;
    let len_val = length.0;

    let end = start_idx
        .checked_add(len_val)
        .ok_or(TextOperationFailure::OutOfBounds)?;

    let mut start_byte = None;
    let mut end_byte = None;
    let mut char_count = 0;

    for (byte_idx, _) in text.char_indices() {
        if char_count == start_idx {
            start_byte = Some(byte_idx);
        }
        if char_count == end {
            end_byte = Some(byte_idx);
            break;
        }
        char_count += 1;
    }

    if start_byte.is_none() && start_idx == char_count {
        start_byte = Some(text.len());
    }
    if end_byte.is_none() && end == char_count {
        end_byte = Some(text.len());
    }

    match (start_byte, end_byte) {
        (Some(s), Some(e)) if s <= e => Ok(&text[s..e]),
        _ => Err(TextOperationFailure::OutOfBounds),
    }
}

pub const SUBSTRING: substring_definition::Substring = substring;
