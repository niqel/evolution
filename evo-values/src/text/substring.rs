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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn substring_ascii() {
        assert_eq!(
            substring("Gustavo", TextPosition(0), TextLength(3)),
            Ok("Gus")
        );
        assert_eq!(
            substring("Gustavo", TextPosition(3), TextLength(4)),
            Ok("tavo")
        );
    }

    #[test]
    fn substring_unicode() {
        assert_eq!(
            substring("México", TextPosition(1), TextLength(3)),
            Ok("éxi")
        );
    }

    #[test]
    fn substring_entire_text() {
        assert_eq!(
            substring("México", TextPosition(0), TextLength(6)),
            Ok("México")
        );
    }

    #[test]
    fn substring_zero_length_beginning() {
        assert_eq!(substring("Gustavo", TextPosition(0), TextLength(0)), Ok(""));
    }

    #[test]
    fn substring_zero_length_end() {
        assert_eq!(substring("Gustavo", TextPosition(7), TextLength(0)), Ok(""));
    }

    #[test]
    fn substring_start_out_of_bounds() {
        assert_eq!(
            substring("Gustavo", TextPosition(8), TextLength(0)),
            Err(TextOperationFailure::OutOfBounds)
        );
        assert_eq!(
            substring("Gustavo", TextPosition(8), TextLength(1)),
            Err(TextOperationFailure::OutOfBounds)
        );
    }

    #[test]
    fn substring_length_out_of_bounds() {
        assert_eq!(
            substring("Gustavo", TextPosition(5), TextLength(3)),
            Err(TextOperationFailure::OutOfBounds)
        );
    }

    #[test]
    fn substring_overflow_safe() {
        assert_eq!(
            substring("Gustavo", TextPosition(usize::MAX), TextLength(1)),
            Err(TextOperationFailure::OutOfBounds)
        );
        assert_eq!(
            substring("Gustavo", TextPosition(1), TextLength(usize::MAX)),
            Err(TextOperationFailure::OutOfBounds)
        );
    }

    #[test]
    fn substring_empty_text_zero_length() {
        assert_eq!(substring("", TextPosition(0), TextLength(0)), Ok(""));
    }

    #[test]
    fn substring_empty_text_invalid() {
        assert_eq!(
            substring("", TextPosition(1), TextLength(0)),
            Err(TextOperationFailure::OutOfBounds)
        );
        assert_eq!(
            substring("", TextPosition(0), TextLength(1)),
            Err(TextOperationFailure::OutOfBounds)
        );
    }

    #[test]
    fn substring_function_pointer() {
        let op: substring_definition::Substring = SUBSTRING;
        assert_eq!(op("México", TextPosition(0), TextLength(3)), Ok("Méx"));
    }
}
