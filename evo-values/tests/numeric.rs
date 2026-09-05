use evo_values::NumericFailure;
use evo_values::definitions::numeric::{
    Abs, Add, Divide, FloatAbs, FloatAdd, FloatCeil, FloatClamp, FloatDivide, FloatFloor,
    FloatFract, FloatIsFinite, FloatIsInfinite, FloatIsNan, FloatMax, FloatMin, FloatMultiply,
    FloatNegate, FloatRemainder, FloatRound, FloatSubtract, FloatTrunc, IntegerClamp, IntegerMax,
    IntegerMin, Multiply, Negate, Pow, Remainder, Subtract,
};
use evo_values::definitions::scalars::PowerExponent;
use evo_values::numeric::*;

// ============================================================================
// 1. Add tests for all 10 widths
// ============================================================================

macro_rules! test_add_signed {
    ($test_name:ident, $fn_name:ident, $const_name:ident, $t:ty) => {
        #[test]
        fn $test_name() {
            // Normal
            assert_eq!($fn_name(10, 20), Ok(30));
            assert_eq!($fn_name(-10, 20), Ok(10));
            // Valid boundary
            assert_eq!($fn_name(<$t>::MAX, 0), Ok(<$t>::MAX));
            assert_eq!($fn_name(<$t>::MIN, 0), Ok(<$t>::MIN));
            // Overflow
            assert_eq!($fn_name(<$t>::MAX, 1), Err(NumericFailure::Overflow));
            assert_eq!($fn_name(<$t>::MIN, -1), Err(NumericFailure::Overflow));
            // Function pointer binding
            let op: Add<$t> = $const_name;
            assert_eq!(op(1, 2), Ok(3));
        }
    };
}

test_add_signed!(test_add_i8, add_i8, ADD_I8, i8);
test_add_signed!(test_add_i16, add_i16, ADD_I16, i16);
test_add_signed!(test_add_i32, add_i32, ADD_I32, i32);
test_add_signed!(test_add_i64, add_i64, ADD_I64, i64);
test_add_signed!(test_add_i128, add_i128, ADD_I128, i128);

macro_rules! test_add_unsigned {
    ($test_name:ident, $fn_name:ident, $const_name:ident, $t:ty) => {
        #[test]
        fn $test_name() {
            // Normal
            assert_eq!($fn_name(10, 20), Ok(30));
            // Valid boundary
            assert_eq!($fn_name(<$t>::MAX, 0), Ok(<$t>::MAX));
            assert_eq!($fn_name(0, 0), Ok(0));
            // Overflow
            assert_eq!($fn_name(<$t>::MAX, 1), Err(NumericFailure::Overflow));
            // Function pointer binding
            let op: Add<$t> = $const_name;
            assert_eq!(op(1, 2), Ok(3));
        }
    };
}

test_add_unsigned!(test_add_u8, add_u8, ADD_U8, u8);
test_add_unsigned!(test_add_u16, add_u16, ADD_U16, u16);
test_add_unsigned!(test_add_u32, add_u32, ADD_U32, u32);
test_add_unsigned!(test_add_u64, add_u64, ADD_U64, u64);
test_add_unsigned!(test_add_u128, add_u128, ADD_U128, u128);

// ============================================================================
// 2. Subtract tests for all 10 widths
// ============================================================================

macro_rules! test_subtract_signed {
    ($test_name:ident, $fn_name:ident, $const_name:ident, $t:ty) => {
        #[test]
        fn $test_name() {
            // Normal
            assert_eq!($fn_name(30, 10), Ok(20));
            assert_eq!($fn_name(10, 30), Ok(-20));
            // Boundary
            assert_eq!($fn_name(<$t>::MIN, 0), Ok(<$t>::MIN));
            assert_eq!($fn_name(<$t>::MAX, 0), Ok(<$t>::MAX));
            // Overflow
            assert_eq!($fn_name(<$t>::MIN, 1), Err(NumericFailure::Overflow));
            assert_eq!($fn_name(<$t>::MAX, -1), Err(NumericFailure::Overflow));
            // Function pointer binding
            let op: Subtract<$t> = $const_name;
            assert_eq!(op(5, 2), Ok(3));
        }
    };
}

test_subtract_signed!(test_subtract_i8, subtract_i8, SUBTRACT_I8, i8);
test_subtract_signed!(test_subtract_i16, subtract_i16, SUBTRACT_I16, i16);
test_subtract_signed!(test_subtract_i32, subtract_i32, SUBTRACT_I32, i32);
test_subtract_signed!(test_subtract_i64, subtract_i64, SUBTRACT_I64, i64);
test_subtract_signed!(test_subtract_i128, subtract_i128, SUBTRACT_I128, i128);

macro_rules! test_subtract_unsigned {
    ($test_name:ident, $fn_name:ident, $const_name:ident, $t:ty) => {
        #[test]
        fn $test_name() {
            // Normal
            assert_eq!($fn_name(30, 10), Ok(20));
            // Boundary
            assert_eq!($fn_name(0, 0), Ok(0));
            assert_eq!($fn_name(<$t>::MAX, 0), Ok(<$t>::MAX));
            // Overflow
            assert_eq!($fn_name(0, 1), Err(NumericFailure::Overflow));
            // Function pointer binding
            let op: Subtract<$t> = $const_name;
            assert_eq!(op(5, 2), Ok(3));
        }
    };
}

test_subtract_unsigned!(test_subtract_u8, subtract_u8, SUBTRACT_U8, u8);
test_subtract_unsigned!(test_subtract_u16, subtract_u16, SUBTRACT_U16, u16);
test_subtract_unsigned!(test_subtract_u32, subtract_u32, SUBTRACT_U32, u32);
test_subtract_unsigned!(test_subtract_u64, subtract_u64, SUBTRACT_U64, u64);
test_subtract_unsigned!(test_subtract_u128, subtract_u128, SUBTRACT_U128, u128);

// ============================================================================
// 3. Multiply tests for all 10 widths
// ============================================================================

macro_rules! test_multiply_signed {
    ($test_name:ident, $fn_name:ident, $const_name:ident, $t:ty) => {
        #[test]
        fn $test_name() {
            // Normal, zero, one, signed
            assert_eq!($fn_name(6, 7), Ok(42));
            assert_eq!($fn_name(0, 100), Ok(0));
            assert_eq!($fn_name(100, 0), Ok(0));
            assert_eq!($fn_name(1, 42), Ok(42));
            assert_eq!($fn_name(-1, 42), Ok(-42));
            assert_eq!($fn_name(-6, -7), Ok(42));
            // Overflow
            assert_eq!($fn_name(<$t>::MAX, 2), Err(NumericFailure::Overflow));
            assert_eq!($fn_name(<$t>::MIN, -1), Err(NumericFailure::Overflow));
            assert_eq!($fn_name(<$t>::MIN, 2), Err(NumericFailure::Overflow));
            // Function pointer binding
            let op: Multiply<$t> = $const_name;
            assert_eq!(op(3, 4), Ok(12));
        }
    };
}

test_multiply_signed!(test_multiply_i8, multiply_i8, MULTIPLY_I8, i8);
test_multiply_signed!(test_multiply_i16, multiply_i16, MULTIPLY_I16, i16);
test_multiply_signed!(test_multiply_i32, multiply_i32, MULTIPLY_I32, i32);
test_multiply_signed!(test_multiply_i64, multiply_i64, MULTIPLY_I64, i64);
test_multiply_signed!(test_multiply_i128, multiply_i128, MULTIPLY_I128, i128);

macro_rules! test_multiply_unsigned {
    ($test_name:ident, $fn_name:ident, $const_name:ident, $t:ty) => {
        #[test]
        fn $test_name() {
            // Normal, zero, one
            assert_eq!($fn_name(6, 7), Ok(42));
            assert_eq!($fn_name(0, 100), Ok(0));
            assert_eq!($fn_name(100, 0), Ok(0));
            assert_eq!($fn_name(1, 42), Ok(42));
            // Overflow
            assert_eq!($fn_name(<$t>::MAX, 2), Err(NumericFailure::Overflow));
            // Function pointer binding
            let op: Multiply<$t> = $const_name;
            assert_eq!(op(3, 4), Ok(12));
        }
    };
}

test_multiply_unsigned!(test_multiply_u8, multiply_u8, MULTIPLY_U8, u8);
test_multiply_unsigned!(test_multiply_u16, multiply_u16, MULTIPLY_U16, u16);
test_multiply_unsigned!(test_multiply_u32, multiply_u32, MULTIPLY_U32, u32);
test_multiply_unsigned!(test_multiply_u64, multiply_u64, MULTIPLY_U64, u64);
test_multiply_unsigned!(test_multiply_u128, multiply_u128, MULTIPLY_U128, u128);

// ============================================================================
// 4. Divide tests for all 10 widths
// ============================================================================

macro_rules! test_divide_signed {
    ($test_name:ident, $fn_name:ident, $const_name:ident, $t:ty) => {
        #[test]
        fn $test_name() {
            // Normal
            assert_eq!($fn_name(42, 6), Ok(7));
            // Division by zero
            assert_eq!($fn_name(42, 0), Err(NumericFailure::DivisionByZero));
            assert_eq!($fn_name(0, 0), Err(NumericFailure::DivisionByZero));
            // Sign cases & truncation toward zero
            assert_eq!($fn_name(-7, 3), Ok(-2));
            assert_eq!($fn_name(7, -3), Ok(-2));
            assert_eq!($fn_name(-7, -3), Ok(2));
            // MIN / -1 Overflow
            assert_eq!($fn_name(<$t>::MIN, -1), Err(NumericFailure::Overflow));
            // Function pointer binding
            let op: Divide<$t> = $const_name;
            assert_eq!(op(10, 2), Ok(5));
        }
    };
}

test_divide_signed!(test_divide_i8, divide_i8, DIVIDE_I8, i8);
test_divide_signed!(test_divide_i16, divide_i16, DIVIDE_I16, i16);
test_divide_signed!(test_divide_i32, divide_i32, DIVIDE_I32, i32);
test_divide_signed!(test_divide_i64, divide_i64, DIVIDE_I64, i64);
test_divide_signed!(test_divide_i128, divide_i128, DIVIDE_I128, i128);

macro_rules! test_divide_unsigned {
    ($test_name:ident, $fn_name:ident, $const_name:ident, $t:ty) => {
        #[test]
        fn $test_name() {
            // Normal
            assert_eq!($fn_name(42, 6), Ok(7));
            assert_eq!($fn_name(7, 3), Ok(2));
            // Division by zero
            assert_eq!($fn_name(42, 0), Err(NumericFailure::DivisionByZero));
            assert_eq!($fn_name(0, 0), Err(NumericFailure::DivisionByZero));
            // Function pointer binding
            let op: Divide<$t> = $const_name;
            assert_eq!(op(10, 2), Ok(5));
        }
    };
}

test_divide_unsigned!(test_divide_u8, divide_u8, DIVIDE_U8, u8);
test_divide_unsigned!(test_divide_u16, divide_u16, DIVIDE_U16, u16);
test_divide_unsigned!(test_divide_u32, divide_u32, DIVIDE_U32, u32);
test_divide_unsigned!(test_divide_u64, divide_u64, DIVIDE_U64, u64);
test_divide_unsigned!(test_divide_u128, divide_u128, DIVIDE_U128, u128);

// ============================================================================
// 5. Remainder tests for all 10 widths
// ============================================================================

macro_rules! test_remainder_signed {
    ($test_name:ident, $fn_name:ident, $const_name:ident, $t:ty) => {
        #[test]
        fn $test_name() {
            // Normal & signed semantics
            assert_eq!($fn_name(7, 3), Ok(1));
            assert_eq!($fn_name(-7, 3), Ok(-1));
            assert_eq!($fn_name(7, -3), Ok(1));
            assert_eq!($fn_name(-7, -3), Ok(-1));
            // Division by zero
            assert_eq!($fn_name(42, 0), Err(NumericFailure::DivisionByZero));
            // MIN % -1 Overflow
            assert_eq!($fn_name(<$t>::MIN, -1), Err(NumericFailure::Overflow));
            // Function pointer binding
            let op: Remainder<$t> = $const_name;
            assert_eq!(op(10, 3), Ok(1));
        }
    };
}

test_remainder_signed!(test_remainder_i8, remainder_i8, REMAINDER_I8, i8);
test_remainder_signed!(test_remainder_i16, remainder_i16, REMAINDER_I16, i16);
test_remainder_signed!(test_remainder_i32, remainder_i32, REMAINDER_I32, i32);
test_remainder_signed!(test_remainder_i64, remainder_i64, REMAINDER_I64, i64);
test_remainder_signed!(test_remainder_i128, remainder_i128, REMAINDER_I128, i128);

macro_rules! test_remainder_unsigned {
    ($test_name:ident, $fn_name:ident, $const_name:ident, $t:ty) => {
        #[test]
        fn $test_name() {
            // Normal
            assert_eq!($fn_name(7, 3), Ok(1));
            assert_eq!($fn_name(6, 3), Ok(0));
            // Division by zero
            assert_eq!($fn_name(42, 0), Err(NumericFailure::DivisionByZero));
            // Function pointer binding
            let op: Remainder<$t> = $const_name;
            assert_eq!(op(10, 3), Ok(1));
        }
    };
}

test_remainder_unsigned!(test_remainder_u8, remainder_u8, REMAINDER_U8, u8);
test_remainder_unsigned!(test_remainder_u16, remainder_u16, REMAINDER_U16, u16);
test_remainder_unsigned!(test_remainder_u32, remainder_u32, REMAINDER_U32, u32);
test_remainder_unsigned!(test_remainder_u64, remainder_u64, REMAINDER_U64, u64);
test_remainder_unsigned!(test_remainder_u128, remainder_u128, REMAINDER_U128, u128);

// ============================================================================
// 6. Negate tests for all 5 signed widths
// ============================================================================

macro_rules! test_negate_signed {
    ($test_name:ident, $fn_name:ident, $const_name:ident, $t:ty) => {
        #[test]
        fn $test_name() {
            // Zero, positive, negative
            assert_eq!($fn_name(0), Ok(0));
            assert_eq!($fn_name(5), Ok(-5));
            assert_eq!($fn_name(-5), Ok(5));
            assert_eq!($fn_name(<$t>::MAX), Ok(-<$t>::MAX));
            // MIN -> Overflow
            assert_eq!($fn_name(<$t>::MIN), Err(NumericFailure::Overflow));
            // Function pointer binding
            let op: Negate<$t> = $const_name;
            assert_eq!(op(10), Ok(-10));
        }
    };
}

test_negate_signed!(test_negate_i8, negate_i8, NEGATE_I8, i8);
test_negate_signed!(test_negate_i16, negate_i16, NEGATE_I16, i16);
test_negate_signed!(test_negate_i32, negate_i32, NEGATE_I32, i32);
test_negate_signed!(test_negate_i64, negate_i64, NEGATE_I64, i64);
test_negate_signed!(test_negate_i128, negate_i128, NEGATE_I128, i128);

// ============================================================================
// 7. Abs tests for all 5 signed widths
// ============================================================================

macro_rules! test_abs_signed {
    ($test_name:ident, $fn_name:ident, $const_name:ident, $t:ty) => {
        #[test]
        fn $test_name() {
            // Zero, positive, negative
            assert_eq!($fn_name(0), Ok(0));
            assert_eq!($fn_name(5), Ok(5));
            assert_eq!($fn_name(-5), Ok(5));
            assert_eq!($fn_name(<$t>::MAX), Ok(<$t>::MAX));
            // MIN -> Overflow
            assert_eq!($fn_name(<$t>::MIN), Err(NumericFailure::Overflow));
            // Function pointer binding
            let op: Abs<$t> = $const_name;
            assert_eq!(op(-10), Ok(10));
        }
    };
}

test_abs_signed!(test_abs_i8, abs_i8, ABS_I8, i8);
test_abs_signed!(test_abs_i16, abs_i16, ABS_I16, i16);
test_abs_signed!(test_abs_i32, abs_i32, ABS_I32, i32);
test_abs_signed!(test_abs_i64, abs_i64, ABS_I64, i64);
test_abs_signed!(test_abs_i128, abs_i128, ABS_I128, i128);

// ============================================================================
// 8. Pow tests for all 10 widths
// ============================================================================

macro_rules! test_pow_signed {
    ($test_name:ident, $fn_name:ident, $const_name:ident, $t:ty, $overflow_exp:expr) => {
        #[test]
        fn $test_name() {
            // x^0, 0^0, x^1
            assert_eq!($fn_name(5, PowerExponent(0)), Ok(1));
            assert_eq!($fn_name(0, PowerExponent(0)), Ok(1));
            assert_eq!($fn_name(5, PowerExponent(1)), Ok(5));
            assert_eq!($fn_name(-5, PowerExponent(1)), Ok(-5));
            // Normal power
            assert_eq!($fn_name(2, PowerExponent(3)), Ok(8));
            assert_eq!($fn_name(-2, PowerExponent(3)), Ok(-8));
            assert_eq!($fn_name(-2, PowerExponent(2)), Ok(4));
            // Overflow
            assert_eq!(
                $fn_name(2, PowerExponent($overflow_exp)),
                Err(NumericFailure::Overflow)
            );
            // Function pointer binding
            let op: Pow<$t> = $const_name;
            assert_eq!(op(3, PowerExponent(2)), Ok(9));
        }
    };
}

test_pow_signed!(test_pow_i8, pow_i8, POW_I8, i8, 7);
test_pow_signed!(test_pow_i16, pow_i16, POW_I16, i16, 15);
test_pow_signed!(test_pow_i32, pow_i32, POW_I32, i32, 31);
test_pow_signed!(test_pow_i64, pow_i64, POW_I64, i64, 63);
test_pow_signed!(test_pow_i128, pow_i128, POW_I128, i128, 127);

macro_rules! test_pow_unsigned {
    ($test_name:ident, $fn_name:ident, $const_name:ident, $t:ty, $overflow_exp:expr) => {
        #[test]
        fn $test_name() {
            // x^0, 0^0, x^1
            assert_eq!($fn_name(5, PowerExponent(0)), Ok(1));
            assert_eq!($fn_name(0, PowerExponent(0)), Ok(1));
            assert_eq!($fn_name(5, PowerExponent(1)), Ok(5));
            // Normal power
            assert_eq!($fn_name(2, PowerExponent(3)), Ok(8));
            // Overflow
            assert_eq!(
                $fn_name(2, PowerExponent($overflow_exp)),
                Err(NumericFailure::Overflow)
            );
            // Function pointer binding
            let op: Pow<$t> = $const_name;
            assert_eq!(op(3, PowerExponent(2)), Ok(9));
        }
    };
}

test_pow_unsigned!(test_pow_u8, pow_u8, POW_U8, u8, 8);
test_pow_unsigned!(test_pow_u16, pow_u16, POW_U16, u16, 16);
test_pow_unsigned!(test_pow_u32, pow_u32, POW_U32, u32, 32);
test_pow_unsigned!(test_pow_u64, pow_u64, POW_U64, u64, 64);
test_pow_unsigned!(test_pow_u128, pow_u128, POW_U128, u128, 128);

// ============================================================================
// 9. Function pointer contracts
// ============================================================================

#[test]
fn test_function_pointer_contracts() {
    let op_add: Add<i32> = ADD_I32;
    assert_eq!(op_add(2, 3), Ok(5));

    let op_sub: Subtract<u32> = SUBTRACT_U32;
    assert_eq!(op_sub(10, 4), Ok(6));

    let op_mul: Multiply<i128> = MULTIPLY_I128;
    assert_eq!(op_mul(3, 7), Ok(21));

    let op_div: Divide<u8> = DIVIDE_U8;
    assert_eq!(op_div(20, 4), Ok(5));

    let op_rem: Remainder<i8> = REMAINDER_I8;
    assert_eq!(op_rem(10, 3), Ok(1));

    let op_neg: Negate<i64> = NEGATE_I64;
    assert_eq!(op_neg(123), Ok(-123));

    let op_abs: Abs<i16> = ABS_I16;
    assert_eq!(op_abs(-456), Ok(456));

    let op_pow: Pow<u128> = POW_U128;
    assert_eq!(op_pow(10, PowerExponent(3)), Ok(1000));
}

// ============================================================================
// 10. Min tests for all 10 widths
// ============================================================================

macro_rules! test_min_signed {
    ($test_name:ident, $fn_name:ident, $const_name:ident, $t:ty) => {
        #[test]
        fn $test_name() {
            assert_eq!($fn_name(2, 5), 2);
            assert_eq!($fn_name(5, 2), 2);
            assert_eq!($fn_name(5, 5), 5);
            assert_eq!($fn_name(-5, 5), -5);
            assert_eq!($fn_name(5, -5), -5);
            assert_eq!($fn_name(<$t>::MIN, <$t>::MAX), <$t>::MIN);
            assert_eq!($fn_name(<$t>::MAX, <$t>::MIN), <$t>::MIN);

            let op: IntegerMin<$t> = $const_name;
            assert_eq!(op(10, 20), 10);
        }
    };
}

test_min_signed!(test_min_i8, min_i8, MIN_I8, i8);
test_min_signed!(test_min_i16, min_i16, MIN_I16, i16);
test_min_signed!(test_min_i32, min_i32, MIN_I32, i32);
test_min_signed!(test_min_i64, min_i64, MIN_I64, i64);
test_min_signed!(test_min_i128, min_i128, MIN_I128, i128);

macro_rules! test_min_unsigned {
    ($test_name:ident, $fn_name:ident, $const_name:ident, $t:ty) => {
        #[test]
        fn $test_name() {
            assert_eq!($fn_name(2, 5), 2);
            assert_eq!($fn_name(5, 2), 2);
            assert_eq!($fn_name(5, 5), 5);
            assert_eq!($fn_name(0, <$t>::MAX), 0);
            assert_eq!($fn_name(<$t>::MAX, 0), 0);

            let op: IntegerMin<$t> = $const_name;
            assert_eq!(op(10, 20), 10);
        }
    };
}

test_min_unsigned!(test_min_u8, min_u8, MIN_U8, u8);
test_min_unsigned!(test_min_u16, min_u16, MIN_U16, u16);
test_min_unsigned!(test_min_u32, min_u32, MIN_U32, u32);
test_min_unsigned!(test_min_u64, min_u64, MIN_U64, u64);
test_min_unsigned!(test_min_u128, min_u128, MIN_U128, u128);

// ============================================================================
// 11. Max tests for all 10 widths
// ============================================================================

macro_rules! test_max_signed {
    ($test_name:ident, $fn_name:ident, $const_name:ident, $t:ty) => {
        #[test]
        fn $test_name() {
            assert_eq!($fn_name(2, 5), 5);
            assert_eq!($fn_name(5, 2), 5);
            assert_eq!($fn_name(5, 5), 5);
            assert_eq!($fn_name(-5, 5), 5);
            assert_eq!($fn_name(5, -5), 5);
            assert_eq!($fn_name(<$t>::MIN, <$t>::MAX), <$t>::MAX);
            assert_eq!($fn_name(<$t>::MAX, <$t>::MIN), <$t>::MAX);

            let op: IntegerMax<$t> = $const_name;
            assert_eq!(op(10, 20), 20);
        }
    };
}

test_max_signed!(test_max_i8, max_i8, MAX_I8, i8);
test_max_signed!(test_max_i16, max_i16, MAX_I16, i16);
test_max_signed!(test_max_i32, max_i32, MAX_I32, i32);
test_max_signed!(test_max_i64, max_i64, MAX_I64, i64);
test_max_signed!(test_max_i128, max_i128, MAX_I128, i128);

macro_rules! test_max_unsigned {
    ($test_name:ident, $fn_name:ident, $const_name:ident, $t:ty) => {
        #[test]
        fn $test_name() {
            assert_eq!($fn_name(2, 5), 5);
            assert_eq!($fn_name(5, 2), 5);
            assert_eq!($fn_name(5, 5), 5);
            assert_eq!($fn_name(0, <$t>::MAX), <$t>::MAX);
            assert_eq!($fn_name(<$t>::MAX, 0), <$t>::MAX);

            let op: IntegerMax<$t> = $const_name;
            assert_eq!(op(10, 20), 20);
        }
    };
}

test_max_unsigned!(test_max_u8, max_u8, MAX_U8, u8);
test_max_unsigned!(test_max_u16, max_u16, MAX_U16, u16);
test_max_unsigned!(test_max_u32, max_u32, MAX_U32, u32);
test_max_unsigned!(test_max_u64, max_u64, MAX_U64, u64);
test_max_unsigned!(test_max_u128, max_u128, MAX_U128, u128);

// ============================================================================
// 12. Clamp tests for all 10 widths
// ============================================================================

macro_rules! test_clamp_signed {
    ($test_name:ident, $fn_name:ident, $const_name:ident, $t:ty) => {
        #[test]
        fn $test_name() {
            // Within, below, above
            assert_eq!($fn_name(5, 0, 10), Ok(5));
            assert_eq!($fn_name(-5, 0, 10), Ok(0));
            assert_eq!($fn_name(20, 0, 10), Ok(10));

            // Equals minimum, equals maximum
            assert_eq!($fn_name(0, 0, 10), Ok(0));
            assert_eq!($fn_name(10, 0, 10), Ok(10));

            // Minimum == maximum
            assert_eq!($fn_name(100, 7, 7), Ok(7));
            assert_eq!($fn_name(-100, 7, 7), Ok(7));
            assert_eq!($fn_name(7, 7, 7), Ok(7));

            // Boundaries
            assert_eq!($fn_name(<$t>::MIN, -10, 10), Ok(-10));
            assert_eq!($fn_name(<$t>::MAX, -10, 10), Ok(10));
            assert_eq!($fn_name(0, <$t>::MIN, <$t>::MAX), Ok(0));

            // Invalid bounds
            assert_eq!($fn_name(5, 10, 0), Err(NumericFailure::InvalidBounds));
            assert_eq!(
                $fn_name(0, <$t>::MAX, <$t>::MIN),
                Err(NumericFailure::InvalidBounds)
            );

            // Function pointer binding
            let op: IntegerClamp<$t> = $const_name;
            assert_eq!(op(5, 0, 10), Ok(5));
        }
    };
}

test_clamp_signed!(test_clamp_i8, clamp_i8, CLAMP_I8, i8);
test_clamp_signed!(test_clamp_i16, clamp_i16, CLAMP_I16, i16);
test_clamp_signed!(test_clamp_i32, clamp_i32, CLAMP_I32, i32);
test_clamp_signed!(test_clamp_i64, clamp_i64, CLAMP_I64, i64);
test_clamp_signed!(test_clamp_i128, clamp_i128, CLAMP_I128, i128);

macro_rules! test_clamp_unsigned {
    ($test_name:ident, $fn_name:ident, $const_name:ident, $t:ty) => {
        #[test]
        fn $test_name() {
            // Within, below, above
            assert_eq!($fn_name(5, 2, 10), Ok(5));
            assert_eq!($fn_name(1, 2, 10), Ok(2));
            assert_eq!($fn_name(20, 2, 10), Ok(10));

            // Equals minimum, equals maximum
            assert_eq!($fn_name(2, 2, 10), Ok(2));
            assert_eq!($fn_name(10, 2, 10), Ok(10));

            // Minimum == maximum
            assert_eq!($fn_name(100, 7, 7), Ok(7));
            assert_eq!($fn_name(0, 7, 7), Ok(7));
            assert_eq!($fn_name(7, 7, 7), Ok(7));

            // Boundaries
            assert_eq!($fn_name(0, 10, 100), Ok(10));
            assert_eq!($fn_name(<$t>::MAX, 10, 100), Ok(100));
            assert_eq!($fn_name(50, 0, <$t>::MAX), Ok(50));

            // Invalid bounds
            assert_eq!($fn_name(5, 10, 2), Err(NumericFailure::InvalidBounds));
            assert_eq!(
                $fn_name(0, <$t>::MAX, 0),
                Err(NumericFailure::InvalidBounds)
            );

            // Function pointer binding
            let op: IntegerClamp<$t> = $const_name;
            assert_eq!(op(5, 2, 10), Ok(5));
        }
    };
}

test_clamp_unsigned!(test_clamp_u8, clamp_u8, CLAMP_U8, u8);
test_clamp_unsigned!(test_clamp_u16, clamp_u16, CLAMP_U16, u16);
test_clamp_unsigned!(test_clamp_u32, clamp_u32, CLAMP_U32, u32);
test_clamp_unsigned!(test_clamp_u64, clamp_u64, CLAMP_U64, u64);
test_clamp_unsigned!(test_clamp_u128, clamp_u128, CLAMP_U128, u128);

// ============================================================================
// 13. Min, Max, Clamp function pointer contracts & public surface
// ============================================================================

#[test]
fn test_min_max_clamp_function_pointer_contracts() {
    let min_op: IntegerMin<i32> = MIN_I32;
    assert_eq!(min_op(10, 20), 10);

    let max_op: IntegerMax<u64> = MAX_U64;
    assert_eq!(max_op(10, 20), 20);

    let clamp_op: IntegerClamp<i128> = CLAMP_I128;
    assert_eq!(clamp_op(5, 0, 10), Ok(5));
}

// ============================================================================
// 14. Float Negate tests (f32, f64)
// ============================================================================

#[test]
fn test_float_negate_f32() {
    assert_eq!(negate_f32(5.5), -5.5);
    assert_eq!(negate_f32(-5.5), 5.5);
    assert_eq!(negate_f32(0.0), -0.0);
    assert_eq!(negate_f32(-0.0), 0.0);
    assert_eq!(negate_f32(f32::INFINITY), f32::NEG_INFINITY);
    assert_eq!(negate_f32(f32::NEG_INFINITY), f32::INFINITY);
    assert!(negate_f32(f32::NAN).is_nan());

    let op: FloatNegate<f32> = NEGATE_F32;
    assert_eq!(op(10.0), -10.0);
}

#[test]
fn test_float_negate_f64() {
    assert_eq!(negate_f64(5.5), -5.5);
    assert_eq!(negate_f64(-5.5), 5.5);
    assert_eq!(negate_f64(0.0), -0.0);
    assert_eq!(negate_f64(-0.0), 0.0);
    assert_eq!(negate_f64(f64::INFINITY), f64::NEG_INFINITY);
    assert_eq!(negate_f64(f64::NEG_INFINITY), f64::INFINITY);
    assert!(negate_f64(f64::NAN).is_nan());

    let op: FloatNegate<f64> = NEGATE_F64;
    assert_eq!(op(10.0), -10.0);
}

// ============================================================================
// 15. Float Add tests (f32, f64)
// ============================================================================

#[test]
fn test_float_add_f32() {
    assert_eq!(add_f32(1.5, 2.5), 4.0);
    assert_eq!(add_f32(-1.5, 2.5), 1.0);
    assert_eq!(add_f32(0.0, -0.0), 0.0);
    assert_eq!(add_f32(f32::MAX, f32::MAX), f32::INFINITY);
    assert_eq!(add_f32(f32::INFINITY, 1.0), f32::INFINITY);
    assert!(add_f32(f32::INFINITY, f32::NEG_INFINITY).is_nan());
    assert!(add_f32(f32::NAN, 1.0).is_nan());

    let op: FloatAdd<f32> = ADD_F32;
    assert_eq!(op(1.0, 2.0), 3.0);
}

#[test]
fn test_float_add_f64() {
    assert_eq!(add_f64(1.5, 2.5), 4.0);
    assert_eq!(add_f64(-1.5, 2.5), 1.0);
    assert_eq!(add_f64(0.0, -0.0), 0.0);
    assert_eq!(add_f64(f64::MAX, f64::MAX), f64::INFINITY);
    assert_eq!(add_f64(f64::INFINITY, 1.0), f64::INFINITY);
    assert!(add_f64(f64::INFINITY, f64::NEG_INFINITY).is_nan());
    assert!(add_f64(f64::NAN, 1.0).is_nan());

    let op: FloatAdd<f64> = ADD_F64;
    assert_eq!(op(1.0, 2.0), 3.0);
}

// ============================================================================
// 16. Float Subtract tests (f32, f64)
// ============================================================================

#[test]
fn test_float_subtract_f32() {
    assert_eq!(subtract_f32(5.5, 2.5), 3.0);
    assert_eq!(subtract_f32(2.5, 5.5), -3.0);
    assert_eq!(subtract_f32(0.0, 0.0), 0.0);
    assert_eq!(subtract_f32(-f32::MAX, f32::MAX), f32::NEG_INFINITY);
    assert!(subtract_f32(f32::INFINITY, f32::INFINITY).is_nan());
    assert!(subtract_f32(f32::NAN, 1.0).is_nan());

    let op: FloatSubtract<f32> = SUBTRACT_F32;
    assert_eq!(op(10.0, 4.0), 6.0);
}

#[test]
fn test_float_subtract_f64() {
    assert_eq!(subtract_f64(5.5, 2.5), 3.0);
    assert_eq!(subtract_f64(2.5, 5.5), -3.0);
    assert_eq!(subtract_f64(0.0, 0.0), 0.0);
    assert_eq!(subtract_f64(-f64::MAX, f64::MAX), f64::NEG_INFINITY);
    assert!(subtract_f64(f64::INFINITY, f64::INFINITY).is_nan());
    assert!(subtract_f64(f64::NAN, 1.0).is_nan());

    let op: FloatSubtract<f64> = SUBTRACT_F64;
    assert_eq!(op(10.0, 4.0), 6.0);
}

// ============================================================================
// 17. Float Multiply tests (f32, f64)
// ============================================================================

#[test]
fn test_float_multiply_f32() {
    assert_eq!(multiply_f32(2.5, 4.0), 10.0);
    assert_eq!(multiply_f32(-2.5, 4.0), -10.0);
    assert_eq!(multiply_f32(-2.5, -4.0), 10.0);
    assert_eq!(multiply_f32(0.0, 100.0), 0.0);
    assert_eq!(multiply_f32(f32::MAX, 2.0), f32::INFINITY);
    assert!(multiply_f32(0.0, f32::INFINITY).is_nan());
    assert!(multiply_f32(f32::NAN, 2.0).is_nan());

    let op: FloatMultiply<f32> = MULTIPLY_F32;
    assert_eq!(op(3.0, 4.0), 12.0);
}

#[test]
fn test_float_multiply_f64() {
    assert_eq!(multiply_f64(2.5, 4.0), 10.0);
    assert_eq!(multiply_f64(-2.5, 4.0), -10.0);
    assert_eq!(multiply_f64(-2.5, -4.0), 10.0);
    assert_eq!(multiply_f64(0.0, 100.0), 0.0);
    assert_eq!(multiply_f64(f64::MAX, 2.0), f64::INFINITY);
    assert!(multiply_f64(0.0, f64::INFINITY).is_nan());
    assert!(multiply_f64(f64::NAN, 2.0).is_nan());

    let op: FloatMultiply<f64> = MULTIPLY_F64;
    assert_eq!(op(3.0, 4.0), 12.0);
}

// ============================================================================
// 18. Float Divide tests (f32, f64)
// ============================================================================

#[test]
fn test_float_divide_f32() {
    assert_eq!(divide_f32(10.0, 2.0), 5.0);
    assert_eq!(divide_f32(1.0, 0.0), f32::INFINITY);
    assert_eq!(divide_f32(-1.0, 0.0), f32::NEG_INFINITY);
    assert!(divide_f32(0.0, 0.0).is_nan());
    assert!(divide_f32(f32::INFINITY, f32::INFINITY).is_nan());
    assert!(divide_f32(f32::NAN, 2.0).is_nan());

    let op: FloatDivide<f32> = DIVIDE_F32;
    assert_eq!(op(20.0, 4.0), 5.0);
}

#[test]
fn test_float_divide_f64() {
    assert_eq!(divide_f64(10.0, 2.0), 5.0);
    assert_eq!(divide_f64(1.0, 0.0), f64::INFINITY);
    assert_eq!(divide_f64(-1.0, 0.0), f64::NEG_INFINITY);
    assert!(divide_f64(0.0, 0.0).is_nan());
    assert!(divide_f64(f64::INFINITY, f64::INFINITY).is_nan());
    assert!(divide_f64(f64::NAN, 2.0).is_nan());

    let op: FloatDivide<f64> = DIVIDE_F64;
    assert_eq!(op(20.0, 4.0), 5.0);
}

// ============================================================================
// 19. Float Remainder tests (f32, f64)
// ============================================================================

#[test]
fn test_float_remainder_f32() {
    assert_eq!(remainder_f32(7.5, 2.5), 0.0);
    assert_eq!(remainder_f32(7.0, 2.5), 2.0);
    assert_eq!(remainder_f32(-7.0, 2.5), -2.0);
    assert_eq!(remainder_f32(7.0, -2.5), 2.0);
    assert!(remainder_f32(7.0, 0.0).is_nan());
    assert!(remainder_f32(f32::NAN, 2.0).is_nan());

    let op: FloatRemainder<f32> = REMAINDER_F32;
    assert_eq!(op(7.0, 2.5), 2.0);
}

#[test]
fn test_float_remainder_f64() {
    assert_eq!(remainder_f64(7.5, 2.5), 0.0);
    assert_eq!(remainder_f64(7.0, 2.5), 2.0);
    assert_eq!(remainder_f64(-7.0, 2.5), -2.0);
    assert_eq!(remainder_f64(7.0, -2.5), 2.0);
    assert!(remainder_f64(7.0, 0.0).is_nan());
    assert!(remainder_f64(f64::NAN, 2.0).is_nan());

    let op: FloatRemainder<f64> = REMAINDER_F64;
    assert_eq!(op(7.0, 2.5), 2.0);
}

// ============================================================================
// 20. Float Abs tests (f32, f64)
// ============================================================================

#[test]
fn test_float_abs_f32() {
    assert_eq!(abs_f32(5.5), 5.5);
    assert_eq!(abs_f32(-5.5), 5.5);
    assert_eq!(abs_f32(0.0), 0.0);
    assert_eq!(abs_f32(-0.0), 0.0);
    assert_eq!(abs_f32(f32::INFINITY), f32::INFINITY);
    assert_eq!(abs_f32(f32::NEG_INFINITY), f32::INFINITY);
    assert!(abs_f32(f32::NAN).is_nan());

    let op: FloatAbs<f32> = ABS_F32;
    assert_eq!(op(-10.0), 10.0);
}

#[test]
fn test_float_abs_f64() {
    assert_eq!(abs_f64(5.5), 5.5);
    assert_eq!(abs_f64(-5.5), 5.5);
    assert_eq!(abs_f64(0.0), 0.0);
    assert_eq!(abs_f64(-0.0), 0.0);
    assert_eq!(abs_f64(f64::INFINITY), f64::INFINITY);
    assert_eq!(abs_f64(f64::NEG_INFINITY), f64::INFINITY);
    assert!(abs_f64(f64::NAN).is_nan());

    let op: FloatAbs<f64> = ABS_F64;
    assert_eq!(op(-10.0), 10.0);
}

// ============================================================================
// 21. Float Min tests (f32, f64)
// ============================================================================

#[test]
fn test_float_min_f32() {
    assert_eq!(min_f32(2.0, 5.0), 2.0);
    assert_eq!(min_f32(5.0, 2.0), 2.0);
    assert_eq!(min_f32(-5.0, 2.0), -5.0);
    assert_eq!(min_f32(2.0, -5.0), -5.0);
    assert_eq!(min_f32(f32::NEG_INFINITY, f32::INFINITY), f32::NEG_INFINITY);
    // IEEE 754-2008 min returns the non-NaN operand
    assert_eq!(min_f32(2.0, f32::NAN), 2.0);
    assert_eq!(min_f32(f32::NAN, 2.0), 2.0);

    let op: FloatMin<f32> = MIN_F32;
    assert_eq!(op(10.0, 20.0), 10.0);
}

#[test]
fn test_float_min_f64() {
    assert_eq!(min_f64(2.0, 5.0), 2.0);
    assert_eq!(min_f64(5.0, 2.0), 2.0);
    assert_eq!(min_f64(-5.0, 2.0), -5.0);
    assert_eq!(min_f64(2.0, -5.0), -5.0);
    assert_eq!(min_f64(f64::NEG_INFINITY, f64::INFINITY), f64::NEG_INFINITY);
    assert_eq!(min_f64(2.0, f64::NAN), 2.0);
    assert_eq!(min_f64(f64::NAN, 2.0), 2.0);

    let op: FloatMin<f64> = MIN_F64;
    assert_eq!(op(10.0, 20.0), 10.0);
}

// ============================================================================
// 22. Float Max tests (f32, f64)
// ============================================================================

#[test]
fn test_float_max_f32() {
    assert_eq!(max_f32(2.0, 5.0), 5.0);
    assert_eq!(max_f32(5.0, 2.0), 5.0);
    assert_eq!(max_f32(-5.0, 2.0), 2.0);
    assert_eq!(max_f32(2.0, -5.0), 2.0);
    assert_eq!(max_f32(f32::NEG_INFINITY, f32::INFINITY), f32::INFINITY);
    // IEEE 754-2008 max returns the non-NaN operand
    assert_eq!(max_f32(2.0, f32::NAN), 2.0);
    assert_eq!(max_f32(f32::NAN, 2.0), 2.0);

    let op: FloatMax<f32> = MAX_F32;
    assert_eq!(op(10.0, 20.0), 20.0);
}

#[test]
fn test_float_max_f64() {
    assert_eq!(max_f64(2.0, 5.0), 5.0);
    assert_eq!(max_f64(5.0, 2.0), 5.0);
    assert_eq!(max_f64(-5.0, 2.0), 2.0);
    assert_eq!(max_f64(2.0, -5.0), 2.0);
    assert_eq!(max_f64(f64::NEG_INFINITY, f64::INFINITY), f64::INFINITY);
    assert_eq!(max_f64(2.0, f64::NAN), 2.0);
    assert_eq!(max_f64(f64::NAN, 2.0), 2.0);

    let op: FloatMax<f64> = MAX_F64;
    assert_eq!(op(10.0, 20.0), 20.0);
}

// ============================================================================
// 23. Float Clamp tests (f32, f64)
// ============================================================================

#[test]
fn test_float_clamp_f32() {
    // Within, below, above
    assert_eq!(clamp_f32(5.0, 0.0, 10.0), Ok(5.0));
    assert_eq!(clamp_f32(-5.0, 0.0, 10.0), Ok(0.0));
    assert_eq!(clamp_f32(20.0, 0.0, 10.0), Ok(10.0));

    // Equals minimum, equals maximum
    assert_eq!(clamp_f32(0.0, 0.0, 10.0), Ok(0.0));
    assert_eq!(clamp_f32(10.0, 0.0, 10.0), Ok(10.0));

    // Minimum == maximum
    assert_eq!(clamp_f32(100.0, 7.0, 7.0), Ok(7.0));
    assert_eq!(clamp_f32(-100.0, 7.0, 7.0), Ok(7.0));

    // Invalid bounds
    assert_eq!(
        clamp_f32(5.0, 10.0, 0.0),
        Err(NumericFailure::InvalidBounds)
    );
    assert_eq!(
        clamp_f32(5.0, f32::NAN, 10.0),
        Err(NumericFailure::InvalidBounds)
    );
    assert_eq!(
        clamp_f32(5.0, 0.0, f32::NAN),
        Err(NumericFailure::InvalidBounds)
    );
    assert_eq!(
        clamp_f32(5.0, f32::NAN, f32::NAN),
        Err(NumericFailure::InvalidBounds)
    );

    // Value is NaN with valid bounds evaluates comparisons to false -> Ok(NaN)
    let nan_val = clamp_f32(f32::NAN, 0.0, 10.0).unwrap();
    assert!(nan_val.is_nan());

    let op: FloatClamp<f32> = CLAMP_F32;
    assert_eq!(op(5.0, 0.0, 10.0), Ok(5.0));
}

#[test]
fn test_float_clamp_f64() {
    // Within, below, above
    assert_eq!(clamp_f64(5.0, 0.0, 10.0), Ok(5.0));
    assert_eq!(clamp_f64(-5.0, 0.0, 10.0), Ok(0.0));
    assert_eq!(clamp_f64(20.0, 0.0, 10.0), Ok(10.0));

    // Equals minimum, equals maximum
    assert_eq!(clamp_f64(0.0, 0.0, 10.0), Ok(0.0));
    assert_eq!(clamp_f64(10.0, 0.0, 10.0), Ok(10.0));

    // Minimum == maximum
    assert_eq!(clamp_f64(100.0, 7.0, 7.0), Ok(7.0));
    assert_eq!(clamp_f64(-100.0, 7.0, 7.0), Ok(7.0));

    // Invalid bounds
    assert_eq!(
        clamp_f64(5.0, 10.0, 0.0),
        Err(NumericFailure::InvalidBounds)
    );
    assert_eq!(
        clamp_f64(5.0, f64::NAN, 10.0),
        Err(NumericFailure::InvalidBounds)
    );
    assert_eq!(
        clamp_f64(5.0, 0.0, f64::NAN),
        Err(NumericFailure::InvalidBounds)
    );
    assert_eq!(
        clamp_f64(5.0, f64::NAN, f64::NAN),
        Err(NumericFailure::InvalidBounds)
    );

    // Value is NaN with valid bounds evaluates comparisons to false -> Ok(NaN)
    let nan_val = clamp_f64(f64::NAN, 0.0, 10.0).unwrap();
    assert!(nan_val.is_nan());

    let op: FloatClamp<f64> = CLAMP_F64;
    assert_eq!(op(5.0, 0.0, 10.0), Ok(5.0));
}

// ============================================================================
// 24. Float Floor tests (f32, f64)
// ============================================================================

#[test]
fn test_float_floor_f32() {
    assert_eq!(floor_f32(3.7), 3.0);
    assert_eq!(floor_f32(-3.7), -4.0);
    assert_eq!(floor_f32(3.0), 3.0);
    assert_eq!(floor_f32(0.0), 0.0);
    assert_eq!(floor_f32(-0.0), -0.0);

    let op: FloatFloor<f32> = FLOOR_F32;
    assert_eq!(op(2.9), 2.0);
}

#[test]
fn test_float_floor_f64() {
    assert_eq!(floor_f64(3.7), 3.0);
    assert_eq!(floor_f64(-3.7), -4.0);
    assert_eq!(floor_f64(3.0), 3.0);
    assert_eq!(floor_f64(0.0), 0.0);
    assert_eq!(floor_f64(-0.0), -0.0);

    let op: FloatFloor<f64> = FLOOR_F64;
    assert_eq!(op(2.9), 2.0);
}

// ============================================================================
// 25. Float Ceil tests (f32, f64)
// ============================================================================

#[test]
fn test_float_ceil_f32() {
    assert_eq!(ceil_f32(3.2), 4.0);
    assert_eq!(ceil_f32(-3.2), -3.0);
    assert_eq!(ceil_f32(3.0), 3.0);
    assert_eq!(ceil_f32(0.0), 0.0);

    let op: FloatCeil<f32> = CEIL_F32;
    assert_eq!(op(2.1), 3.0);
}

#[test]
fn test_float_ceil_f64() {
    assert_eq!(ceil_f64(3.2), 4.0);
    assert_eq!(ceil_f64(-3.2), -3.0);
    assert_eq!(ceil_f64(3.0), 3.0);
    assert_eq!(ceil_f64(0.0), 0.0);

    let op: FloatCeil<f64> = CEIL_F64;
    assert_eq!(op(2.1), 3.0);
}

// ============================================================================
// 26. Float Round tests (f32, f64)
// ============================================================================

#[test]
fn test_float_round_f32() {
    assert_eq!(round_f32(3.2), 3.0);
    assert_eq!(round_f32(3.5), 4.0);
    assert_eq!(round_f32(3.7), 4.0);
    assert_eq!(round_f32(-3.5), -4.0);
    assert_eq!(round_f32(-3.2), -3.0);

    let op: FloatRound<f32> = ROUND_F32;
    assert_eq!(op(2.5), 3.0);
}

#[test]
fn test_float_round_f64() {
    assert_eq!(round_f64(3.2), 3.0);
    assert_eq!(round_f64(3.5), 4.0);
    assert_eq!(round_f64(3.7), 4.0);
    assert_eq!(round_f64(-3.5), -4.0);
    assert_eq!(round_f64(-3.2), -3.0);

    let op: FloatRound<f64> = ROUND_F64;
    assert_eq!(op(2.5), 3.0);
}

// ============================================================================
// 27. Float Trunc tests (f32, f64)
// ============================================================================

#[test]
fn test_float_trunc_f32() {
    assert_eq!(trunc_f32(3.7), 3.0);
    assert_eq!(trunc_f32(-3.7), -3.0);
    assert_eq!(trunc_f32(0.5), 0.0);
    assert_eq!(trunc_f32(-0.5), -0.0);

    let op: FloatTrunc<f32> = TRUNC_F32;
    assert_eq!(op(2.9), 2.0);
}

#[test]
fn test_float_trunc_f64() {
    assert_eq!(trunc_f64(3.7), 3.0);
    assert_eq!(trunc_f64(-3.7), -3.0);
    assert_eq!(trunc_f64(0.5), 0.0);
    assert_eq!(trunc_f64(-0.5), -0.0);

    let op: FloatTrunc<f64> = TRUNC_F64;
    assert_eq!(op(2.9), 2.0);
}

// ============================================================================
// 28. Float Fract tests (f32, f64)
// ============================================================================

#[test]
fn test_float_fract_f32() {
    assert!((fract_f32(3.5) - 0.5).abs() < 1e-6);
    assert!((fract_f32(-3.5) - -0.5).abs() < 1e-6);
    assert_eq!(fract_f32(3.0), 0.0);

    let op: FloatFract<f32> = FRACT_F32;
    assert!((op(1.25) - 0.25).abs() < 1e-6);
}

#[test]
fn test_float_fract_f64() {
    assert!((fract_f64(3.5) - 0.5).abs() < 1e-10);
    assert!((fract_f64(-3.5) - -0.5).abs() < 1e-10);
    assert_eq!(fract_f64(3.0), 0.0);

    let op: FloatFract<f64> = FRACT_F64;
    assert!((op(1.25) - 0.25).abs() < 1e-10);
}

// ============================================================================
// 29. Float IsNan tests (f32, f64)
// ============================================================================

#[test]
fn test_float_is_nan_f32() {
    assert!(is_nan_f32(f32::NAN));
    assert!(!is_nan_f32(0.0));
    assert!(!is_nan_f32(f32::INFINITY));
    assert!(!is_nan_f32(1.23));

    let op: FloatIsNan<f32> = IS_NAN_F32;
    assert!(op(f32::NAN));
    assert!(!op(1.0));
}

#[test]
fn test_float_is_nan_f64() {
    assert!(is_nan_f64(f64::NAN));
    assert!(!is_nan_f64(0.0));
    assert!(!is_nan_f64(f64::INFINITY));
    assert!(!is_nan_f64(1.23));

    let op: FloatIsNan<f64> = IS_NAN_F64;
    assert!(op(f64::NAN));
    assert!(!op(1.0));
}

// ============================================================================
// 30. Float IsInfinite tests (f32, f64)
// ============================================================================

#[test]
fn test_float_is_infinite_f32() {
    assert!(is_infinite_f32(f32::INFINITY));
    assert!(is_infinite_f32(f32::NEG_INFINITY));
    assert!(!is_infinite_f32(0.0));
    assert!(!is_infinite_f32(f32::NAN));
    assert!(!is_infinite_f32(1.23));

    let op: FloatIsInfinite<f32> = IS_INFINITE_F32;
    assert!(op(f32::INFINITY));
    assert!(!op(1.0));
}

#[test]
fn test_float_is_infinite_f64() {
    assert!(is_infinite_f64(f64::INFINITY));
    assert!(is_infinite_f64(f64::NEG_INFINITY));
    assert!(!is_infinite_f64(0.0));
    assert!(!is_infinite_f64(f64::NAN));
    assert!(!is_infinite_f64(1.23));

    let op: FloatIsInfinite<f64> = IS_INFINITE_F64;
    assert!(op(f64::INFINITY));
    assert!(!op(1.0));
}

// ============================================================================
// 31. Float IsFinite tests (f32, f64)
// ============================================================================

#[test]
fn test_float_is_finite_f32() {
    assert!(is_finite_f32(0.0));
    assert!(is_finite_f32(-100.5));
    assert!(!is_finite_f32(f32::INFINITY));
    assert!(!is_finite_f32(f32::NEG_INFINITY));
    assert!(!is_finite_f32(f32::NAN));

    let op: FloatIsFinite<f32> = IS_FINITE_F32;
    assert!(op(1.0));
    assert!(!op(f32::INFINITY));
}

#[test]
fn test_float_is_finite_f64() {
    assert!(is_finite_f64(0.0));
    assert!(is_finite_f64(-100.5));
    assert!(!is_finite_f64(f64::INFINITY));
    assert!(!is_finite_f64(f64::NEG_INFINITY));
    assert!(!is_finite_f64(f64::NAN));

    let op: FloatIsFinite<f64> = IS_FINITE_F64;
    assert!(op(1.0));
    assert!(!op(f64::INFINITY));
}

// ============================================================================
// 32. Float all 18 operations function pointer contracts
// ============================================================================

#[test]
fn test_all_18_float_function_pointer_contracts() {
    let _op_neg_f32: FloatNegate<f32> = NEGATE_F32;
    let _op_neg_f64: FloatNegate<f64> = NEGATE_F64;

    let _op_add_f32: FloatAdd<f32> = ADD_F32;
    let _op_add_f64: FloatAdd<f64> = ADD_F64;

    let _op_sub_f32: FloatSubtract<f32> = SUBTRACT_F32;
    let _op_sub_f64: FloatSubtract<f64> = SUBTRACT_F64;

    let _op_mul_f32: FloatMultiply<f32> = MULTIPLY_F32;
    let _op_mul_f64: FloatMultiply<f64> = MULTIPLY_F64;

    let _op_div_f32: FloatDivide<f32> = DIVIDE_F32;
    let _op_div_f64: FloatDivide<f64> = DIVIDE_F64;

    let _op_rem_f32: FloatRemainder<f32> = REMAINDER_F32;
    let _op_rem_f64: FloatRemainder<f64> = REMAINDER_F64;

    let _op_abs_f32: FloatAbs<f32> = ABS_F32;
    let _op_abs_f64: FloatAbs<f64> = ABS_F64;

    let _op_min_f32: FloatMin<f32> = MIN_F32;
    let _op_min_f64: FloatMin<f64> = MIN_F64;

    let _op_max_f32: FloatMax<f32> = MAX_F32;
    let _op_max_f64: FloatMax<f64> = MAX_F64;

    let _op_clamp_f32: FloatClamp<f32> = CLAMP_F32;
    let _op_clamp_f64: FloatClamp<f64> = CLAMP_F64;

    let _op_floor_f32: FloatFloor<f32> = FLOOR_F32;
    let _op_floor_f64: FloatFloor<f64> = FLOOR_F64;

    let _op_ceil_f32: FloatCeil<f32> = CEIL_F32;
    let _op_ceil_f64: FloatCeil<f64> = CEIL_F64;

    let _op_round_f32: FloatRound<f32> = ROUND_F32;
    let _op_round_f64: FloatRound<f64> = ROUND_F64;

    let _op_trunc_f32: FloatTrunc<f32> = TRUNC_F32;
    let _op_trunc_f64: FloatTrunc<f64> = TRUNC_F64;

    let _op_fract_f32: FloatFract<f32> = FRACT_F32;
    let _op_fract_f64: FloatFract<f64> = FRACT_F64;

    let _op_is_nan_f32: FloatIsNan<f32> = IS_NAN_F32;
    let _op_is_nan_f64: FloatIsNan<f64> = IS_NAN_F64;

    let _op_is_infinite_f32: FloatIsInfinite<f32> = IS_INFINITE_F32;
    let _op_is_infinite_f64: FloatIsInfinite<f64> = IS_INFINITE_F64;

    let _op_is_finite_f32: FloatIsFinite<f32> = IS_FINITE_F32;
    let _op_is_finite_f64: FloatIsFinite<f64> = IS_FINITE_F64;
}
