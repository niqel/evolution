use alloc::borrow::Cow;

pub type Concat = for<'text> fn(&[&'text str]) -> Cow<'text, str>;
