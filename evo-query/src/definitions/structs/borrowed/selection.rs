use super::new_field::NewField;

#[derive(Debug, Clone, PartialEq)]
pub enum Selection<'selection> {
    Field(&'selection str),
    New(NewField<'selection>),
}
