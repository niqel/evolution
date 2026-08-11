use crate::definitions::types::number::Number;
use crate::definitions::use_cases::multiply;
use crate::impl_int_op;
use crate::tools::number_coercion::{to_f32, to_f64};
use crate::tools::number_inspector::{is_integer, is_same_integer_type};

pub fn collaborate(left: Number, right: Number) -> Result<Number, multiply::Error> {
    if is_same_integer_type(left, right) {
        impl_int_op!(left, right, checked_mul).ok_or(multiply::Error::Overflow)
    } else if is_integer(left) && is_integer(right) {
        Err(multiply::Error::UnsupportedTypes)
    } else if matches!(left, Number::F64(_)) || matches!(right, Number::F64(_)) {
        let l = to_f64(left).ok_or(multiply::Error::UnsupportedTypes)?;
        let r = to_f64(right).ok_or(multiply::Error::UnsupportedTypes)?;
        Ok(Number::F64(l * r))
    } else {
        let l = to_f32(left).ok_or(multiply::Error::UnsupportedTypes)?;
        let r = to_f32(right).ok_or(multiply::Error::UnsupportedTypes)?;
        Ok(Number::F32(l * r))
    }
}
