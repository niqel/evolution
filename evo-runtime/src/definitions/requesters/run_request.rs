pub type Request = fn();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_request_type_compatibility() {
        fn run_example() {}

        let _: Request = run_example;
    }
}
