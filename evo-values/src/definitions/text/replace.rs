use crate::definitions::failures::TextOperationFailure;
use alloc::borrow::Cow;

pub type Replace =
    for<'text> fn(&'text str, &str, &str) -> Result<Cow<'text, str>, TextOperationFailure>;
