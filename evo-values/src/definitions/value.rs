use alloc::borrow::Cow;
use alloc::boxed::Box;

#[derive(Clone)]
pub enum Value<'value> {
    Boolean(bool),

    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),

    Uint8(u8),
    Uint16(u16),
    Uint32(u32),
    Uint64(u64),
    Uint128(u128),

    Float32(f32),
    Float64(f64),

    String(&'value str),

    Dynamic(DynamicValue<'value>),

    Struct(Box<[Value<'value>]>),

    Enum {
        variant: usize,
        payload: EnumPayload<'value>,
    },
}

#[derive(Clone)]
pub enum DynamicValue<'value> {
    Integer(DynamicIntegerValue<'value>),
    Float32(f32),
    Float64(f64),
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct DynamicIntegerValue<'value> {
    negative: bool,
    magnitude: Cow<'value, [u8]>,
}

#[derive(Clone)]
pub enum EnumPayload<'value> {
    Simple,
    Associated(Box<Value<'value>>),
    Structured { fields: Box<[Value<'value>]> },
}

#[derive(Clone)]
pub enum OwnedValue {
    Boolean(bool),

    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),

    Uint8(u8),
    Uint16(u16),
    Uint32(u32),
    Uint64(u64),
    Uint128(u128),

    Float32(f32),
    Float64(f64),

    String(Box<str>),

    Dynamic(OwnedDynamicValue),

    Struct(Box<[OwnedValue]>),

    Enum {
        variant: usize,
        payload: OwnedEnumPayload,
    },
}

#[derive(Clone)]
pub enum OwnedDynamicValue {
    Integer(OwnedDynamicInteger),
    Float32(f32),
    Float64(f64),
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct OwnedDynamicInteger {
    negative: bool,
    magnitude: Box<[u8]>,
}

#[derive(Clone)]
pub enum OwnedEnumPayload {
    Simple,
    Associated(Box<OwnedValue>),
    Structured { fields: Box<[OwnedValue]> },
}
