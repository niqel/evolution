use crate::definitions::{requesters::run_request, use_cases::start};

pub fn start(run: run_request::Request) {
    run();
}

pub const START: start::Start = start;
