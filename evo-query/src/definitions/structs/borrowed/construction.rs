use super::record::Record;
use evo_values::definitions::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum Construction<'construction> {
    Record(Record<'construction>),
    Value(Value<'construction>),
}
