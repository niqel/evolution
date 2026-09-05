pub(crate) mod kernel;

pub mod equal;
pub mod greater;
pub mod greater_equal;
pub mod less;
pub mod less_equal;
pub mod not_equal;

#[cfg(test)]
mod tests;

pub use equal::{EQUAL, OWNED_EQUAL, equal, owned_equal};
pub use greater::{GREATER, OWNED_GREATER, greater, owned_greater};
pub use greater_equal::{GREATER_EQUAL, OWNED_GREATER_EQUAL, greater_equal, owned_greater_equal};
pub use less::{LESS, OWNED_LESS, less, owned_less};
pub use less_equal::{LESS_EQUAL, OWNED_LESS_EQUAL, less_equal, owned_less_equal};
pub use not_equal::{NOT_EQUAL, OWNED_NOT_EQUAL, not_equal, owned_not_equal};

pub use crate::definitions::comparison::*;
