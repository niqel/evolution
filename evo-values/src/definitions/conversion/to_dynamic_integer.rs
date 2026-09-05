use crate::definitions::value::OwnedDynamicValue;

pub type ToDynamicInteger<Source> = fn(Source) -> OwnedDynamicValue;
