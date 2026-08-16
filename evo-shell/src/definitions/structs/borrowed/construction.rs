use super::record::Record;
use super::value::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Construction<'construction> {
    Record(Record<'construction>),
    Value(Value<'construction>),
}
