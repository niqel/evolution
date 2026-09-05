#![allow(dead_code)]

use crate::definitions::failures::ComparisonFailure;
use crate::definitions::value::{DynamicIntegerValue, DynamicValue, Value};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ComparisonOp {
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

fn apply_ordering(op: ComparisonOp, ordering: core::cmp::Ordering) -> bool {
    match op {
        ComparisonOp::Equal => ordering == core::cmp::Ordering::Equal,
        ComparisonOp::NotEqual => ordering != core::cmp::Ordering::Equal,
        ComparisonOp::Less => ordering == core::cmp::Ordering::Less,
        ComparisonOp::LessEqual => ordering != core::cmp::Ordering::Greater,
        ComparisonOp::Greater => ordering == core::cmp::Ordering::Greater,
        ComparisonOp::GreaterEqual => ordering != core::cmp::Ordering::Less,
    }
}

fn compare_f32(op: ComparisonOp, lhs: f32, rhs: f32) -> bool {
    match op {
        ComparisonOp::Equal => lhs == rhs,
        ComparisonOp::NotEqual => lhs != rhs,
        ComparisonOp::Less => lhs < rhs,
        ComparisonOp::LessEqual => lhs <= rhs,
        ComparisonOp::Greater => lhs > rhs,
        ComparisonOp::GreaterEqual => lhs >= rhs,
    }
}

fn compare_f64(op: ComparisonOp, lhs: f64, rhs: f64) -> bool {
    match op {
        ComparisonOp::Equal => lhs == rhs,
        ComparisonOp::NotEqual => lhs != rhs,
        ComparisonOp::Less => lhs < rhs,
        ComparisonOp::LessEqual => lhs <= rhs,
        ComparisonOp::Greater => lhs > rhs,
        ComparisonOp::GreaterEqual => lhs >= rhs,
    }
}

pub(crate) fn compare_dynamic_integer(
    left: &DynamicIntegerValue<'_>,
    right: &DynamicIntegerValue<'_>,
) -> core::cmp::Ordering {
    match (left.negative(), right.negative()) {
        (true, false) => core::cmp::Ordering::Less,
        (false, true) => core::cmp::Ordering::Greater,
        (false, false) => {
            let l_mag = left.magnitude();
            let r_mag = right.magnitude();
            l_mag.len().cmp(&r_mag.len()).then_with(|| l_mag.cmp(r_mag))
        }
        (true, true) => {
            let l_mag = left.magnitude();
            let r_mag = right.magnitude();
            r_mag.len().cmp(&l_mag.len()).then_with(|| r_mag.cmp(l_mag))
        }
    }
}

pub(crate) fn compare_scalar_dynamic(
    op: ComparisonOp,
    left: &Value<'_>,
    right: &Value<'_>,
) -> Option<Result<bool, ComparisonFailure>> {
    match (left, right) {
        // Composites: defer to TASK-EV-013
        (Value::Struct(_), Value::Struct(_)) => None,
        (Value::Enum { .. }, Value::Enum { .. }) => None,

        // Incompatible composite pairs
        (Value::Struct(_), Value::Enum { .. }) | (Value::Enum { .. }, Value::Struct(_)) => {
            Some(Err(ComparisonFailure::DifferentFamily))
        }

        // Composite vs Scalar
        (Value::Struct(_), _)
        | (_, Value::Struct(_))
        | (Value::Enum { .. }, _)
        | (_, Value::Enum { .. }) => Some(Err(ComparisonFailure::DifferentFamily)),

        // Boolean
        (Value::Boolean(l), Value::Boolean(r)) => Some(Ok(apply_ordering(op, l.cmp(r)))),

        // String
        (Value::String(l), Value::String(r)) => Some(Ok(apply_ordering(op, l.cmp(r)))),

        // Fixed Signed Integer
        (Value::Int8(l), Value::Int8(r)) => Some(Ok(apply_ordering(op, l.cmp(r)))),
        (Value::Int16(l), Value::Int16(r)) => Some(Ok(apply_ordering(op, l.cmp(r)))),
        (Value::Int32(l), Value::Int32(r)) => Some(Ok(apply_ordering(op, l.cmp(r)))),
        (Value::Int64(l), Value::Int64(r)) => Some(Ok(apply_ordering(op, l.cmp(r)))),
        (Value::Int128(l), Value::Int128(r)) => Some(Ok(apply_ordering(op, l.cmp(r)))),

        // Fixed Unsigned Integer
        (Value::Uint8(l), Value::Uint8(r)) => Some(Ok(apply_ordering(op, l.cmp(r)))),
        (Value::Uint16(l), Value::Uint16(r)) => Some(Ok(apply_ordering(op, l.cmp(r)))),
        (Value::Uint32(l), Value::Uint32(r)) => Some(Ok(apply_ordering(op, l.cmp(r)))),
        (Value::Uint64(l), Value::Uint64(r)) => Some(Ok(apply_ordering(op, l.cmp(r)))),
        (Value::Uint128(l), Value::Uint128(r)) => Some(Ok(apply_ordering(op, l.cmp(r)))),

        // Float
        (Value::Float32(l), Value::Float32(r)) => Some(Ok(compare_f32(op, *l, *r))),
        (Value::Float64(l), Value::Float64(r)) => Some(Ok(compare_f64(op, *l, *r))),

        // Dynamic
        (Value::Dynamic(l_dyn), Value::Dynamic(r_dyn)) => match (l_dyn, r_dyn) {
            (DynamicValue::Integer(l), DynamicValue::Integer(r)) => {
                Some(Ok(apply_ordering(op, compare_dynamic_integer(l, r))))
            }
            (DynamicValue::Float32(l), DynamicValue::Float32(r)) => {
                Some(Ok(compare_f32(op, *l, *r)))
            }
            (DynamicValue::Float64(l), DynamicValue::Float64(r)) => {
                Some(Ok(compare_f64(op, *l, *r)))
            }
            // Cross-variant within Dynamic
            _ => Some(Err(ComparisonFailure::DifferentFamily)),
        },

        // Any other cross-family combination
        _ => Some(Err(ComparisonFailure::DifferentFamily)),
    }
}

pub(crate) fn equal_scalar_dynamic(
    left: &Value<'_>,
    right: &Value<'_>,
) -> Option<Result<bool, ComparisonFailure>> {
    compare_scalar_dynamic(ComparisonOp::Equal, left, right)
}

pub(crate) fn not_equal_scalar_dynamic(
    left: &Value<'_>,
    right: &Value<'_>,
) -> Option<Result<bool, ComparisonFailure>> {
    compare_scalar_dynamic(ComparisonOp::NotEqual, left, right)
}

pub(crate) fn less_scalar_dynamic(
    left: &Value<'_>,
    right: &Value<'_>,
) -> Option<Result<bool, ComparisonFailure>> {
    compare_scalar_dynamic(ComparisonOp::Less, left, right)
}

pub(crate) fn less_equal_scalar_dynamic(
    left: &Value<'_>,
    right: &Value<'_>,
) -> Option<Result<bool, ComparisonFailure>> {
    compare_scalar_dynamic(ComparisonOp::LessEqual, left, right)
}

pub(crate) fn greater_scalar_dynamic(
    left: &Value<'_>,
    right: &Value<'_>,
) -> Option<Result<bool, ComparisonFailure>> {
    compare_scalar_dynamic(ComparisonOp::Greater, left, right)
}

pub(crate) fn greater_equal_scalar_dynamic(
    left: &Value<'_>,
    right: &Value<'_>,
) -> Option<Result<bool, ComparisonFailure>> {
    compare_scalar_dynamic(ComparisonOp::GreaterEqual, left, right)
}
