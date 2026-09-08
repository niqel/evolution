use crate::definitions::boolean::not as not_definition;

pub fn not(value: bool) -> bool {
    !value
}

pub const NOT: not_definition::Not = not;
