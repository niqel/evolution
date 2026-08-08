pub const NAME: &str = "Evolution Shell";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const DESCRIPTION: &str = "A lightweight functional shell.";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_matches_cargo_pkg_version() {
        assert_eq!(VERSION, env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn static_content_invariants() {
        assert_eq!(NAME, "Evolution Shell");
        assert_eq!(DESCRIPTION, "A lightweight functional shell.");
    }
}
