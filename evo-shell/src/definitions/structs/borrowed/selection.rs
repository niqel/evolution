use super::new_field::NewField;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Selection<'selection> {
    Field(&'selection str),
    New(NewField<'selection>),
}
