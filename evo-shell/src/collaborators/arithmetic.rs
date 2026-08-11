use crate::definitions::types::number::Number;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    UnsupportedTypes,
    Overflow,
    DivisionByZero,
}

fn is_integer(num: Number) -> bool {
    matches!(
        num,
        Number::I8(_)
            | Number::I16(_)
            | Number::I32(_)
            | Number::I64(_)
            | Number::I128(_)
            | Number::U8(_)
            | Number::U16(_)
            | Number::U32(_)
            | Number::U64(_)
            | Number::U128(_)
    )
}

fn is_zero(num: Number) -> bool {
    matches!(
        num,
        Number::I8(0)
            | Number::I16(0)
            | Number::I32(0)
            | Number::I64(0)
            | Number::I128(0)
            | Number::U8(0)
            | Number::U16(0)
            | Number::U32(0)
            | Number::U64(0)
            | Number::U128(0)
    )
}

fn is_same_integer_type(left: Number, right: Number) -> bool {
    matches!(
        (left, right),
        (Number::I8(_), Number::I8(_))
            | (Number::I16(_), Number::I16(_))
            | (Number::I32(_), Number::I32(_))
            | (Number::I64(_), Number::I64(_))
            | (Number::I128(_), Number::I128(_))
            | (Number::U8(_), Number::U8(_))
            | (Number::U16(_), Number::U16(_))
            | (Number::U32(_), Number::U32(_))
            | (Number::U64(_), Number::U64(_))
            | (Number::U128(_), Number::U128(_))
    )
}

fn to_f32(num: Number) -> Option<f32> {
    match num {
        Number::I8(v) => Some(v as f32),
        Number::I16(v) => Some(v as f32),
        Number::I32(v) => Some(v as f32),
        Number::I64(v) => Some(v as f32),
        Number::I128(v) => Some(v as f32),
        Number::U8(v) => Some(v as f32),
        Number::U16(v) => Some(v as f32),
        Number::U32(v) => Some(v as f32),
        Number::U64(v) => Some(v as f32),
        Number::U128(v) => Some(v as f32),
        Number::F32(v) => Some(v),
        Number::F64(_) => None,
    }
}

fn to_f64(num: Number) -> Option<f64> {
    match num {
        Number::I8(v) => Some(v as f64),
        Number::I16(v) => Some(v as f64),
        Number::I32(v) => Some(v as f64),
        Number::I64(v) => Some(v as f64),
        Number::I128(v) => Some(v as f64),
        Number::U8(v) => Some(v as f64),
        Number::U16(v) => Some(v as f64),
        Number::U32(v) => Some(v as f64),
        Number::U64(v) => Some(v as f64),
        Number::U128(v) => Some(v as f64),
        Number::F32(v) => Some(v as f64),
        Number::F64(v) => Some(v),
    }
}

macro_rules! impl_int_op {
    ($left:expr, $right:expr, $op:ident) => {
        match ($left, $right) {
            (Number::I8(a), Number::I8(b)) => a.$op(b).map(Number::I8),
            (Number::I16(a), Number::I16(b)) => a.$op(b).map(Number::I16),
            (Number::I32(a), Number::I32(b)) => a.$op(b).map(Number::I32),
            (Number::I64(a), Number::I64(b)) => a.$op(b).map(Number::I64),
            (Number::I128(a), Number::I128(b)) => a.$op(b).map(Number::I128),
            (Number::U8(a), Number::U8(b)) => a.$op(b).map(Number::U8),
            (Number::U16(a), Number::U16(b)) => a.$op(b).map(Number::U16),
            (Number::U32(a), Number::U32(b)) => a.$op(b).map(Number::U32),
            (Number::U64(a), Number::U64(b)) => a.$op(b).map(Number::U64),
            (Number::U128(a), Number::U128(b)) => a.$op(b).map(Number::U128),
            _ => None,
        }
    };
}

pub fn add(left: Number, right: Number) -> Result<Number, Error> {
    if is_same_integer_type(left, right) {
        impl_int_op!(left, right, checked_add).ok_or(Error::Overflow)
    } else if is_integer(left) && is_integer(right) {
        Err(Error::UnsupportedTypes)
    } else if matches!(left, Number::F64(_)) || matches!(right, Number::F64(_)) {
        let l = to_f64(left).ok_or(Error::UnsupportedTypes)?;
        let r = to_f64(right).ok_or(Error::UnsupportedTypes)?;
        Ok(Number::F64(l + r))
    } else {
        let l = to_f32(left).ok_or(Error::UnsupportedTypes)?;
        let r = to_f32(right).ok_or(Error::UnsupportedTypes)?;
        Ok(Number::F32(l + r))
    }
}

pub fn subtract(left: Number, right: Number) -> Result<Number, Error> {
    if is_same_integer_type(left, right) {
        impl_int_op!(left, right, checked_sub).ok_or(Error::Overflow)
    } else if is_integer(left) && is_integer(right) {
        Err(Error::UnsupportedTypes)
    } else if matches!(left, Number::F64(_)) || matches!(right, Number::F64(_)) {
        let l = to_f64(left).ok_or(Error::UnsupportedTypes)?;
        let r = to_f64(right).ok_or(Error::UnsupportedTypes)?;
        Ok(Number::F64(l - r))
    } else {
        let l = to_f32(left).ok_or(Error::UnsupportedTypes)?;
        let r = to_f32(right).ok_or(Error::UnsupportedTypes)?;
        Ok(Number::F32(l - r))
    }
}

pub fn multiply(left: Number, right: Number) -> Result<Number, Error> {
    if is_same_integer_type(left, right) {
        impl_int_op!(left, right, checked_mul).ok_or(Error::Overflow)
    } else if is_integer(left) && is_integer(right) {
        Err(Error::UnsupportedTypes)
    } else if matches!(left, Number::F64(_)) || matches!(right, Number::F64(_)) {
        let l = to_f64(left).ok_or(Error::UnsupportedTypes)?;
        let r = to_f64(right).ok_or(Error::UnsupportedTypes)?;
        Ok(Number::F64(l * r))
    } else {
        let l = to_f32(left).ok_or(Error::UnsupportedTypes)?;
        let r = to_f32(right).ok_or(Error::UnsupportedTypes)?;
        Ok(Number::F32(l * r))
    }
}

pub fn divide(left: Number, right: Number) -> Result<Number, Error> {
    if is_same_integer_type(left, right) {
        if is_zero(right) {
            return Err(Error::DivisionByZero);
        }
        impl_int_op!(left, right, checked_div).ok_or(Error::Overflow)
    } else if is_integer(left) && is_integer(right) {
        Err(Error::UnsupportedTypes)
    } else if matches!(left, Number::F64(_)) || matches!(right, Number::F64(_)) {
        let l = to_f64(left).ok_or(Error::UnsupportedTypes)?;
        let r = to_f64(right).ok_or(Error::UnsupportedTypes)?;
        Ok(Number::F64(l / r))
    } else {
        let l = to_f32(left).ok_or(Error::UnsupportedTypes)?;
        let r = to_f32(right).ok_or(Error::UnsupportedTypes)?;
        Ok(Number::F32(l / r))
    }
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
