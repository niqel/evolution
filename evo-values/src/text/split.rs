use crate::definitions::control::ProductionControl;
use crate::definitions::failures::TextOperationFailure;
pub use crate::definitions::text::split::{ReceiveTextSegment, Split};

pub fn split<State>(
    text: &str,
    separator: &str,
    state: &mut State,
    receiver: ReceiveTextSegment<State>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::String;
    use alloc::vec::Vec;

    #[derive(Default)]
    struct CollectorState {
        segments: Vec<String>,
        call_count: usize,
    }

    fn collect_all(state: &mut CollectorState, segment: &str) -> ProductionControl {
        state.call_count += 1;
        state.segments.push(String::from(segment));
        ProductionControl::Continue
    }

    #[test]
    fn split_normal() {
        let mut state = CollectorState::default();
        let res = split("a,b,c", ",", &mut state, collect_all);
        assert_eq!(res, Ok(()));
        assert_eq!(state.segments, ["a", "b", "c"]);
        assert_eq!(state.call_count, 3);
    }

    #[test]
    fn split_separator_not_found() {
        let mut state = CollectorState::default();
        let res = split("abc", ",", &mut state, collect_all);
        assert_eq!(res, Ok(()));
        assert_eq!(state.segments, ["abc"]);
        assert_eq!(state.call_count, 1);
    }

    #[test]
    fn split_empty_text() {
        let mut state = CollectorState::default();
        let res = split("", ",", &mut state, collect_all);
        assert_eq!(res, Ok(()));
        assert_eq!(state.segments, [""]);
        assert_eq!(state.call_count, 1);
    }

    #[test]
    fn split_empty_separator_fails_without_invoking_receiver() {
        let mut state = CollectorState::default();
        let res = split("abc", "", &mut state, collect_all);
        assert_eq!(res, Err(TextOperationFailure::EmptySeparator));
        assert_eq!(state.call_count, 0);
        assert!(state.segments.is_empty());
    }

    #[test]
    fn split_early_stop() {
        struct StopAtBState {
            segments: Vec<String>,
            call_count: usize,
        }

        fn receiver(state: &mut StopAtBState, segment: &str) -> ProductionControl {
            state.call_count += 1;
            state.segments.push(String::from(segment));
            if segment == "b" {
                ProductionControl::Stop
            } else {
                ProductionControl::Continue
            }
        }

        let mut state = StopAtBState {
            segments: Vec::new(),
            call_count: 0,
        };

        let res = split("a,b,c,d", ",", &mut state, receiver);
        assert_eq!(res, Ok(()));
        assert_eq!(state.segments, ["a", "b"]);
        assert_eq!(state.call_count, 2);
    }

    #[test]
    fn split_function_pointer_contract() {
        let operation: Split<CollectorState> = split::<CollectorState>;
        let mut state = CollectorState::default();
        let res = operation("hello world", " ", &mut state, collect_all);
        assert_eq!(res, Ok(()));
        assert_eq!(state.segments, ["hello", "world"]);
    }
}
