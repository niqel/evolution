use crate::definitions::text::substring as substring_definition;

pub fn substring(
    text: &str,
    start: usize,
    length: usize,
) -> Result<&str, substring_definition::Error> {
    let end = start
        .checked_add(length)
        .ok_or(substring_definition::Error::OutOfBounds)?;

    let mut start_byte = None;
    let mut end_byte = None;
    let mut char_count = 0;

    for (byte_idx, _) in text.char_indices() {
        if char_count == start {
            start_byte = Some(byte_idx);
        }
        if char_count == end {
            end_byte = Some(byte_idx);
            break;
        }
        char_count += 1;
    }

    if start_byte.is_none() && start == char_count {
        start_byte = Some(text.len());
    }
    if end_byte.is_none() && end == char_count {
        end_byte = Some(text.len());
    }

    match (start_byte, end_byte) {
        (Some(s), Some(e)) if s <= e => Ok(&text[s..e]),
        _ => Err(substring_definition::Error::OutOfBounds),
    }
}

pub const SUBSTRING: substring_definition::Substring = substring;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn substring_ascii() {
        assert_eq!(substring("Gustavo", 0, 3), Ok("Gus"));
        assert_eq!(substring("Gustavo", 3, 4), Ok("tavo"));
    }

    #[test]
    fn substring_unicode() {
        assert_eq!(substring("México", 1, 3), Ok("éxi"));
    }

    #[test]
    fn substring_entire_text() {
        assert_eq!(substring("México", 0, 6), Ok("México"));
    }

    #[test]
    fn substring_zero_length_beginning() {
        assert_eq!(substring("Gustavo", 0, 0), Ok(""));
    }

    #[test]
    fn substring_zero_length_end() {
        assert_eq!(substring("Gustavo", 7, 0), Ok(""));
    }

    #[test]
    fn substring_start_out_of_bounds() {
        assert_eq!(
            substring("Gustavo", 8, 0),
            Err(substring_definition::Error::OutOfBounds)
        );
        assert_eq!(
            substring("Gustavo", 8, 1),
            Err(substring_definition::Error::OutOfBounds)
        );
    }

    #[test]
    fn substring_length_out_of_bounds() {
        assert_eq!(
            substring("Gustavo", 5, 3),
            Err(substring_definition::Error::OutOfBounds)
        );
    }

    #[test]
    fn substring_overflow_safe() {
        assert_eq!(
            substring("Gustavo", usize::MAX, 1),
            Err(substring_definition::Error::OutOfBounds)
        );
        assert_eq!(
            substring("Gustavo", 1, usize::MAX),
            Err(substring_definition::Error::OutOfBounds)
        );
    }

    #[test]
    fn substring_empty_text_zero_length() {
        assert_eq!(substring("", 0, 0), Ok(""));
    }

    #[test]
    fn substring_empty_text_invalid() {
        assert_eq!(
            substring("", 1, 0),
            Err(substring_definition::Error::OutOfBounds)
        );
        assert_eq!(
            substring("", 0, 1),
            Err(substring_definition::Error::OutOfBounds)
        );
    }

    #[test]
    fn substring_function_pointer() {
        let op: substring_definition::Substring = SUBSTRING;
        assert_eq!(op("México", 0, 3), Ok("Méx"));
    }
}
