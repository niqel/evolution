use crate::definitions::control::ProductionControl;
use crate::definitions::failures::TextOperationFailure;

pub type ReceiveTextSegment<State> =
    for<'segment> fn(&mut State, &'segment str) -> ProductionControl;

pub type Split<State> = for<'text, 'separator> fn(
    &'text str,
    &'separator str,
    &mut State,
    ReceiveTextSegment<State>,
) -> Result<(), TextOperationFailure>;
