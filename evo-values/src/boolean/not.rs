use crate::definitions::boolean::not as not_definition;

pub fn not(value: bool) -> bool {
    !value
}

pub const NOT: not_definition::Not = not;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_false() {
        assert!(not(false));
    }

    #[test]
    fn not_true() {
        assert!(!not(true));
    }

    #[test]
    fn not_constant() {
        let op: not_definition::Not = NOT;
        assert!(op(false));
        assert!(!op(true));
    }
}
