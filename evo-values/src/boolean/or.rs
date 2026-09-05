use crate::definitions::boolean::or as or_definition;

pub fn or(lhs: bool, rhs: bool) -> bool {
    lhs || rhs
}

pub const OR: or_definition::Or = or;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn or_truth_table() {
        assert!(!or(false, false));
        assert!(or(false, true));
        assert!(or(true, false));
        assert!(or(true, true));
    }

    #[test]
    fn or_constant() {
        let op: or_definition::Or = OR;
        assert!(!op(false, false));
        assert!(op(false, true));
        assert!(op(true, false));
        assert!(op(true, true));
    }
}
