use super::record::Record;
use evo_values::definitions::value::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Construction<'construction> {
    Record(Record<'construction>),
    Value(Value<'construction>),
}
