#[macro_export]
macro_rules! impl_int_op {
    ($left:expr, $right:expr, $op:ident) => {
        match ($left, $right) {
            (
                $crate::definitions::types::number::Number::I8(a),
                $crate::definitions::types::number::Number::I8(b),
            ) => a.$op(b).map($crate::definitions::types::number::Number::I8),
            (
                $crate::definitions::types::number::Number::I16(a),
                $crate::definitions::types::number::Number::I16(b),
            ) => a
                .$op(b)
                .map($crate::definitions::types::number::Number::I16),
            (
                $crate::definitions::types::number::Number::I32(a),
                $crate::definitions::types::number::Number::I32(b),
            ) => a
                .$op(b)
                .map($crate::definitions::types::number::Number::I32),
            (
                $crate::definitions::types::number::Number::I64(a),
                $crate::definitions::types::number::Number::I64(b),
            ) => a
                .$op(b)
                .map($crate::definitions::types::number::Number::I64),
            (
                $crate::definitions::types::number::Number::I128(a),
                $crate::definitions::types::number::Number::I128(b),
            ) => a
                .$op(b)
                .map($crate::definitions::types::number::Number::I128),
            (
                $crate::definitions::types::number::Number::U8(a),
                $crate::definitions::types::number::Number::U8(b),
            ) => a.$op(b).map($crate::definitions::types::number::Number::U8),
            (
                $crate::definitions::types::number::Number::U16(a),
                $crate::definitions::types::number::Number::U16(b),
            ) => a
                .$op(b)
                .map($crate::definitions::types::number::Number::U16),
            (
                $crate::definitions::types::number::Number::U32(a),
                $crate::definitions::types::number::Number::U32(b),
            ) => a
                .$op(b)
                .map($crate::definitions::types::number::Number::U32),
            (
                $crate::definitions::types::number::Number::U64(a),
                $crate::definitions::types::number::Number::U64(b),
            ) => a
                .$op(b)
                .map($crate::definitions::types::number::Number::U64),
            (
                $crate::definitions::types::number::Number::U128(a),
                $crate::definitions::types::number::Number::U128(b),
            ) => a
                .$op(b)
                .map($crate::definitions::types::number::Number::U128),
            _ => None,
        }
    };
}
