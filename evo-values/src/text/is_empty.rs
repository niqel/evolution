use crate::definitions::text::is_empty as is_empty_definition;

pub fn is_empty(text: &str) -> bool {
    text.is_empty()
}

pub const IS_EMPTY: is_empty_definition::IsEmpty = is_empty;
