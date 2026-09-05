use evo_values::BitwiseFailure;
use evo_values::bitwise::*;
use evo_values::definitions::bitwise::{BitAnd, BitNot, BitOr, BitXor, ShiftLeft, ShiftRight};
use evo_values::definitions::scalars::ShiftAmount;

// ============================================================================
// 1. BitAnd tests for all 10 integer families
// ============================================================================

macro_rules! test_bit_and_signed {
    ($test_name:ident, $fn_name:ident, $const_name:ident, $t:ty) => {
        #[test]
        fn $test_name() {
            // Zero
            assert_eq!($fn_name(0, 0), 0);
            assert_eq!($fn_name(42, 0), 0);
            assert_eq!($fn_name(0, 42), 0);

            // All bits / mask
            assert_eq!($fn_name(<$t>::MAX, <$t>::MAX), <$t>::MAX);
            assert_eq!($fn_name(42, -1), 42); // -1 has all bits set

            // Pattern normal
            assert_eq!($fn_name(0b1100, 0b1010), 0b1000);

            // Negative pattern (signed)
            assert_eq!($fn_name(-1, 0b0101_0101), 0b0101_0101);
            assert_eq!($fn_name(-2, 1), 0); // -2 has lowest bit 0

            // Function pointer binding
            let op: BitAnd<$t> = $const_name;
            assert_eq!(op(0b1111, 0b0101), 0b0101);
        }
    };
}

test_bit_and_signed!(test_bit_and_i8, bit_and_i8, BIT_AND_I8, i8);
test_bit_and_signed!(test_bit_and_i16, bit_and_i16, BIT_AND_I16, i16);
test_bit_and_signed!(test_bit_and_i32, bit_and_i32, BIT_AND_I32, i32);
test_bit_and_signed!(test_bit_and_i64, bit_and_i64, BIT_AND_I64, i64);
test_bit_and_signed!(test_bit_and_i128, bit_and_i128, BIT_AND_I128, i128);

macro_rules! test_bit_and_unsigned {
    ($test_name:ident, $fn_name:ident, $const_name:ident, $t:ty) => {
        #[test]
        fn $test_name() {
            // Zero
            assert_eq!($fn_name(0, 0), 0);
            assert_eq!($fn_name(42, 0), 0);
            assert_eq!($fn_name(0, 42), 0);

            // All bits / mask
            assert_eq!($fn_name(<$t>::MAX, <$t>::MAX), <$t>::MAX);
            assert_eq!($fn_name(42, <$t>::MAX), 42);

            // Pattern normal
            assert_eq!($fn_name(0b1100, 0b1010), 0b1000);

            // Function pointer binding
            let op: BitAnd<$t> = $const_name;
            assert_eq!(op(0b1111, 0b0101), 0b0101);
        }
    };
}

test_bit_and_unsigned!(test_bit_and_u8, bit_and_u8, BIT_AND_U8, u8);
test_bit_and_unsigned!(test_bit_and_u16, bit_and_u16, BIT_AND_U16, u16);
test_bit_and_unsigned!(test_bit_and_u32, bit_and_u32, BIT_AND_U32, u32);
test_bit_and_unsigned!(test_bit_and_u64, bit_and_u64, BIT_AND_U64, u64);
test_bit_and_unsigned!(test_bit_and_u128, bit_and_u128, BIT_AND_U128, u128);

// ============================================================================
// 2. BitOr tests for all 10 integer families
// ============================================================================

macro_rules! test_bit_or_signed {
    ($test_name:ident, $fn_name:ident, $const_name:ident, $t:ty) => {
        #[test]
        fn $test_name() {
            // Zero
            assert_eq!($fn_name(0, 0), 0);
            assert_eq!($fn_name(42, 0), 42);
            assert_eq!($fn_name(0, 42), 42);

            // Pattern normal
            assert_eq!($fn_name(0b1100, 0b1010), 0b1110);

            // All bits
            assert_eq!($fn_name(0, -1), -1);
            assert_eq!($fn_name(42, -1), -1);

            // Function pointer binding
            let op: BitOr<$t> = $const_name;
            assert_eq!(op(0b1100, 0b0011), 0b1111);
        }
    };
}

test_bit_or_signed!(test_bit_or_i8, bit_or_i8, BIT_OR_I8, i8);
test_bit_or_signed!(test_bit_or_i16, bit_or_i16, BIT_OR_I16, i16);
test_bit_or_signed!(test_bit_or_i32, bit_or_i32, BIT_OR_I32, i32);
test_bit_or_signed!(test_bit_or_i64, bit_or_i64, BIT_OR_I64, i64);
test_bit_or_signed!(test_bit_or_i128, bit_or_i128, BIT_OR_I128, i128);

macro_rules! test_bit_or_unsigned {
    ($test_name:ident, $fn_name:ident, $const_name:ident, $t:ty) => {
        #[test]
        fn $test_name() {
            // Zero
            assert_eq!($fn_name(0, 0), 0);
            assert_eq!($fn_name(42, 0), 42);
            assert_eq!($fn_name(0, 42), 42);

            // Pattern normal
            assert_eq!($fn_name(0b1100, 0b1010), 0b1110);

            // All bits
            assert_eq!($fn_name(0, <$t>::MAX), <$t>::MAX);
            assert_eq!($fn_name(42, <$t>::MAX), <$t>::MAX);

            // Function pointer binding
            let op: BitOr<$t> = $const_name;
            assert_eq!(op(0b1100, 0b0011), 0b1111);
        }
    };
}

test_bit_or_unsigned!(test_bit_or_u8, bit_or_u8, BIT_OR_U8, u8);
test_bit_or_unsigned!(test_bit_or_u16, bit_or_u16, BIT_OR_U16, u16);
test_bit_or_unsigned!(test_bit_or_u32, bit_or_u32, BIT_OR_U32, u32);
test_bit_or_unsigned!(test_bit_or_u64, bit_or_u64, BIT_OR_U64, u64);
test_bit_or_unsigned!(test_bit_or_u128, bit_or_u128, BIT_OR_U128, u128);

// ============================================================================
// 3. BitXor tests for all 10 integer families
// ============================================================================

macro_rules! test_bit_xor {
    ($test_name:ident, $fn_name:ident, $const_name:ident, $t:ty) => {
        #[test]
        fn $test_name() {
            // value ^ value -> 0
            assert_eq!($fn_name(0, 0), 0);
            assert_eq!($fn_name(42, 42), 0);
            assert_eq!($fn_name(<$t>::MAX, <$t>::MAX), 0);

            // value ^ 0 -> value
            assert_eq!($fn_name(42, 0), 42);
            assert_eq!($fn_name(0, 42), 42);

            // Pattern normal
            assert_eq!($fn_name(0b1100, 0b1010), 0b0110);

            // Function pointer binding
            let op: BitXor<$t> = $const_name;
            assert_eq!(op(0b1111, 0b0101), 0b1010);
        }
    };
}

test_bit_xor!(test_bit_xor_i8, bit_xor_i8, BIT_XOR_I8, i8);
test_bit_xor!(test_bit_xor_i16, bit_xor_i16, BIT_XOR_I16, i16);
test_bit_xor!(test_bit_xor_i32, bit_xor_i32, BIT_XOR_I32, i32);
test_bit_xor!(test_bit_xor_i64, bit_xor_i64, BIT_XOR_I64, i64);
test_bit_xor!(test_bit_xor_i128, bit_xor_i128, BIT_XOR_I128, i128);

test_bit_xor!(test_bit_xor_u8, bit_xor_u8, BIT_XOR_U8, u8);
test_bit_xor!(test_bit_xor_u16, bit_xor_u16, BIT_XOR_U16, u16);
test_bit_xor!(test_bit_xor_u32, bit_xor_u32, BIT_XOR_U32, u32);
test_bit_xor!(test_bit_xor_u64, bit_xor_u64, BIT_XOR_U64, u64);
test_bit_xor!(test_bit_xor_u128, bit_xor_u128, BIT_XOR_U128, u128);

// ============================================================================
// 4. BitNot tests for all 10 integer families
// ============================================================================

macro_rules! test_bit_not_signed {
    ($test_name:ident, $fn_name:ident, $const_name:ident, $t:ty) => {
        #[test]
        fn $test_name() {
            // !0 -> -1
            assert_eq!($fn_name(0), -1);

            // !-1 -> 0
            assert_eq!($fn_name(-1), 0);

            // Pattern normal
            assert_eq!($fn_name(1), -2);

            // Function pointer binding
            let op: BitNot<$t> = $const_name;
            assert_eq!(op(0), -1);
            assert_eq!(op(-1), 0);
        }
    };
}

test_bit_not_signed!(test_bit_not_i8, bit_not_i8, BIT_NOT_I8, i8);
test_bit_not_signed!(test_bit_not_i16, bit_not_i16, BIT_NOT_I16, i16);
test_bit_not_signed!(test_bit_not_i32, bit_not_i32, BIT_NOT_I32, i32);
test_bit_not_signed!(test_bit_not_i64, bit_not_i64, BIT_NOT_I64, i64);
test_bit_not_signed!(test_bit_not_i128, bit_not_i128, BIT_NOT_I128, i128);

macro_rules! test_bit_not_unsigned {
    ($test_name:ident, $fn_name:ident, $const_name:ident, $t:ty) => {
        #[test]
        fn $test_name() {
            // !0 -> MAX
            assert_eq!($fn_name(0), <$t>::MAX);

            // !MAX -> 0
            assert_eq!($fn_name(<$t>::MAX), 0);

            // Pattern normal
            assert_eq!($fn_name(1), <$t>::MAX - 1);

            // Function pointer binding
            let op: BitNot<$t> = $const_name;
            assert_eq!(op(0), <$t>::MAX);
            assert_eq!(op(<$t>::MAX), 0);
        }
    };
}

test_bit_not_unsigned!(test_bit_not_u8, bit_not_u8, BIT_NOT_U8, u8);
test_bit_not_unsigned!(test_bit_not_u16, bit_not_u16, BIT_NOT_U16, u16);
test_bit_not_unsigned!(test_bit_not_u32, bit_not_u32, BIT_NOT_U32, u32);
test_bit_not_unsigned!(test_bit_not_u64, bit_not_u64, BIT_NOT_U64, u64);
test_bit_not_unsigned!(test_bit_not_u128, bit_not_u128, BIT_NOT_U128, u128);

// ============================================================================
// 5. ShiftLeft tests for all 10 integer families
// ============================================================================

macro_rules! test_shift_left {
    ($test_name:ident, $fn_name:ident, $const_name:ident, $t:ty, $width:expr) => {
        #[test]
        fn $test_name() {
            // shift 0
            assert_eq!($fn_name(42, ShiftAmount(0)), Ok(42));

            // shift 1
            assert_eq!($fn_name(1, ShiftAmount(1)), Ok(2));

            // shift width - 1
            let expected_max_shift = (1 as $t) << ($width - 1);
            assert_eq!($fn_name(1, ShiftAmount($width - 1)), Ok(expected_max_shift));

            // shift width -> InvalidShift
            assert_eq!(
                $fn_name(1, ShiftAmount($width)),
                Err(BitwiseFailure::InvalidShift)
            );

            // shift > width -> InvalidShift
            assert_eq!(
                $fn_name(1, ShiftAmount($width + 1)),
                Err(BitwiseFailure::InvalidShift)
            );
            assert_eq!(
                $fn_name(1, ShiftAmount($width + 100)),
                Err(BitwiseFailure::InvalidShift)
            );

            // upper bits discarded without error
            let top_and_bottom = expected_max_shift | (1 as $t);
            assert_eq!($fn_name(top_and_bottom, ShiftAmount(1)), Ok(2 as $t));

            // Function pointer binding
            let op: ShiftLeft<$t> = $const_name;
            assert_eq!(op(1, ShiftAmount(2)), Ok(4));
        }
    };
}

test_shift_left!(test_shift_left_i8, shift_left_i8, SHIFT_LEFT_I8, i8, 8);
test_shift_left!(test_shift_left_i16, shift_left_i16, SHIFT_LEFT_I16, i16, 16);
test_shift_left!(test_shift_left_i32, shift_left_i32, SHIFT_LEFT_I32, i32, 32);
test_shift_left!(test_shift_left_i64, shift_left_i64, SHIFT_LEFT_I64, i64, 64);
test_shift_left!(
    test_shift_left_i128,
    shift_left_i128,
    SHIFT_LEFT_I128,
    i128,
    128
);

test_shift_left!(test_shift_left_u8, shift_left_u8, SHIFT_LEFT_U8, u8, 8);
test_shift_left!(test_shift_left_u16, shift_left_u16, SHIFT_LEFT_U16, u16, 16);
test_shift_left!(test_shift_left_u32, shift_left_u32, SHIFT_LEFT_U32, u32, 32);
test_shift_left!(test_shift_left_u64, shift_left_u64, SHIFT_LEFT_U64, u64, 64);
test_shift_left!(
    test_shift_left_u128,
    shift_left_u128,
    SHIFT_LEFT_U128,
    u128,
    128
);

// ============================================================================
// 6. ShiftRight tests for all 10 integer families
// ============================================================================

macro_rules! test_shift_right_unsigned {
    ($test_name:ident, $fn_name:ident, $const_name:ident, $t:ty, $width:expr) => {
        #[test]
        fn $test_name() {
            // shift 0
            assert_eq!($fn_name(42, ShiftAmount(0)), Ok(42));

            // shift 1
            assert_eq!($fn_name(2, ShiftAmount(1)), Ok(1));

            // shift width - 1
            let top_bit = (1 as $t) << ($width - 1);
            assert_eq!($fn_name(top_bit, ShiftAmount($width - 1)), Ok(1 as $t));

            // shift width -> InvalidShift
            assert_eq!(
                $fn_name(top_bit, ShiftAmount($width)),
                Err(BitwiseFailure::InvalidShift)
            );

            // shift > width -> InvalidShift
            assert_eq!(
                $fn_name(top_bit, ShiftAmount($width + 1)),
                Err(BitwiseFailure::InvalidShift)
            );
            assert_eq!(
                $fn_name(top_bit, ShiftAmount($width + 100)),
                Err(BitwiseFailure::InvalidShift)
            );

            // Function pointer binding
            let op: ShiftRight<$t> = $const_name;
            assert_eq!(op(8, ShiftAmount(2)), Ok(2));
        }
    };
}

test_shift_right_unsigned!(test_shift_right_u8, shift_right_u8, SHIFT_RIGHT_U8, u8, 8);
test_shift_right_unsigned!(
    test_shift_right_u16,
    shift_right_u16,
    SHIFT_RIGHT_U16,
    u16,
    16
);
test_shift_right_unsigned!(
    test_shift_right_u32,
    shift_right_u32,
    SHIFT_RIGHT_U32,
    u32,
    32
);
test_shift_right_unsigned!(
    test_shift_right_u64,
    shift_right_u64,
    SHIFT_RIGHT_U64,
    u64,
    64
);
test_shift_right_unsigned!(
    test_shift_right_u128,
    shift_right_u128,
    SHIFT_RIGHT_U128,
    u128,
    128
);

macro_rules! test_shift_right_signed {
    ($test_name:ident, $fn_name:ident, $const_name:ident, $t:ty, $width:expr) => {
        #[test]
        fn $test_name() {
            // shift 0
            assert_eq!($fn_name(42, ShiftAmount(0)), Ok(42));
            assert_eq!($fn_name(-42, ShiftAmount(0)), Ok(-42));

            // shift 1
            assert_eq!($fn_name(2, ShiftAmount(1)), Ok(1));

            // shift width - 1
            assert_eq!($fn_name(1, ShiftAmount($width - 1)), Ok(0));

            // shift width -> InvalidShift
            assert_eq!(
                $fn_name(1, ShiftAmount($width)),
                Err(BitwiseFailure::InvalidShift)
            );

            // shift > width -> InvalidShift
            assert_eq!(
                $fn_name(1, ShiftAmount($width + 1)),
                Err(BitwiseFailure::InvalidShift)
            );
            assert_eq!(
                $fn_name(1, ShiftAmount($width + 100)),
                Err(BitwiseFailure::InvalidShift)
            );

            // Function pointer binding
            let op: ShiftRight<$t> = $const_name;
            assert_eq!(op(8, ShiftAmount(2)), Ok(2));
        }
    };
}

test_shift_right_signed!(test_shift_right_i8, shift_right_i8, SHIFT_RIGHT_I8, i8, 8);
test_shift_right_signed!(
    test_shift_right_i16,
    shift_right_i16,
    SHIFT_RIGHT_I16,
    i16,
    16
);
test_shift_right_signed!(
    test_shift_right_i32,
    shift_right_i32,
    SHIFT_RIGHT_I32,
    i32,
    32
);
test_shift_right_signed!(
    test_shift_right_i64,
    shift_right_i64,
    SHIFT_RIGHT_I64,
    i64,
    64
);
test_shift_right_signed!(
    test_shift_right_i128,
    shift_right_i128,
    SHIFT_RIGHT_I128,
    i128,
    128
);

// ============================================================================
// 7. Signed arithmetic right shift (sign extension) for all 5 signed families
// ============================================================================

macro_rules! test_signed_arithmetic_right_shift {
    ($test_name:ident, $fn_name:ident, $t:ty, $width:expr) => {
        #[test]
        fn $test_name() {
            // -2 >> 1 -> -1
            assert_eq!($fn_name(-2, ShiftAmount(1)), Ok(-1));

            // -4 >> 1 -> -2
            assert_eq!($fn_name(-4, ShiftAmount(1)), Ok(-2));

            // -1 >> n -> -1 for small and large shifts
            assert_eq!($fn_name(-1, ShiftAmount(1)), Ok(-1));
            assert_eq!($fn_name(-1, ShiftAmount($width - 1)), Ok(-1));

            // Sign bit extension on MIN
            assert_eq!($fn_name(<$t>::MIN, ShiftAmount(1)), Ok(<$t>::MIN / 2));
            assert_eq!($fn_name(<$t>::MIN, ShiftAmount($width - 1)), Ok(-1));
        }
    };
}

test_signed_arithmetic_right_shift!(test_sign_extension_i8, shift_right_i8, i8, 8);
test_signed_arithmetic_right_shift!(test_sign_extension_i16, shift_right_i16, i16, 16);
test_signed_arithmetic_right_shift!(test_sign_extension_i32, shift_right_i32, i32, 32);
test_signed_arithmetic_right_shift!(test_sign_extension_i64, shift_right_i64, i64, 64);
test_signed_arithmetic_right_shift!(test_sign_extension_i128, shift_right_i128, i128, 128);

// ============================================================================
// 8. Function pointer contracts across all 6 operations and 10 families
// ============================================================================

#[test]
fn test_all_bitwise_function_pointer_contracts() {
    let _op_and_i8: BitAnd<i8> = BIT_AND_I8;
    let _op_and_i16: BitAnd<i16> = BIT_AND_I16;
    let _op_and_i32: BitAnd<i32> = BIT_AND_I32;
    let _op_and_i64: BitAnd<i64> = BIT_AND_I64;
    let _op_and_i128: BitAnd<i128> = BIT_AND_I128;
    let _op_and_u8: BitAnd<u8> = BIT_AND_U8;
    let _op_and_u16: BitAnd<u16> = BIT_AND_U16;
    let _op_and_u32: BitAnd<u32> = BIT_AND_U32;
    let _op_and_u64: BitAnd<u64> = BIT_AND_U64;
    let _op_and_u128: BitAnd<u128> = BIT_AND_U128;

    let _op_or_i8: BitOr<i8> = BIT_OR_I8;
    let _op_or_i16: BitOr<i16> = BIT_OR_I16;
    let _op_or_i32: BitOr<i32> = BIT_OR_I32;
    let _op_or_i64: BitOr<i64> = BIT_OR_I64;
    let _op_or_i128: BitOr<i128> = BIT_OR_I128;
    let _op_or_u8: BitOr<u8> = BIT_OR_U8;
    let _op_or_u16: BitOr<u16> = BIT_OR_U16;
    let _op_or_u32: BitOr<u32> = BIT_OR_U32;
    let _op_or_u64: BitOr<u64> = BIT_OR_U64;
    let _op_or_u128: BitOr<u128> = BIT_OR_U128;

    let _op_xor_i8: BitXor<i8> = BIT_XOR_I8;
    let _op_xor_i16: BitXor<i16> = BIT_XOR_I16;
    let _op_xor_i32: BitXor<i32> = BIT_XOR_I32;
    let _op_xor_i64: BitXor<i64> = BIT_XOR_I64;
    let _op_xor_i128: BitXor<i128> = BIT_XOR_I128;
    let _op_xor_u8: BitXor<u8> = BIT_XOR_U8;
    let _op_xor_u16: BitXor<u16> = BIT_XOR_U16;
    let _op_xor_u32: BitXor<u32> = BIT_XOR_U32;
    let _op_xor_u64: BitXor<u64> = BIT_XOR_U64;
    let _op_xor_u128: BitXor<u128> = BIT_XOR_U128;

    let _op_not_i8: BitNot<i8> = BIT_NOT_I8;
    let _op_not_i16: BitNot<i16> = BIT_NOT_I16;
    let _op_not_i32: BitNot<i32> = BIT_NOT_I32;
    let _op_not_i64: BitNot<i64> = BIT_NOT_I64;
    let _op_not_i128: BitNot<i128> = BIT_NOT_I128;
    let _op_not_u8: BitNot<u8> = BIT_NOT_U8;
    let _op_not_u16: BitNot<u16> = BIT_NOT_U16;
    let _op_not_u32: BitNot<u32> = BIT_NOT_U32;
    let _op_not_u64: BitNot<u64> = BIT_NOT_U64;
    let _op_not_u128: BitNot<u128> = BIT_NOT_U128;

    let _op_shl_i8: ShiftLeft<i8> = SHIFT_LEFT_I8;
    let _op_shl_i16: ShiftLeft<i16> = SHIFT_LEFT_I16;
    let _op_shl_i32: ShiftLeft<i32> = SHIFT_LEFT_I32;
    let _op_shl_i64: ShiftLeft<i64> = SHIFT_LEFT_I64;
    let _op_shl_i128: ShiftLeft<i128> = SHIFT_LEFT_I128;
    let _op_shl_u8: ShiftLeft<u8> = SHIFT_LEFT_U8;
    let _op_shl_u16: ShiftLeft<u16> = SHIFT_LEFT_U16;
    let _op_shl_u32: ShiftLeft<u32> = SHIFT_LEFT_U32;
    let _op_shl_u64: ShiftLeft<u64> = SHIFT_LEFT_U64;
    let _op_shl_u128: ShiftLeft<u128> = SHIFT_LEFT_U128;

    let _op_shr_i8: ShiftRight<i8> = SHIFT_RIGHT_I8;
    let _op_shr_i16: ShiftRight<i16> = SHIFT_RIGHT_I16;
    let _op_shr_i32: ShiftRight<i32> = SHIFT_RIGHT_I32;
    let _op_shr_i64: ShiftRight<i64> = SHIFT_RIGHT_I64;
    let _op_shr_i128: ShiftRight<i128> = SHIFT_RIGHT_I128;
    let _op_shr_u8: ShiftRight<u8> = SHIFT_RIGHT_U8;
    let _op_shr_u16: ShiftRight<u16> = SHIFT_RIGHT_U16;
    let _op_shr_u32: ShiftRight<u32> = SHIFT_RIGHT_U32;
    let _op_shr_u64: ShiftRight<u64> = SHIFT_RIGHT_U64;
    let _op_shr_u128: ShiftRight<u128> = SHIFT_RIGHT_U128;
}
