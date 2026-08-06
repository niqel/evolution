use crate::definitions::domain::value_objects::structured_items::StructuredItems;

pub type Index =
    for<'a> fn(items: StructuredItems<'a>, index: usize) -> Result<StructuredItems<'a>, IndexError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IndexError {
    NotFound { index: usize },
    AmbiguousIndex { index: usize },
}

impl IndexError {
    pub fn not_found(index: usize) -> Self {
        Self::NotFound { index }
    }

    pub fn ambiguous_index(index: usize) -> Self {
        Self::AmbiguousIndex { index }
    }
}
