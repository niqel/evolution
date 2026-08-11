use crate::definitions::types::number::Number;

pub fn to_f32(num: Number) -> Option<f32> {
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

pub fn to_f64(num: Number) -> Option<f64> {
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
