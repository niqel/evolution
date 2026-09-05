use crate::definitions::scalars::TextPosition;

pub type Find = fn(&str, &str) -> Option<TextPosition>;
