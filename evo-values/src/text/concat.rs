use crate::definitions::text::concat as concat_definition;
use alloc::string::String;

pub fn concat(parts: &[&str]) -> String {
    let mut result = String::new();
    for part in parts {
        result.push_str(part);
    }
    result
}

pub const CONCAT: concat_definition::Concat = concat;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concat_empty() {
        assert_eq!(concat(&[]), "");
    }

    #[test]
    fn concat_single() {
        assert_eq!(concat(&["Gustavo"]), "Gustavo");
    }

    #[test]
    fn concat_multiple() {
        assert_eq!(concat(&["Gustavo", " ", "Melendez"]), "Gustavo Melendez");
    }

    #[test]
    fn concat_unicode() {
        assert_eq!(concat(&["Mé", "xi", "co"]), "México");
    }

    #[test]
    fn concat_function_pointer() {
        let op: concat_definition::Concat = CONCAT;
        assert_eq!(op(&["Hello", " ", "World"]), "Hello World");
    }
}
