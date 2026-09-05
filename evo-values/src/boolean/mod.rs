pub mod and;
pub mod not;
pub mod or;
pub mod xor;

pub use and::{AND, and};
pub use not::{NOT, not};
pub use or::{OR, or};
pub use xor::{XOR, xor};

pub use crate::definitions::boolean::{And, Not, Or, Xor};
