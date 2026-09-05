use crate::definitions::boolean::and as and_definition;

pub fn and(lhs: bool, rhs: bool) -> bool {
    lhs && rhs
}

pub const AND: and_definition::And = and;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn and_truth_table() {
        assert!(!and(false, false));
        assert!(!and(false, true));
        assert!(!and(true, false));
        assert!(and(true, true));
    }

    #[test]
    fn and_constant() {
        let op: and_definition::And = AND;
        assert!(!op(false, false));
        assert!(!op(false, true));
        assert!(!op(true, false));
        assert!(op(true, true));
    }
}
