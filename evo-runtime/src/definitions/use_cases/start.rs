use crate::definitions::requesters::run_request;

pub type Start = fn(run_request::Request);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_use_case_type_compatibility() {
        fn start_example(run: run_request::Request) {
            let _ = run;
        }

        let _: Start = start_example;
    }
}
