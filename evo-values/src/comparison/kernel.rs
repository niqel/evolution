use crate::definitions::failures::ComparisonFailure;
use crate::definitions::value::{
    DynamicIntegerValue, DynamicValue, EnumPayload, OwnedDynamicInteger, OwnedDynamicValue,
    OwnedEnumPayload, OwnedValue, Value,
};

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

pub(crate) fn compare_raw_dynamic_integer(
    l_neg: bool,
    l_mag: &[u8],
    r_neg: bool,
    r_mag: &[u8],
) -> core::cmp::Ordering {
    match (l_neg, r_neg) {
        (true, false) => core::cmp::Ordering::Less,
        (false, true) => core::cmp::Ordering::Greater,
        (false, false) => l_mag.len().cmp(&r_mag.len()).then_with(|| l_mag.cmp(r_mag)),
        (true, true) => r_mag.len().cmp(&l_mag.len()).then_with(|| r_mag.cmp(l_mag)),
    }
}

pub(crate) fn compare_dynamic_integer(
    left: &DynamicIntegerValue<'_>,
    right: &DynamicIntegerValue<'_>,
) -> core::cmp::Ordering {
    compare_raw_dynamic_integer(
        left.negative(),
        left.magnitude(),
        right.negative(),
        right.magnitude(),
    )
}

pub(crate) fn compare_owned_dynamic_integer(
    left: &OwnedDynamicInteger,
    right: &OwnedDynamicInteger,
) -> core::cmp::Ordering {
    compare_raw_dynamic_integer(
        left.negative(),
        left.magnitude(),
        right.negative(),
        right.magnitude(),
    )
}

pub(crate) fn compare_scalar_dynamic(
    op: ComparisonOp,
    left: &Value<'_>,
    right: &Value<'_>,
) -> Option<Result<bool, ComparisonFailure>> {
    match (left, right) {
        // Composites: defer to struct/enum handler
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

pub(crate) fn compare_owned_scalar_dynamic(
    op: ComparisonOp,
    left: &OwnedValue,
    right: &OwnedValue,
) -> Option<Result<bool, ComparisonFailure>> {
    match (left, right) {
        // Composites: defer to struct/enum handler
        (OwnedValue::Struct(_), OwnedValue::Struct(_)) => None,
        (OwnedValue::Enum { .. }, OwnedValue::Enum { .. }) => None,

        // Incompatible composite pairs
        (OwnedValue::Struct(_), OwnedValue::Enum { .. })
        | (OwnedValue::Enum { .. }, OwnedValue::Struct(_)) => {
            Some(Err(ComparisonFailure::DifferentFamily))
        }

        // Composite vs Scalar
        (OwnedValue::Struct(_), _)
        | (_, OwnedValue::Struct(_))
        | (OwnedValue::Enum { .. }, _)
        | (_, OwnedValue::Enum { .. }) => Some(Err(ComparisonFailure::DifferentFamily)),

        // Boolean
        (OwnedValue::Boolean(l), OwnedValue::Boolean(r)) => Some(Ok(apply_ordering(op, l.cmp(r)))),

        // String
        (OwnedValue::String(l), OwnedValue::String(r)) => {
            Some(Ok(apply_ordering(op, l.as_ref().cmp(r.as_ref()))))
        }

        // Fixed Signed Integer
        (OwnedValue::Int8(l), OwnedValue::Int8(r)) => Some(Ok(apply_ordering(op, l.cmp(r)))),
        (OwnedValue::Int16(l), OwnedValue::Int16(r)) => Some(Ok(apply_ordering(op, l.cmp(r)))),
        (OwnedValue::Int32(l), OwnedValue::Int32(r)) => Some(Ok(apply_ordering(op, l.cmp(r)))),
        (OwnedValue::Int64(l), OwnedValue::Int64(r)) => Some(Ok(apply_ordering(op, l.cmp(r)))),
        (OwnedValue::Int128(l), OwnedValue::Int128(r)) => Some(Ok(apply_ordering(op, l.cmp(r)))),

        // Fixed Unsigned Integer
        (OwnedValue::Uint8(l), OwnedValue::Uint8(r)) => Some(Ok(apply_ordering(op, l.cmp(r)))),
        (OwnedValue::Uint16(l), OwnedValue::Uint16(r)) => Some(Ok(apply_ordering(op, l.cmp(r)))),
        (OwnedValue::Uint32(l), OwnedValue::Uint32(r)) => Some(Ok(apply_ordering(op, l.cmp(r)))),
        (OwnedValue::Uint64(l), OwnedValue::Uint64(r)) => Some(Ok(apply_ordering(op, l.cmp(r)))),
        (OwnedValue::Uint128(l), OwnedValue::Uint128(r)) => Some(Ok(apply_ordering(op, l.cmp(r)))),

        // Float
        (OwnedValue::Float32(l), OwnedValue::Float32(r)) => Some(Ok(compare_f32(op, *l, *r))),
        (OwnedValue::Float64(l), OwnedValue::Float64(r)) => Some(Ok(compare_f64(op, *l, *r))),

        // Dynamic
        (OwnedValue::Dynamic(l_dyn), OwnedValue::Dynamic(r_dyn)) => match (l_dyn, r_dyn) {
            (OwnedDynamicValue::Integer(l), OwnedDynamicValue::Integer(r)) => {
                Some(Ok(apply_ordering(op, compare_owned_dynamic_integer(l, r))))
            }
            (OwnedDynamicValue::Float32(l), OwnedDynamicValue::Float32(r)) => {
                Some(Ok(compare_f32(op, *l, *r)))
            }
            (OwnedDynamicValue::Float64(l), OwnedDynamicValue::Float64(r)) => {
                Some(Ok(compare_f64(op, *l, *r)))
            }
            // Cross-variant within Dynamic
            _ => Some(Err(ComparisonFailure::DifferentFamily)),
        },

        // Any other cross-family combination
        _ => Some(Err(ComparisonFailure::DifferentFamily)),
    }
}

pub(crate) fn compare_fields(
    op: ComparisonOp,
    left: &[Value<'_>],
    right: &[Value<'_>],
) -> Result<bool, ComparisonFailure> {
    if left.len() != right.len() {
        return Err(ComparisonFailure::NotComparable);
    }
    match op {
        ComparisonOp::Equal => {
            for (l, r) in left.iter().zip(right.iter()) {
                if !compare_value(ComparisonOp::Equal, l, r)? {
                    return Ok(false);
                }
            }
            Ok(true)
        }
        ComparisonOp::NotEqual => {
            for (l, r) in left.iter().zip(right.iter()) {
                if !compare_value(ComparisonOp::Equal, l, r)? {
                    return Ok(true);
                }
            }
            Ok(false)
        }
        ordered_op => {
            for (l, r) in left.iter().zip(right.iter()) {
                if !compare_value(ComparisonOp::Equal, l, r)? {
                    return compare_value(ordered_op, l, r);
                }
            }
            // All fields are Equal
            match ordered_op {
                ComparisonOp::Less | ComparisonOp::Greater => Ok(false),
                ComparisonOp::LessEqual | ComparisonOp::GreaterEqual => Ok(true),
                _ => unreachable!(),
            }
        }
    }
}

pub(crate) fn compare_owned_fields(
    op: ComparisonOp,
    left: &[OwnedValue],
    right: &[OwnedValue],
) -> Result<bool, ComparisonFailure> {
    if left.len() != right.len() {
        return Err(ComparisonFailure::NotComparable);
    }
    match op {
        ComparisonOp::Equal => {
            for (l, r) in left.iter().zip(right.iter()) {
                if !compare_owned_value(ComparisonOp::Equal, l, r)? {
                    return Ok(false);
                }
            }
            Ok(true)
        }
        ComparisonOp::NotEqual => {
            for (l, r) in left.iter().zip(right.iter()) {
                if !compare_owned_value(ComparisonOp::Equal, l, r)? {
                    return Ok(true);
                }
            }
            Ok(false)
        }
        ordered_op => {
            for (l, r) in left.iter().zip(right.iter()) {
                if !compare_owned_value(ComparisonOp::Equal, l, r)? {
                    return compare_owned_value(ordered_op, l, r);
                }
            }
            // All fields are Equal
            match ordered_op {
                ComparisonOp::Less | ComparisonOp::Greater => Ok(false),
                ComparisonOp::LessEqual | ComparisonOp::GreaterEqual => Ok(true),
                _ => unreachable!(),
            }
        }
    }
}

pub(crate) fn compare_enum(
    op: ComparisonOp,
    l_var: usize,
    l_payload: &EnumPayload<'_>,
    r_var: usize,
    r_payload: &EnumPayload<'_>,
) -> Result<bool, ComparisonFailure> {
    if l_var != r_var {
        return Ok(apply_ordering(op, l_var.cmp(&r_var)));
    }
    match (l_payload, r_payload) {
        (EnumPayload::Simple, EnumPayload::Simple) => match op {
            ComparisonOp::Equal | ComparisonOp::LessEqual | ComparisonOp::GreaterEqual => Ok(true),
            ComparisonOp::NotEqual | ComparisonOp::Less | ComparisonOp::Greater => Ok(false),
        },
        (EnumPayload::Associated(l), EnumPayload::Associated(r)) => compare_value(op, l, r),
        (
            EnumPayload::Structured { fields: l_fields },
            EnumPayload::Structured { fields: r_fields },
        ) => compare_fields(op, l_fields, r_fields),
        _ => Err(ComparisonFailure::NotComparable),
    }
}

pub(crate) fn compare_owned_enum(
    op: ComparisonOp,
    l_var: usize,
    l_payload: &OwnedEnumPayload,
    r_var: usize,
    r_payload: &OwnedEnumPayload,
) -> Result<bool, ComparisonFailure> {
    if l_var != r_var {
        return Ok(apply_ordering(op, l_var.cmp(&r_var)));
    }
    match (l_payload, r_payload) {
        (OwnedEnumPayload::Simple, OwnedEnumPayload::Simple) => match op {
            ComparisonOp::Equal | ComparisonOp::LessEqual | ComparisonOp::GreaterEqual => Ok(true),
            ComparisonOp::NotEqual | ComparisonOp::Less | ComparisonOp::Greater => Ok(false),
        },
        (OwnedEnumPayload::Associated(l), OwnedEnumPayload::Associated(r)) => {
            compare_owned_value(op, l, r)
        }
        (
            OwnedEnumPayload::Structured { fields: l_fields },
            OwnedEnumPayload::Structured { fields: r_fields },
        ) => compare_owned_fields(op, l_fields, r_fields),
        _ => Err(ComparisonFailure::NotComparable),
    }
}

pub(crate) fn compare_value(
    op: ComparisonOp,
    left: &Value<'_>,
    right: &Value<'_>,
) -> Result<bool, ComparisonFailure> {
    if let Some(res) = compare_scalar_dynamic(op, left, right) {
        return res;
    }

    match (left, right) {
        (Value::Struct(l_fields), Value::Struct(r_fields)) => {
            compare_fields(op, l_fields, r_fields)
        }
        (
            Value::Enum {
                variant: l_var,
                payload: l_payload,
            },
            Value::Enum {
                variant: r_var,
                payload: r_payload,
            },
        ) => compare_enum(op, *l_var, l_payload, *r_var, r_payload),
        _ => Err(ComparisonFailure::DifferentFamily),
    }
}

pub(crate) fn compare_owned_value(
    op: ComparisonOp,
    left: &OwnedValue,
    right: &OwnedValue,
) -> Result<bool, ComparisonFailure> {
    if let Some(res) = compare_owned_scalar_dynamic(op, left, right) {
        return res;
    }

    match (left, right) {
        (OwnedValue::Struct(l_fields), OwnedValue::Struct(r_fields)) => {
            compare_owned_fields(op, l_fields, r_fields)
        }
        (
            OwnedValue::Enum {
                variant: l_var,
                payload: l_payload,
            },
            OwnedValue::Enum {
                variant: r_var,
                payload: r_payload,
            },
        ) => compare_owned_enum(op, *l_var, l_payload, *r_var, r_payload),
        _ => Err(ComparisonFailure::DifferentFamily),
    }
}

#[cfg(test)]
pub(crate) fn equal_scalar_dynamic(
    left: &Value<'_>,
    right: &Value<'_>,
) -> Option<Result<bool, ComparisonFailure>> {
    compare_scalar_dynamic(ComparisonOp::Equal, left, right)
}

#[cfg(test)]
pub(crate) fn not_equal_scalar_dynamic(
    left: &Value<'_>,
    right: &Value<'_>,
) -> Option<Result<bool, ComparisonFailure>> {
    compare_scalar_dynamic(ComparisonOp::NotEqual, left, right)
}

#[cfg(test)]
pub(crate) fn less_scalar_dynamic(
    left: &Value<'_>,
    right: &Value<'_>,
) -> Option<Result<bool, ComparisonFailure>> {
    compare_scalar_dynamic(ComparisonOp::Less, left, right)
}

#[cfg(test)]
pub(crate) fn less_equal_scalar_dynamic(
    left: &Value<'_>,
    right: &Value<'_>,
) -> Option<Result<bool, ComparisonFailure>> {
    compare_scalar_dynamic(ComparisonOp::LessEqual, left, right)
}

#[cfg(test)]
pub(crate) fn greater_scalar_dynamic(
    left: &Value<'_>,
    right: &Value<'_>,
) -> Option<Result<bool, ComparisonFailure>> {
    compare_scalar_dynamic(ComparisonOp::Greater, left, right)
}

#[cfg(test)]
pub(crate) fn greater_equal_scalar_dynamic(
    left: &Value<'_>,
    right: &Value<'_>,
) -> Option<Result<bool, ComparisonFailure>> {
    compare_scalar_dynamic(ComparisonOp::GreaterEqual, left, right)
}
