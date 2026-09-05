use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::definitions::materialize_owned::MaterializeOwned;
use crate::definitions::value::{
    DynamicValue, EnumPayload, OwnedDynamicInteger, OwnedDynamicValue, OwnedEnumPayload,
    OwnedValue, Value,
};

pub fn materialize_owned(value: &Value<'_>) -> OwnedValue {
    match value {
        Value::Boolean(b) => OwnedValue::Boolean(*b),

        Value::Int8(i) => OwnedValue::Int8(*i),
        Value::Int16(i) => OwnedValue::Int16(*i),
        Value::Int32(i) => OwnedValue::Int32(*i),
        Value::Int64(i) => OwnedValue::Int64(*i),
        Value::Int128(i) => OwnedValue::Int128(*i),

        Value::Uint8(u) => OwnedValue::Uint8(*u),
        Value::Uint16(u) => OwnedValue::Uint16(*u),
        Value::Uint32(u) => OwnedValue::Uint32(*u),
        Value::Uint64(u) => OwnedValue::Uint64(*u),
        Value::Uint128(u) => OwnedValue::Uint128(*u),

        Value::Float32(f) => OwnedValue::Float32(*f),
        Value::Float64(f) => OwnedValue::Float64(*f),

        Value::String(s) => OwnedValue::String(Box::from(*s)),

        Value::Dynamic(dyn_val) => {
            let owned_dyn = match dyn_val {
                DynamicValue::Integer(int_val) => {
                    let negative = int_val.negative();
                    let magnitude = Box::from(int_val.magnitude());
                    OwnedDynamicValue::Integer(OwnedDynamicInteger::from_parts(negative, magnitude))
                }
                DynamicValue::Float32(f) => OwnedDynamicValue::Float32(*f),
                DynamicValue::Float64(f) => OwnedDynamicValue::Float64(*f),
            };
            OwnedValue::Dynamic(owned_dyn)
        }

        Value::Struct(fields) => {
            let owned_fields: Box<[OwnedValue]> = fields
                .iter()
                .map(materialize_owned)
                .collect::<Vec<OwnedValue>>()
                .into_boxed_slice();
            OwnedValue::Struct(owned_fields)
        }

        Value::Enum { variant, payload } => {
            let owned_payload = match payload {
                EnumPayload::Simple => OwnedEnumPayload::Simple,
                EnumPayload::Associated(inner) => {
                    OwnedEnumPayload::Associated(Box::new(materialize_owned(inner)))
                }
                EnumPayload::Structured { fields } => {
                    let owned_fields: Box<[OwnedValue]> = fields
                        .iter()
                        .map(materialize_owned)
                        .collect::<Vec<OwnedValue>>()
                        .into_boxed_slice();
                    OwnedEnumPayload::Structured {
                        fields: owned_fields,
                    }
                }
            };
            OwnedValue::Enum {
                variant: *variant,
                payload: owned_payload,
            }
        }
    }
}

pub const MATERIALIZE_OWNED: MaterializeOwned = materialize_owned;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn materialize_owned_function_pointer_binding() {
        let op: MaterializeOwned = MATERIALIZE_OWNED;
        let borrowed = Value::Int32(42);
        let owned = op(&borrowed);
        assert!(matches!(owned, OwnedValue::Int32(42)));
    }
}
