use crate::definitions::types::number::Number;

pub fn is_integer(num: Number) -> bool {
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

pub fn is_zero(num: Number) -> bool {
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

pub fn is_same_integer_type(left: Number, right: Number) -> bool {
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
