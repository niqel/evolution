use crate::definitions::boolean::xor as xor_definition;

pub fn xor(lhs: bool, rhs: bool) -> bool {
    lhs ^ rhs
}

pub const XOR: xor_definition::Xor = xor;
