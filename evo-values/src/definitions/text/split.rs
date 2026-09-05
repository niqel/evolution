use crate::definitions::control::ProductionControl;
use crate::definitions::failures::TextOperationFailure;

pub type ReceiveTextSegment<'text, State> = fn(&mut State, &'text str) -> ProductionControl;

pub type Split<'text, State> = fn(
    &'text str,
    &str,
    &mut State,
    ReceiveTextSegment<'text, State>,
) -> Result<(), TextOperationFailure>;
