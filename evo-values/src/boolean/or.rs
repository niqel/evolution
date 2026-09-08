use crate::definitions::boolean::or as or_definition;

pub fn or(lhs: bool, rhs: bool) -> bool {
    lhs || rhs
}

pub const OR: or_definition::Or = or;
