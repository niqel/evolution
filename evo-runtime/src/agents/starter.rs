use crate::definitions::{requesters::run_request, use_cases::start};

pub fn start(run: run_request::Request) {
    run();
}

pub const START: start::Start = start;

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

    #[test]
    fn test_binding_exact() {
        let _: start::Start = START;
    }

    #[test]
    fn test_run_invoked_exactly_once() {
        static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

        fn count_run() {
            CALL_COUNT.fetch_add(1, Ordering::SeqCst);
        }

        assert_eq!(CALL_COUNT.load(Ordering::SeqCst), 0);
        START(count_run);
        assert_eq!(CALL_COUNT.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_return_propagation_when_run_returns() {
        static RUN_COMPLETED: AtomicBool = AtomicBool::new(false);

        fn completing_run() {
            RUN_COMPLETED.store(true, Ordering::SeqCst);
        }

        assert!(!RUN_COMPLETED.load(Ordering::SeqCst));
        START(completing_run);
        assert!(RUN_COMPLETED.load(Ordering::SeqCst));
    }

    #[test]
    fn test_independent_invocations() {
        static RUN_A_COUNT: AtomicUsize = AtomicUsize::new(0);
        static RUN_B_COUNT: AtomicUsize = AtomicUsize::new(0);

        fn run_a() {
            RUN_A_COUNT.fetch_add(1, Ordering::SeqCst);
        }

        fn run_b() {
            RUN_B_COUNT.fetch_add(1, Ordering::SeqCst);
        }

        assert_eq!(RUN_A_COUNT.load(Ordering::SeqCst), 0);
        assert_eq!(RUN_B_COUNT.load(Ordering::SeqCst), 0);

        START(run_a);
        assert_eq!(RUN_A_COUNT.load(Ordering::SeqCst), 1);
        assert_eq!(RUN_B_COUNT.load(Ordering::SeqCst), 0);

        START(run_b);
        assert_eq!(RUN_A_COUNT.load(Ordering::SeqCst), 1);
        assert_eq!(RUN_B_COUNT.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_no_outcome() {
        fn run_example() {}

        let start_fn: start::Start = START;
        let result: () = start_fn(run_example);
        assert_eq!(result, ());
    }
}
