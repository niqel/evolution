use evo_values::NumericFailure;
use evo_values::definitions::numeric::{
    Abs, Add, Divide, Multiply, Negate, Pow, Remainder, Subtract,
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
