use evo_values::definitions::value::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Field<'field> {
    pub name: &'field str,
    pub value: Value<'field>,
}
