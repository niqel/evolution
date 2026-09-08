use crate::definitions::scalars::TextPosition;
use crate::definitions::text::find as find_definition;

pub fn find(text: &str, pattern: &str) -> Option<TextPosition> {
    if pattern.is_empty() {
        return Some(TextPosition(0));
    }
    text.find(pattern).map(|byte_idx| {
        let char_pos = text[..byte_idx].chars().count();
        TextPosition(char_pos)
    })
}

pub const FIND: find_definition::Find = find;
