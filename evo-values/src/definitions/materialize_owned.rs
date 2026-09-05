use crate::definitions::value::{OwnedValue, Value};

pub type MaterializeOwned = for<'value> fn(&Value<'value>) -> OwnedValue;
