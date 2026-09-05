use crate::definitions::value::OwnedDynamicValue;

pub type ToDynamicFloat64 = fn(f64) -> OwnedDynamicValue;
