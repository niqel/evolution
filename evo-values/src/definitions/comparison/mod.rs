pub mod equal;
pub mod greater;
pub mod greater_equal;
pub mod less;
pub mod less_equal;
pub mod not_equal;

pub use equal::{Equal, OwnedEqual};
pub use greater::{Greater, OwnedGreater};
pub use greater_equal::{GreaterEqual, OwnedGreaterEqual};
pub use less::{Less, OwnedLess};
pub use less_equal::{LessEqual, OwnedLessEqual};
pub use not_equal::{NotEqual, OwnedNotEqual};

use crate::definitions::failures::ComparisonFailure;
use crate::definitions::value::{OwnedValue, Value};

pub type ValueComparison =
    for<'left, 'right> fn(&Value<'left>, &Value<'right>) -> Result<bool, ComparisonFailure>;

pub type OwnedValueComparison = fn(&OwnedValue, &OwnedValue) -> Result<bool, ComparisonFailure>;
