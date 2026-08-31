use alloc::borrow::Cow;
use alloc::boxed::Box;

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum DynamicValue<'value> {
    Integer(DynamicIntegerValue<'value>),
    Float32(f32),
    Float64(f64),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DynamicIntegerValue<'value> {
    pub negative: bool,
    pub magnitude: Cow<'value, [u8]>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EnumPayload<'value> {
    Simple,
    Associated(Box<Value<'value>>),
    Structured { fields: Box<[Value<'value>]> },
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum OwnedDynamicValue {
    Integer(OwnedDynamicInteger),
    Float32(f32),
    Float64(f64),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedDynamicInteger {
    pub negative: bool,
    pub magnitude: Box<[u8]>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OwnedEnumPayload {
    Simple,
    Associated(Box<OwnedValue>),
    Structured { fields: Box<[OwnedValue]> },
}
