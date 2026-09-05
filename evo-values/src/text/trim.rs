use crate::definitions::text::trim as trim_definition;

pub fn trim(text: &str) -> &str {
    text.trim()
}

pub const TRIM: trim_definition::Trim = trim;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_empty() {
        assert_eq!(trim(""), "");
    }

    #[test]
    fn trim_no_whitespace() {
        assert_eq!(trim("Evo"), "Evo");
    }

    #[test]
    fn trim_leading() {
        assert_eq!(trim("   Evo"), "Evo");
    }

    #[test]
    fn trim_trailing() {
        assert_eq!(trim("Evo   "), "Evo");
    }

    #[test]
    fn trim_both_ends() {
        assert_eq!(trim("  Evo  "), "Evo");
        assert_eq!(trim("\n\tEvo \r"), "Evo");
    }

    #[test]
    fn trim_interior_preserved() {
        assert_eq!(trim("  Evo   Language  "), "Evo   Language");
    }

    #[test]
    fn trim_all_whitespace() {
        assert_eq!(trim("   "), "");
        assert_eq!(trim("\t\n\r "), "");
    }

    #[test]
    fn trim_unicode_whitespace() {
        assert_eq!(trim("\u{00A0}Evo\u{3000}"), "Evo");
    }

    #[test]
    fn trim_returns_borrowed_slice() {
        let input: &str = "  hello  ";
        let output: &str = trim(input);
        assert_eq!(output, "hello");
    }

    #[test]
    fn trim_function_pointer() {
        let op: trim_definition::Trim = TRIM;
        assert_eq!(op("  Evo  "), "Evo");
    }
}
