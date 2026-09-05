use crate::definitions::boolean::xor as xor_definition;

pub fn xor(lhs: bool, rhs: bool) -> bool {
    lhs ^ rhs
}

pub const XOR: xor_definition::Xor = xor;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xor_truth_table() {
        assert!(!xor(false, false));
        assert!(xor(false, true));
        assert!(xor(true, false));
        assert!(!xor(true, true));
    }

    #[test]
    fn xor_constant() {
        let op: xor_definition::Xor = XOR;
        assert!(!op(false, false));
        assert!(op(false, true));
        assert!(op(true, false));
        assert!(!op(true, true));
    }
}
