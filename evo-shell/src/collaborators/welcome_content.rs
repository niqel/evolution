pub const COMPANY: &str = "CatarinaSoft";
pub const MESSAGE: &str = "Evo shell is a life :)";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_welcome_content_invariants() {
        assert_eq!(COMPANY, "CatarinaSoft");
        assert_eq!(MESSAGE, "Evo shell is a life :)");
    }
}
