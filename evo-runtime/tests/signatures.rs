use evo_runtime::definitions::{requesters::run_request, use_cases::start};

fn run_example() {}

fn start_example(run: run_request::Request) {
    let _ = run;
}

#[test]
fn run_request_type_compatibility() {
    let _: run_request::Request = run_example;
}

#[test]
fn start_use_case_type_compatibility() {
    let _: start::Start = start_example;
}
