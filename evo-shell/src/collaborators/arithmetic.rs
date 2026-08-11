use crate::definitions::types::number::Number;
use crate::impl_int_op;
use crate::tools::number_inspector::{is_same_integer_type, is_zero};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    UnsupportedTypes,
    Overflow,
    DivisionByZero,
}

pub fn remainder(left: Number, right: Number) -> Result<Number, Error> {
    if is_same_integer_type(left, right) {
        if is_zero(right) {
            return Err(Error::DivisionByZero);
        }
        impl_int_op!(left, right, checked_rem).ok_or(Error::Overflow)
    } else {
        Err(Error::UnsupportedTypes)
    }
}

pub fn negate(value: Number) -> Result<Number, Error> {
    match value {
        Number::I8(a) => a.checked_neg().map(Number::I8).ok_or(Error::Overflow),
        Number::I16(a) => a.checked_neg().map(Number::I16).ok_or(Error::Overflow),
        Number::I32(a) => a.checked_neg().map(Number::I32).ok_or(Error::Overflow),
        Number::I64(a) => a.checked_neg().map(Number::I64).ok_or(Error::Overflow),
        Number::I128(a) => a.checked_neg().map(Number::I128).ok_or(Error::Overflow),
        Number::U8(_) | Number::U16(_) | Number::U32(_) | Number::U64(_) | Number::U128(_) => {
            Err(Error::UnsupportedTypes)
        }
        Number::F32(a) => Ok(Number::F32(-a)),
        Number::F64(a) => Ok(Number::F64(-a)),
    }
}
