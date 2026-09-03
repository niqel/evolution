use super::field::Field;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Record<'record> {
    pub fields: &'record [Field<'record>],
}
