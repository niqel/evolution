use crate::definitions::scalars::TextLength;
use crate::definitions::text::len as len_definition;

pub fn len(text: &str) -> TextLength {
    TextLength(text.chars().count())
}

pub const LEN: len_definition::Len = len;
