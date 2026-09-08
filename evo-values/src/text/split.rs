use crate::definitions::control::ProductionControl;
use crate::definitions::failures::TextOperationFailure;
pub use crate::definitions::text::split::{ReceiveTextSegment, Split};

pub fn split<'text, State>(
    text: &'text str,
    separator: &str,
    state: &mut State,
    receiver: ReceiveTextSegment<'text, State>,
) -> Result<(), TextOperationFailure> {
    if separator.is_empty() {
        return Err(TextOperationFailure::EmptySeparator);
    }

    for segment in text.split(separator) {
        match receiver(state, segment) {
            ProductionControl::Continue => {}
            ProductionControl::Stop => return Ok(()),
        }
    }

    Ok(())
}
