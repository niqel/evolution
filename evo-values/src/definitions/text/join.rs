use alloc::borrow::Cow;

pub type Join = for<'text> fn(&[&'text str], &str) -> Cow<'text, str>;
