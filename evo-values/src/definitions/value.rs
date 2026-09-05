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
pub struct DynamicIntegerValue<'value> {
    negative: bool,
    magnitude: Cow<'value, [u8]>,
}

impl<'value> DynamicIntegerValue<'value> {
    pub fn from_parts(negative: bool, magnitude: Cow<'value, [u8]>) -> Self {
        let leading_zeros = magnitude
            .iter()
            .position(|&b| b != 0)
            .unwrap_or(magnitude.len());
        if leading_zeros == magnitude.len() {
            Self {
                negative: false,
                magnitude: Cow::Borrowed(&[]),
            }
        } else if leading_zeros == 0 {
            Self {
                negative,
                magnitude,
            }
        } else {
            let trimmed = match magnitude {
                Cow::Borrowed(slice) => Cow::Borrowed(&slice[leading_zeros..]),
                Cow::Owned(mut vec) => {
                    vec.drain(..leading_zeros);
                    Cow::Owned(vec)
                }
            };
            Self {
                negative,
                magnitude: trimmed,
            }
        }
    }

    pub fn negative(&self) -> bool {
        self.negative
    }

    pub fn magnitude(&self) -> &[u8] {
        &self.magnitude
    }
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
pub struct OwnedDynamicInteger {
    negative: bool,
    magnitude: Box<[u8]>,
}

impl OwnedDynamicInteger {
    pub fn from_parts(negative: bool, magnitude: Box<[u8]>) -> Self {
        let leading_zeros = magnitude
            .iter()
            .position(|&b| b != 0)
            .unwrap_or(magnitude.len());
        if leading_zeros == magnitude.len() {
            Self {
                negative: false,
                magnitude: Box::new([]),
            }
        } else if leading_zeros == 0 {
            Self {
                negative,
                magnitude,
            }
        } else {
            Self {
                negative,
                magnitude: Box::from(&magnitude[leading_zeros..]),
            }
        }
    }

    pub fn negative(&self) -> bool {
        self.negative
    }

    pub fn magnitude(&self) -> &[u8] {
        &self.magnitude
    }
}

#[derive(Clone)]
pub enum OwnedEnumPayload {
    Simple,
    Associated(Box<OwnedValue>),
    Structured { fields: Box<[OwnedValue]> },
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::borrow::Cow;

    #[test]
    fn borrowed_preserves_borrow_after_stripping_leading_zeros() {
        let source: [u8; 4] = [0x00, 0x00, 0x01, 0xFF];
        let val = DynamicIntegerValue::from_parts(false, Cow::Borrowed(&source));
        assert!(!val.negative());
        assert_eq!(val.magnitude(), &[0x01, 0xFF]);
        match val.magnitude {
            Cow::Borrowed(slice) => {
                assert_eq!(slice.as_ptr(), source[2..].as_ptr());
            }
            Cow::Owned(_) => panic!("expected Cow::Borrowed"),
        }
    }

    #[test]
    fn borrowed_zero_preserves_borrow() {
        let source: [u8; 2] = [0x00, 0x00];
        let val = DynamicIntegerValue::from_parts(true, Cow::Borrowed(&source));
        assert!(!val.negative());
        assert_eq!(val.magnitude(), &[]);
        assert!(matches!(val.magnitude, Cow::Borrowed(_)));
    }
}
