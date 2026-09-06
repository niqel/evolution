use crate::definitions::value::{DynamicValue, OwnedDynamicValue};

pub type DynamicNegate = for<'value> fn(&DynamicValue<'value>) -> OwnedDynamicValue;
