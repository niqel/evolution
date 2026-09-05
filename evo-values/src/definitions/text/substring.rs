use crate::definitions::failures::TextOperationFailure;
use crate::definitions::scalars::{TextLength, TextPosition};

pub type Substring =
    for<'text> fn(&'text str, TextPosition, TextLength) -> Result<&'text str, TextOperationFailure>;
