use crate::definitions::boolean::and as and_definition;

pub fn and(lhs: bool, rhs: bool) -> bool {
    lhs && rhs
}

pub const AND: and_definition::And = and;
