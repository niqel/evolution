extern crate alloc;

use evo_values::conversion::*;
use evo_values::definitions::conversion::{
    ToFloat32, ToFloat64, ToInt8, ToInt16, ToInt32, ToInt64, ToInt128, ToUint8, ToUint16, ToUint32,
    ToUint64, ToUint128,
};
use evo_values::definitions::failures::ConversionFailure;

// ============================================================================
// 1. Integer -> Integer Tests (Section 17)
// ============================================================================

#[test]
fn test_integer_identities_all_ten_families() {
    assert_eq!(to_int8_from_i8(42), Ok(42));
    assert_eq!(to_int16_from_i16(420), Ok(420));
    assert_eq!(to_int32_from_i32(4200), Ok(4200));
    assert_eq!(to_int64_from_i64(42000), Ok(42000));
    assert_eq!(to_int128_from_i128(420000), Ok(420000));

    assert_eq!(to_uint8_from_u8(42), Ok(42));
    assert_eq!(to_uint16_from_u16(420), Ok(420));
    assert_eq!(to_uint32_from_u32(4200), Ok(4200));
    assert_eq!(to_uint64_from_u64(42000), Ok(42000));
    assert_eq!(to_uint128_from_u128(420000), Ok(420000));
}

#[test]
fn test_integer_widening() {
    // Signed widening
    assert_eq!(to_int16_from_i8(-128), Ok(-128));
    assert_eq!(to_int32_from_i16(-32768), Ok(-32768));
    assert_eq!(to_int64_from_i32(-2147483648), Ok(-2147483648));
    assert_eq!(
        to_int128_from_i64(-9223372036854775808),
        Ok(-9223372036854775808)
    );

    // Unsigned widening
    assert_eq!(to_uint16_from_u8(255), Ok(255));
    assert_eq!(to_uint32_from_u16(65535), Ok(65535));
    assert_eq!(to_uint64_from_u32(4294967295), Ok(4294967295));
    assert_eq!(
        to_uint128_from_u64(18446744073709551615),
        Ok(18446744073709551615)
    );

    // Unsigned to Signed widening
    assert_eq!(to_int16_from_u8(255), Ok(255));
    assert_eq!(to_int32_from_u16(65535), Ok(65535));
    assert_eq!(to_int64_from_u32(4294967295), Ok(4294967295));
    assert_eq!(
        to_int128_from_u64(18446744073709551615),
        Ok(18446744073709551615)
    );
}

#[test]
fn test_integer_narrowing_valid_and_invalid() {
    // Narrowing valid
    assert_eq!(to_int8_from_i16(127), Ok(127));
    assert_eq!(to_int8_from_i16(-128), Ok(-128));
    assert_eq!(to_uint8_from_u16(255), Ok(255));
    assert_eq!(to_int16_from_i32(32767), Ok(32767));
    assert_eq!(to_int32_from_i64(2147483647), Ok(2147483647));
    assert_eq!(
        to_int64_from_i128(9223372036854775807),
        Ok(9223372036854775807)
    );

    // Narrowing invalid
    assert_eq!(
        to_int8_from_i16(128),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_int8_from_i16(-129),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_uint8_from_u16(256),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_int16_from_i32(32768),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_int32_from_i64(2147483648),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_int64_from_i128(9223372036854775808),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
}

#[test]
fn test_signed_to_unsigned() {
    // Positive fits
    assert_eq!(to_uint8_from_i8(100), Ok(100));
    assert_eq!(to_uint16_from_i16(1000), Ok(1000));
    assert_eq!(to_uint32_from_i32(10000), Ok(10000));
    assert_eq!(to_uint64_from_i64(100000), Ok(100000));
    assert_eq!(to_uint128_from_i128(1000000), Ok(1000000));

    // Negative must fail for all unsigned targets
    assert_eq!(
        to_uint8_from_i8(-1),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_uint16_from_i8(-1),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_uint32_from_i8(-1),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_uint64_from_i8(-1),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_uint128_from_i8(-1),
        Err(ConversionFailure::NotExactlyRepresentable)
    );

    assert_eq!(
        to_uint8_from_i128(-1),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_uint128_from_i128(-1),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_uint128_from_i128(i128::MIN),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
}

#[test]
fn test_unsigned_to_signed() {
    // Within range
    assert_eq!(to_int8_from_u8(127), Ok(127));
    assert_eq!(to_int16_from_u16(32767), Ok(32767));
    assert_eq!(to_int32_from_u32(2147483647), Ok(2147483647));
    assert_eq!(
        to_int64_from_u64(9223372036854775807),
        Ok(9223372036854775807)
    );
    assert_eq!(to_int128_from_u128(i128::MAX as u128), Ok(i128::MAX));

    // Exceeding range
    assert_eq!(
        to_int8_from_u8(128),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_int8_from_u8(255),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_int16_from_u16(32768),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_int32_from_u32(2147483648),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_int64_from_u64(9223372036854775808),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_int128_from_u128(i128::MAX as u128 + 1),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_int128_from_u128(u128::MAX),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
}

#[test]
fn test_integer_min_max_boundaries_exhaustive() {
    // i8 boundaries
    assert_eq!(to_int128_from_i8(i8::MIN), Ok(-128));
    assert_eq!(to_int128_from_i8(i8::MAX), Ok(127));

    // i16 boundaries
    assert_eq!(to_int128_from_i16(i16::MIN), Ok(-32768));
    assert_eq!(to_int128_from_i16(i16::MAX), Ok(32767));

    // i32 boundaries
    assert_eq!(to_int128_from_i32(i32::MIN), Ok(-2147483648));
    assert_eq!(to_int128_from_i32(i32::MAX), Ok(2147483647));

    // i64 boundaries
    assert_eq!(to_int128_from_i64(i64::MIN), Ok(-9223372036854775808));
    assert_eq!(to_int128_from_i64(i64::MAX), Ok(9223372036854775807));

    // i128 boundaries
    assert_eq!(to_int128_from_i128(i128::MIN), Ok(i128::MIN));
    assert_eq!(to_int128_from_i128(i128::MAX), Ok(i128::MAX));

    // u8..u128 MAX
    assert_eq!(to_uint128_from_u8(u8::MAX), Ok(255));
    assert_eq!(to_uint128_from_u16(u16::MAX), Ok(65535));
    assert_eq!(to_uint128_from_u32(u32::MAX), Ok(4294967295));
    assert_eq!(to_uint128_from_u64(u64::MAX), Ok(18446744073709551615));
    assert_eq!(to_uint128_from_u128(u128::MAX), Ok(u128::MAX));
}

// ============================================================================
// 2. Integer -> Float32 Tests (Section 18)
// ============================================================================

#[test]
fn test_integer_to_float32() {
    // 0, 1, -1
    assert_eq!(to_float32_from_i8(0), Ok(0.0));
    assert_eq!(to_float32_from_i8(1), Ok(1.0));
    assert_eq!(to_float32_from_i8(-1), Ok(-1.0));
    assert_eq!(to_float32_from_u8(0), Ok(0.0));
    assert_eq!(to_float32_from_u8(1), Ok(1.0));

    // 2^24 = 16_777_216 (exact)
    assert_eq!(to_float32_from_i32(16_777_216), Ok(16_777_216.0));
    assert_eq!(to_float32_from_i32(-16_777_216), Ok(-16_777_216.0));
    assert_eq!(to_float32_from_u32(16_777_216), Ok(16_777_216.0));
    assert_eq!(to_float32_from_i64(16_777_216), Ok(16_777_216.0));
    assert_eq!(to_float32_from_u64(16_777_216), Ok(16_777_216.0));
    assert_eq!(to_float32_from_i128(16_777_216), Ok(16_777_216.0));
    assert_eq!(to_float32_from_u128(16_777_216), Ok(16_777_216.0));

    // 2^24 + 1 = 16_777_217 (NotExactlyRepresentable)
    assert_eq!(
        to_float32_from_i32(16_777_217),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_float32_from_i32(-16_777_217),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_float32_from_u32(16_777_217),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_float32_from_i64(16_777_217),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_float32_from_u64(16_777_217),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_float32_from_i128(16_777_217),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_float32_from_u128(16_777_217),
        Err(ConversionFailure::NotExactlyRepresentable)
    );

    // 2^24 + 2 = 16_777_218 (exact because trailing zero)
    assert_eq!(to_float32_from_i32(16_777_218), Ok(16_777_218.0));

    // Large powers of 2 (exact)
    assert_eq!(to_float32_from_i64(1i64 << 30), Ok((1i64 << 30) as f32));
    assert_eq!(to_float32_from_u64(1u64 << 60), Ok((1u64 << 60) as f32));
    assert_eq!(
        to_float32_from_u128(1u128 << 127),
        Ok((1u128 << 127) as f32)
    );
    assert_eq!(to_float32_from_i128(i128::MIN), Ok(i128::MIN as f32));

    // Extremes with lost precision
    assert_eq!(
        to_float32_from_u128(u128::MAX),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_float32_from_i128(i128::MAX),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_float32_from_u64(u64::MAX),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_float32_from_i64(i64::MAX),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
}

// ============================================================================
// 3. Integer -> Float64 Tests (Section 19)
// ============================================================================

#[test]
fn test_integer_to_float64() {
    // 0, 1, -1
    assert_eq!(to_float64_from_i8(0), Ok(0.0));
    assert_eq!(to_float64_from_i8(1), Ok(1.0));
    assert_eq!(to_float64_from_i8(-1), Ok(-1.0));

    // 2^53 = 9_007_199_254_740_992 (exact)
    assert_eq!(
        to_float64_from_i64(9_007_199_254_740_992),
        Ok(9_007_199_254_740_992.0)
    );
    assert_eq!(
        to_float64_from_i64(-9_007_199_254_740_992),
        Ok(-9_007_199_254_740_992.0)
    );
    assert_eq!(
        to_float64_from_u64(9_007_199_254_740_992),
        Ok(9_007_199_254_740_992.0)
    );
    assert_eq!(
        to_float64_from_i128(9_007_199_254_740_992),
        Ok(9_007_199_254_740_992.0)
    );
    assert_eq!(
        to_float64_from_u128(9_007_199_254_740_992),
        Ok(9_007_199_254_740_992.0)
    );

    // 2^53 + 1 = 9_007_199_254_740_993 (NotExactlyRepresentable)
    assert_eq!(
        to_float64_from_i64(9_007_199_254_740_993),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_float64_from_i64(-9_007_199_254_740_993),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_float64_from_u64(9_007_199_254_740_993),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_float64_from_i128(9_007_199_254_740_993),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_float64_from_u128(9_007_199_254_740_993),
        Err(ConversionFailure::NotExactlyRepresentable)
    );

    // 2^53 + 2 = 9_007_199_254_740_994 (exact)
    assert_eq!(
        to_float64_from_i64(9_007_199_254_740_994),
        Ok(9_007_199_254_740_994.0)
    );

    // Large powers of 2 (exact)
    assert_eq!(to_float64_from_u64(1u64 << 60), Ok((1u64 << 60) as f64));
    assert_eq!(
        to_float64_from_u128(1u128 << 100),
        Ok((1u128 << 100) as f64)
    );
    assert_eq!(to_float64_from_i128(i128::MIN), Ok(i128::MIN as f64));

    // Non-representable extremes
    assert_eq!(
        to_float64_from_u128(u128::MAX),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_float64_from_i128(i128::MAX),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
}

// ============================================================================
// 4. Float -> Integer Tests (Section 20)
// ============================================================================

#[test]
fn test_float_to_integer_basic_and_fractional() {
    assert_eq!(to_int32_from_f32(42.0), Ok(42));
    assert_eq!(
        to_int32_from_f32(42.5),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(to_uint32_from_f32(42.0), Ok(42));
    assert_eq!(
        to_uint32_from_f32(42.5),
        Err(ConversionFailure::NotExactlyRepresentable)
    );

    assert_eq!(to_int64_from_f64(42.0), Ok(42));
    assert_eq!(
        to_int64_from_f64(42.5),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
}

#[test]
fn test_float_signed_zero() {
    // +0.0 and -0.0 both represent integer 0
    assert_eq!(to_int32_from_f32(0.0), Ok(0));
    assert_eq!(to_int32_from_f32(-0.0), Ok(0));
    assert_eq!(to_uint32_from_f32(0.0), Ok(0));
    assert_eq!(to_uint32_from_f32(-0.0), Ok(0));

    assert_eq!(to_int64_from_f64(0.0), Ok(0));
    assert_eq!(to_int64_from_f64(-0.0), Ok(0));
    assert_eq!(to_uint64_from_f64(0.0), Ok(0));
    assert_eq!(to_uint64_from_f64(-0.0), Ok(0));
}

#[test]
fn test_float_negative_to_unsigned_fails() {
    assert_eq!(
        to_uint8_from_f32(-1.0),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_uint16_from_f32(-1.0),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_uint32_from_f32(-1.0),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_uint64_from_f64(-1.0),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_uint128_from_f64(-1.0),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
}

#[test]
fn test_float_special_values_fail_to_integer() {
    // f32 special values
    assert_eq!(
        to_int32_from_f32(f32::NAN),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_int32_from_f32(f32::INFINITY),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_int32_from_f32(f32::NEG_INFINITY),
        Err(ConversionFailure::NotExactlyRepresentable)
    );

    assert_eq!(
        to_uint32_from_f32(f32::NAN),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_uint32_from_f32(f32::INFINITY),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_uint32_from_f32(f32::NEG_INFINITY),
        Err(ConversionFailure::NotExactlyRepresentable)
    );

    // f64 special values
    assert_eq!(
        to_int64_from_f64(f64::NAN),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_int64_from_f64(f64::INFINITY),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_int64_from_f64(f64::NEG_INFINITY),
        Err(ConversionFailure::NotExactlyRepresentable)
    );

    assert_eq!(
        to_uint64_from_f64(f64::NAN),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_uint64_from_f64(f64::INFINITY),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_uint64_from_f64(f64::NEG_INFINITY),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
}

#[test]
fn test_float_to_integer_mandatory_boundary_false_positives() {
    // 2147483648.0f32 -> i32 (exceeds i32::MAX = 2147483647)
    assert_eq!(
        to_int32_from_f32(2147483648.0),
        Err(ConversionFailure::NotExactlyRepresentable)
    );

    // -2147483648.0f32 -> i32 (exact i32::MIN)
    assert_eq!(to_int32_from_f32(-2147483648.0), Ok(-2147483648));

    // 4294967296.0f32 -> u32 (exceeds u32::MAX = 4294967295)
    assert_eq!(
        to_uint32_from_f32(4294967296.0),
        Err(ConversionFailure::NotExactlyRepresentable)
    );

    // 9223372036854775808.0f64 -> i64 (exceeds i64::MAX = 9223372036854775807)
    assert_eq!(
        to_int64_from_f64(9223372036854775808.0),
        Err(ConversionFailure::NotExactlyRepresentable)
    );

    // -9223372036854775808.0f64 -> i64 (exact i64::MIN)
    assert_eq!(
        to_int64_from_f64(-9223372036854775808.0),
        Ok(-9223372036854775808)
    );

    // 18446744073709551616.0f64 -> u64 (exceeds u64::MAX = 18446744073709551615)
    assert_eq!(
        to_uint64_from_f64(18446744073709551616.0),
        Err(ConversionFailure::NotExactlyRepresentable)
    );

    // Additional narrow boundaries
    assert_eq!(
        to_int8_from_f32(128.0),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(to_int8_from_f32(-128.0), Ok(-128));
    assert_eq!(
        to_uint8_from_f32(256.0),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(to_uint8_from_f32(255.0), Ok(255));

    assert_eq!(
        to_int16_from_f32(32768.0),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(to_int16_from_f32(-32768.0), Ok(-32768));
    assert_eq!(
        to_uint16_from_f32(65536.0),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(to_uint16_from_f32(65535.0), Ok(65535));
}

// ============================================================================
// 5. Float32 <-> Float64 Tests (Section 21)
// ============================================================================

#[test]
fn test_float32_to_float64() {
    assert_eq!(to_float64_from_f32(1.5), Ok(1.5f64));
    assert_eq!(to_float64_from_f32(0.0), Ok(0.0f64));
    assert_eq!(to_float64_from_f32(f32::INFINITY), Ok(f64::INFINITY));
    assert_eq!(
        to_float64_from_f32(f32::NEG_INFINITY),
        Ok(f64::NEG_INFINITY)
    );

    // Signed zero preservation
    let pos_zero = to_float64_from_f32(0.0f32).unwrap();
    let neg_zero = to_float64_from_f32(-0.0f32).unwrap();
    assert_eq!(pos_zero.to_bits(), 0.0f64.to_bits());
    assert_eq!(neg_zero.to_bits(), (-0.0f64).to_bits());

    // NaN without == comparison
    let nan_res = to_float64_from_f32(f32::NAN);
    assert!(nan_res.is_ok());
    assert!(nan_res.unwrap().is_nan());
}

#[test]
fn test_float64_to_float32() {
    // Exact values
    assert_eq!(to_float32_from_f64(1.5), Ok(1.5f32));
    assert_eq!(to_float32_from_f64(f64::INFINITY), Ok(f32::INFINITY));
    assert_eq!(
        to_float32_from_f64(f64::NEG_INFINITY),
        Ok(f32::NEG_INFINITY)
    );

    // Value requiring precision loss fails
    assert_eq!(
        to_float32_from_f64(1.0 + 1e-11),
        Err(ConversionFailure::NotExactlyRepresentable)
    );

    // Overflow fails
    assert_eq!(
        to_float32_from_f64(f64::MAX),
        Err(ConversionFailure::NotExactlyRepresentable)
    );

    // Signed zero preservation
    let pos_zero = to_float32_from_f64(0.0f64).unwrap();
    let neg_zero = to_float32_from_f64(-0.0f64).unwrap();
    assert_eq!(pos_zero.to_bits(), 0.0f32.to_bits());
    assert_eq!(neg_zero.to_bits(), (-0.0f32).to_bits());

    // NaN without == comparison
    let nan_res = to_float32_from_f64(f64::NAN);
    assert!(nan_res.is_ok());
    assert!(nan_res.unwrap().is_nan());
}

#[test]
fn test_float_identities() {
    assert_eq!(to_float32_from_f32(3.14), Ok(3.14));
    assert_eq!(
        to_float64_from_f64(3.141592653589793),
        Ok(3.141592653589793)
    );

    let nan32 = to_float32_from_f32(f32::NAN).unwrap();
    assert!(nan32.is_nan());

    let nan64 = to_float64_from_f64(f64::NAN).unwrap();
    assert!(nan64.is_nan());
}

// ============================================================================
// 6. Function Pointer Contracts (Section 22)
// ============================================================================

#[test]
fn test_function_pointer_contracts() {
    // Section 22 explicit examples
    let op1: ToInt8<i16> = TO_INT8_FROM_I16;
    assert_eq!(op1(10), Ok(10));
    assert_eq!(op1(200), Err(ConversionFailure::NotExactlyRepresentable));

    let op2: ToUint64<f64> = TO_UINT64_FROM_F64;
    assert_eq!(op2(100.0), Ok(100));
    assert_eq!(op2(-1.0), Err(ConversionFailure::NotExactlyRepresentable));

    let op3: ToFloat32<u128> = TO_FLOAT32_FROM_U128;
    assert_eq!(op3(16_777_216), Ok(16_777_216.0));
    assert_eq!(
        op3(16_777_217),
        Err(ConversionFailure::NotExactlyRepresentable)
    );

    let op4: ToFloat64<f32> = TO_FLOAT64_FROM_F32;
    assert_eq!(op4(1.5), Ok(1.5));

    // Additional contracts across remaining targets
    let op_i16: ToInt16<i8> = TO_INT16_FROM_I8;
    assert_eq!(op_i16(127), Ok(127));

    let op_i32: ToInt32<f32> = TO_INT32_FROM_F32;
    assert_eq!(op_i32(42.0), Ok(42));

    let op_i64: ToInt64<u32> = TO_INT64_FROM_U32;
    assert_eq!(op_i64(5000), Ok(5000));

    let op_i128: ToInt128<f64> = TO_INT128_FROM_F64;
    assert_eq!(op_i128(100.0), Ok(100));

    let op_u8: ToUint8<i32> = TO_UINT8_FROM_I32;
    assert_eq!(op_u8(200), Ok(200));

    let op_u16: ToUint16<u8> = TO_UINT16_FROM_U8;
    assert_eq!(op_u16(255), Ok(255));

    let op_u32: ToUint32<i16> = TO_UINT32_FROM_I16;
    assert_eq!(op_u32(1000), Ok(1000));

    let op_u128: ToUint128<u64> = TO_UINT128_FROM_U64;
    assert_eq!(op_u128(99999), Ok(99999));
}

#[test]
fn test_all_144_concrete_conversions_and_constants() {
    // ToInt8
    let _: ToInt8<i8> = TO_INT8_FROM_I8;
    let _: ToInt8<i16> = TO_INT8_FROM_I16;
    let _: ToInt8<i32> = TO_INT8_FROM_I32;
    let _: ToInt8<i64> = TO_INT8_FROM_I64;
    let _: ToInt8<i128> = TO_INT8_FROM_I128;
    let _: ToInt8<u8> = TO_INT8_FROM_U8;
    let _: ToInt8<u16> = TO_INT8_FROM_U16;
    let _: ToInt8<u32> = TO_INT8_FROM_U32;
    let _: ToInt8<u64> = TO_INT8_FROM_U64;
    let _: ToInt8<u128> = TO_INT8_FROM_U128;
    let _: ToInt8<f32> = TO_INT8_FROM_F32;
    let _: ToInt8<f64> = TO_INT8_FROM_F64;

    assert_eq!(to_int8_from_i8(1), Ok(1));
    assert_eq!(to_int8_from_i16(1), Ok(1));
    assert_eq!(to_int8_from_i32(1), Ok(1));
    assert_eq!(to_int8_from_i64(1), Ok(1));
    assert_eq!(to_int8_from_i128(1), Ok(1));
    assert_eq!(to_int8_from_u8(1), Ok(1));
    assert_eq!(to_int8_from_u16(1), Ok(1));
    assert_eq!(to_int8_from_u32(1), Ok(1));
    assert_eq!(to_int8_from_u64(1), Ok(1));
    assert_eq!(to_int8_from_u128(1), Ok(1));
    assert_eq!(to_int8_from_f32(1.0), Ok(1));
    assert_eq!(to_int8_from_f64(1.0), Ok(1));

    // ToInt16
    let _: ToInt16<i8> = TO_INT16_FROM_I8;
    let _: ToInt16<i16> = TO_INT16_FROM_I16;
    let _: ToInt16<i32> = TO_INT16_FROM_I32;
    let _: ToInt16<i64> = TO_INT16_FROM_I64;
    let _: ToInt16<i128> = TO_INT16_FROM_I128;
    let _: ToInt16<u8> = TO_INT16_FROM_U8;
    let _: ToInt16<u16> = TO_INT16_FROM_U16;
    let _: ToInt16<u32> = TO_INT16_FROM_U32;
    let _: ToInt16<u64> = TO_INT16_FROM_U64;
    let _: ToInt16<u128> = TO_INT16_FROM_U128;
    let _: ToInt16<f32> = TO_INT16_FROM_F32;
    let _: ToInt16<f64> = TO_INT16_FROM_F64;

    assert_eq!(to_int16_from_i8(1), Ok(1));
    assert_eq!(to_int16_from_i16(1), Ok(1));
    assert_eq!(to_int16_from_i32(1), Ok(1));
    assert_eq!(to_int16_from_i64(1), Ok(1));
    assert_eq!(to_int16_from_i128(1), Ok(1));
    assert_eq!(to_int16_from_u8(1), Ok(1));
    assert_eq!(to_int16_from_u16(1), Ok(1));
    assert_eq!(to_int16_from_u32(1), Ok(1));
    assert_eq!(to_int16_from_u64(1), Ok(1));
    assert_eq!(to_int16_from_u128(1), Ok(1));
    assert_eq!(to_int16_from_f32(1.0), Ok(1));
    assert_eq!(to_int16_from_f64(1.0), Ok(1));

    // ToInt32
    let _: ToInt32<i8> = TO_INT32_FROM_I8;
    let _: ToInt32<i16> = TO_INT32_FROM_I16;
    let _: ToInt32<i32> = TO_INT32_FROM_I32;
    let _: ToInt32<i64> = TO_INT32_FROM_I64;
    let _: ToInt32<i128> = TO_INT32_FROM_I128;
    let _: ToInt32<u8> = TO_INT32_FROM_U8;
    let _: ToInt32<u16> = TO_INT32_FROM_U16;
    let _: ToInt32<u32> = TO_INT32_FROM_U32;
    let _: ToInt32<u64> = TO_INT32_FROM_U64;
    let _: ToInt32<u128> = TO_INT32_FROM_U128;
    let _: ToInt32<f32> = TO_INT32_FROM_F32;
    let _: ToInt32<f64> = TO_INT32_FROM_F64;

    assert_eq!(to_int32_from_i8(1), Ok(1));
    assert_eq!(to_int32_from_i16(1), Ok(1));
    assert_eq!(to_int32_from_i32(1), Ok(1));
    assert_eq!(to_int32_from_i64(1), Ok(1));
    assert_eq!(to_int32_from_i128(1), Ok(1));
    assert_eq!(to_int32_from_u8(1), Ok(1));
    assert_eq!(to_int32_from_u16(1), Ok(1));
    assert_eq!(to_int32_from_u32(1), Ok(1));
    assert_eq!(to_int32_from_u64(1), Ok(1));
    assert_eq!(to_int32_from_u128(1), Ok(1));
    assert_eq!(to_int32_from_f32(1.0), Ok(1));
    assert_eq!(to_int32_from_f64(1.0), Ok(1));

    // ToInt64
    let _: ToInt64<i8> = TO_INT64_FROM_I8;
    let _: ToInt64<i16> = TO_INT64_FROM_I16;
    let _: ToInt64<i32> = TO_INT64_FROM_I32;
    let _: ToInt64<i64> = TO_INT64_FROM_I64;
    let _: ToInt64<i128> = TO_INT64_FROM_I128;
    let _: ToInt64<u8> = TO_INT64_FROM_U8;
    let _: ToInt64<u16> = TO_INT64_FROM_U16;
    let _: ToInt64<u32> = TO_INT64_FROM_U32;
    let _: ToInt64<u64> = TO_INT64_FROM_U64;
    let _: ToInt64<u128> = TO_INT64_FROM_U128;
    let _: ToInt64<f32> = TO_INT64_FROM_F32;
    let _: ToInt64<f64> = TO_INT64_FROM_F64;

    assert_eq!(to_int64_from_i8(1), Ok(1));
    assert_eq!(to_int64_from_i16(1), Ok(1));
    assert_eq!(to_int64_from_i32(1), Ok(1));
    assert_eq!(to_int64_from_i64(1), Ok(1));
    assert_eq!(to_int64_from_i128(1), Ok(1));
    assert_eq!(to_int64_from_u8(1), Ok(1));
    assert_eq!(to_int64_from_u16(1), Ok(1));
    assert_eq!(to_int64_from_u32(1), Ok(1));
    assert_eq!(to_int64_from_u64(1), Ok(1));
    assert_eq!(to_int64_from_u128(1), Ok(1));
    assert_eq!(to_int64_from_f32(1.0), Ok(1));
    assert_eq!(to_int64_from_f64(1.0), Ok(1));

    // ToInt128
    let _: ToInt128<i8> = TO_INT128_FROM_I8;
    let _: ToInt128<i16> = TO_INT128_FROM_I16;
    let _: ToInt128<i32> = TO_INT128_FROM_I32;
    let _: ToInt128<i64> = TO_INT128_FROM_I64;
    let _: ToInt128<i128> = TO_INT128_FROM_I128;
    let _: ToInt128<u8> = TO_INT128_FROM_U8;
    let _: ToInt128<u16> = TO_INT128_FROM_U16;
    let _: ToInt128<u32> = TO_INT128_FROM_U32;
    let _: ToInt128<u64> = TO_INT128_FROM_U64;
    let _: ToInt128<u128> = TO_INT128_FROM_U128;
    let _: ToInt128<f32> = TO_INT128_FROM_F32;
    let _: ToInt128<f64> = TO_INT128_FROM_F64;

    assert_eq!(to_int128_from_i8(1), Ok(1));
    assert_eq!(to_int128_from_i16(1), Ok(1));
    assert_eq!(to_int128_from_i32(1), Ok(1));
    assert_eq!(to_int128_from_i64(1), Ok(1));
    assert_eq!(to_int128_from_i128(1), Ok(1));
    assert_eq!(to_int128_from_u8(1), Ok(1));
    assert_eq!(to_int128_from_u16(1), Ok(1));
    assert_eq!(to_int128_from_u32(1), Ok(1));
    assert_eq!(to_int128_from_u64(1), Ok(1));
    assert_eq!(to_int128_from_u128(1), Ok(1));
    assert_eq!(to_int128_from_f32(1.0), Ok(1));
    assert_eq!(to_int128_from_f64(1.0), Ok(1));

    // ToUint8
    let _: ToUint8<i8> = TO_UINT8_FROM_I8;
    let _: ToUint8<i16> = TO_UINT8_FROM_I16;
    let _: ToUint8<i32> = TO_UINT8_FROM_I32;
    let _: ToUint8<i64> = TO_UINT8_FROM_I64;
    let _: ToUint8<i128> = TO_UINT8_FROM_I128;
    let _: ToUint8<u8> = TO_UINT8_FROM_U8;
    let _: ToUint8<u16> = TO_UINT8_FROM_U16;
    let _: ToUint8<u32> = TO_UINT8_FROM_U32;
    let _: ToUint8<u64> = TO_UINT8_FROM_U64;
    let _: ToUint8<u128> = TO_UINT8_FROM_U128;
    let _: ToUint8<f32> = TO_UINT8_FROM_F32;
    let _: ToUint8<f64> = TO_UINT8_FROM_F64;

    assert_eq!(to_uint8_from_i8(1), Ok(1));
    assert_eq!(to_uint8_from_i16(1), Ok(1));
    assert_eq!(to_uint8_from_i32(1), Ok(1));
    assert_eq!(to_uint8_from_i64(1), Ok(1));
    assert_eq!(to_uint8_from_i128(1), Ok(1));
    assert_eq!(to_uint8_from_u8(1), Ok(1));
    assert_eq!(to_uint8_from_u16(1), Ok(1));
    assert_eq!(to_uint8_from_u32(1), Ok(1));
    assert_eq!(to_uint8_from_u64(1), Ok(1));
    assert_eq!(to_uint8_from_u128(1), Ok(1));
    assert_eq!(to_uint8_from_f32(1.0), Ok(1));
    assert_eq!(to_uint8_from_f64(1.0), Ok(1));

    // ToUint16
    let _: ToUint16<i8> = TO_UINT16_FROM_I8;
    let _: ToUint16<i16> = TO_UINT16_FROM_I16;
    let _: ToUint16<i32> = TO_UINT16_FROM_I32;
    let _: ToUint16<i64> = TO_UINT16_FROM_I64;
    let _: ToUint16<i128> = TO_UINT16_FROM_I128;
    let _: ToUint16<u8> = TO_UINT16_FROM_U8;
    let _: ToUint16<u16> = TO_UINT16_FROM_U16;
    let _: ToUint16<u32> = TO_UINT16_FROM_U32;
    let _: ToUint16<u64> = TO_UINT16_FROM_U64;
    let _: ToUint16<u128> = TO_UINT16_FROM_U128;
    let _: ToUint16<f32> = TO_UINT16_FROM_F32;
    let _: ToUint16<f64> = TO_UINT16_FROM_F64;

    assert_eq!(to_uint16_from_i8(1), Ok(1));
    assert_eq!(to_uint16_from_i16(1), Ok(1));
    assert_eq!(to_uint16_from_i32(1), Ok(1));
    assert_eq!(to_uint16_from_i64(1), Ok(1));
    assert_eq!(to_uint16_from_i128(1), Ok(1));
    assert_eq!(to_uint16_from_u8(1), Ok(1));
    assert_eq!(to_uint16_from_u16(1), Ok(1));
    assert_eq!(to_uint16_from_u32(1), Ok(1));
    assert_eq!(to_uint16_from_u64(1), Ok(1));
    assert_eq!(to_uint16_from_u128(1), Ok(1));
    assert_eq!(to_uint16_from_f32(1.0), Ok(1));
    assert_eq!(to_uint16_from_f64(1.0), Ok(1));

    // ToUint32
    let _: ToUint32<i8> = TO_UINT32_FROM_I8;
    let _: ToUint32<i16> = TO_UINT32_FROM_I16;
    let _: ToUint32<i32> = TO_UINT32_FROM_I32;
    let _: ToUint32<i64> = TO_UINT32_FROM_I64;
    let _: ToUint32<i128> = TO_UINT32_FROM_I128;
    let _: ToUint32<u8> = TO_UINT32_FROM_U8;
    let _: ToUint32<u16> = TO_UINT32_FROM_U16;
    let _: ToUint32<u32> = TO_UINT32_FROM_U32;
    let _: ToUint32<u64> = TO_UINT32_FROM_U64;
    let _: ToUint32<u128> = TO_UINT32_FROM_U128;
    let _: ToUint32<f32> = TO_UINT32_FROM_F32;
    let _: ToUint32<f64> = TO_UINT32_FROM_F64;

    assert_eq!(to_uint32_from_i8(1), Ok(1));
    assert_eq!(to_uint32_from_i16(1), Ok(1));
    assert_eq!(to_uint32_from_i32(1), Ok(1));
    assert_eq!(to_uint32_from_i64(1), Ok(1));
    assert_eq!(to_uint32_from_i128(1), Ok(1));
    assert_eq!(to_uint32_from_u8(1), Ok(1));
    assert_eq!(to_uint32_from_u16(1), Ok(1));
    assert_eq!(to_uint32_from_u32(1), Ok(1));
    assert_eq!(to_uint32_from_u64(1), Ok(1));
    assert_eq!(to_uint32_from_u128(1), Ok(1));
    assert_eq!(to_uint32_from_f32(1.0), Ok(1));
    assert_eq!(to_uint32_from_f64(1.0), Ok(1));

    // ToUint64
    let _: ToUint64<i8> = TO_UINT64_FROM_I8;
    let _: ToUint64<i16> = TO_UINT64_FROM_I16;
    let _: ToUint64<i32> = TO_UINT64_FROM_I32;
    let _: ToUint64<i64> = TO_UINT64_FROM_I64;
    let _: ToUint64<i128> = TO_UINT64_FROM_I128;
    let _: ToUint64<u8> = TO_UINT64_FROM_U8;
    let _: ToUint64<u16> = TO_UINT64_FROM_U16;
    let _: ToUint64<u32> = TO_UINT64_FROM_U32;
    let _: ToUint64<u64> = TO_UINT64_FROM_U64;
    let _: ToUint64<u128> = TO_UINT64_FROM_U128;
    let _: ToUint64<f32> = TO_UINT64_FROM_F32;
    let _: ToUint64<f64> = TO_UINT64_FROM_F64;

    assert_eq!(to_uint64_from_i8(1), Ok(1));
    assert_eq!(to_uint64_from_i16(1), Ok(1));
    assert_eq!(to_uint64_from_i32(1), Ok(1));
    assert_eq!(to_uint64_from_i64(1), Ok(1));
    assert_eq!(to_uint64_from_i128(1), Ok(1));
    assert_eq!(to_uint64_from_u8(1), Ok(1));
    assert_eq!(to_uint64_from_u16(1), Ok(1));
    assert_eq!(to_uint64_from_u32(1), Ok(1));
    assert_eq!(to_uint64_from_u64(1), Ok(1));
    assert_eq!(to_uint64_from_u128(1), Ok(1));
    assert_eq!(to_uint64_from_f32(1.0), Ok(1));
    assert_eq!(to_uint64_from_f64(1.0), Ok(1));

    // ToUint128
    let _: ToUint128<i8> = TO_UINT128_FROM_I8;
    let _: ToUint128<i16> = TO_UINT128_FROM_I16;
    let _: ToUint128<i32> = TO_UINT128_FROM_I32;
    let _: ToUint128<i64> = TO_UINT128_FROM_I64;
    let _: ToUint128<i128> = TO_UINT128_FROM_I128;
    let _: ToUint128<u8> = TO_UINT128_FROM_U8;
    let _: ToUint128<u16> = TO_UINT128_FROM_U16;
    let _: ToUint128<u32> = TO_UINT128_FROM_U32;
    let _: ToUint128<u64> = TO_UINT128_FROM_U64;
    let _: ToUint128<u128> = TO_UINT128_FROM_U128;
    let _: ToUint128<f32> = TO_UINT128_FROM_F32;
    let _: ToUint128<f64> = TO_UINT128_FROM_F64;

    assert_eq!(to_uint128_from_i8(1), Ok(1));
    assert_eq!(to_uint128_from_i16(1), Ok(1));
    assert_eq!(to_uint128_from_i32(1), Ok(1));
    assert_eq!(to_uint128_from_i64(1), Ok(1));
    assert_eq!(to_uint128_from_i128(1), Ok(1));
    assert_eq!(to_uint128_from_u8(1), Ok(1));
    assert_eq!(to_uint128_from_u16(1), Ok(1));
    assert_eq!(to_uint128_from_u32(1), Ok(1));
    assert_eq!(to_uint128_from_u64(1), Ok(1));
    assert_eq!(to_uint128_from_u128(1), Ok(1));
    assert_eq!(to_uint128_from_f32(1.0), Ok(1));
    assert_eq!(to_uint128_from_f64(1.0), Ok(1));

    // ToFloat32
    let _: ToFloat32<i8> = TO_FLOAT32_FROM_I8;
    let _: ToFloat32<i16> = TO_FLOAT32_FROM_I16;
    let _: ToFloat32<i32> = TO_FLOAT32_FROM_I32;
    let _: ToFloat32<i64> = TO_FLOAT32_FROM_I64;
    let _: ToFloat32<i128> = TO_FLOAT32_FROM_I128;
    let _: ToFloat32<u8> = TO_FLOAT32_FROM_U8;
    let _: ToFloat32<u16> = TO_FLOAT32_FROM_U16;
    let _: ToFloat32<u32> = TO_FLOAT32_FROM_U32;
    let _: ToFloat32<u64> = TO_FLOAT32_FROM_U64;
    let _: ToFloat32<u128> = TO_FLOAT32_FROM_U128;
    let _: ToFloat32<f32> = TO_FLOAT32_FROM_F32;
    let _: ToFloat32<f64> = TO_FLOAT32_FROM_F64;

    assert_eq!(to_float32_from_i8(1), Ok(1.0));
    assert_eq!(to_float32_from_i16(1), Ok(1.0));
    assert_eq!(to_float32_from_i32(1), Ok(1.0));
    assert_eq!(to_float32_from_i64(1), Ok(1.0));
    assert_eq!(to_float32_from_i128(1), Ok(1.0));
    assert_eq!(to_float32_from_u8(1), Ok(1.0));
    assert_eq!(to_float32_from_u16(1), Ok(1.0));
    assert_eq!(to_float32_from_u32(1), Ok(1.0));
    assert_eq!(to_float32_from_u64(1), Ok(1.0));
    assert_eq!(to_float32_from_u128(1), Ok(1.0));
    assert_eq!(to_float32_from_f32(1.0), Ok(1.0));
    assert_eq!(to_float32_from_f64(1.0), Ok(1.0));

    // ToFloat64
    let _: ToFloat64<i8> = TO_FLOAT64_FROM_I8;
    let _: ToFloat64<i16> = TO_FLOAT64_FROM_I16;
    let _: ToFloat64<i32> = TO_FLOAT64_FROM_I32;
    let _: ToFloat64<i64> = TO_FLOAT64_FROM_I64;
    let _: ToFloat64<i128> = TO_FLOAT64_FROM_I128;
    let _: ToFloat64<u8> = TO_FLOAT64_FROM_U8;
    let _: ToFloat64<u16> = TO_FLOAT64_FROM_U16;
    let _: ToFloat64<u32> = TO_FLOAT64_FROM_U32;
    let _: ToFloat64<u64> = TO_FLOAT64_FROM_U64;
    let _: ToFloat64<u128> = TO_FLOAT64_FROM_U128;
    let _: ToFloat64<f32> = TO_FLOAT64_FROM_F32;
    let _: ToFloat64<f64> = TO_FLOAT64_FROM_F64;

    assert_eq!(to_float64_from_i8(1), Ok(1.0));
    assert_eq!(to_float64_from_i16(1), Ok(1.0));
    assert_eq!(to_float64_from_i32(1), Ok(1.0));
    assert_eq!(to_float64_from_i64(1), Ok(1.0));
    assert_eq!(to_float64_from_i128(1), Ok(1.0));
    assert_eq!(to_float64_from_u8(1), Ok(1.0));
    assert_eq!(to_float64_from_u16(1), Ok(1.0));
    assert_eq!(to_float64_from_u32(1), Ok(1.0));
    assert_eq!(to_float64_from_u64(1), Ok(1.0));
    assert_eq!(to_float64_from_u128(1), Ok(1.0));
    assert_eq!(to_float64_from_f32(1.0), Ok(1.0));
    assert_eq!(to_float64_from_f64(1.0), Ok(1.0));
}

// ============================================================================
// TASK-EV-015 Dynamic Conversion Tests
// ============================================================================

use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::vec::Vec;
use evo_values::definitions::conversion::{
    ToDynamicFloat32, ToDynamicFloat64, ToDynamicInteger, ToFloat32FromDynamic,
    ToFloat32FromOwnedDynamic, ToFloat64FromDynamic, ToFloat64FromOwnedDynamic, ToInt8FromDynamic,
    ToInt8FromOwnedDynamic, ToInt16FromDynamic, ToInt16FromOwnedDynamic, ToInt32FromDynamic,
    ToInt32FromOwnedDynamic, ToInt64FromDynamic, ToInt64FromOwnedDynamic, ToInt128FromDynamic,
    ToInt128FromOwnedDynamic, ToUint8FromDynamic, ToUint8FromOwnedDynamic, ToUint16FromDynamic,
    ToUint16FromOwnedDynamic, ToUint32FromDynamic, ToUint32FromOwnedDynamic, ToUint64FromDynamic,
    ToUint64FromOwnedDynamic, ToUint128FromDynamic, ToUint128FromOwnedDynamic,
};
use evo_values::definitions::value::{
    DynamicIntegerValue, DynamicValue, OwnedDynamicInteger, OwnedDynamicValue,
};

fn make_dyn_int<'a>(negative: bool, magnitude: &'a [u8]) -> (DynamicValue<'a>, OwnedDynamicValue) {
    (
        DynamicValue::Integer(DynamicIntegerValue::from_parts(
            negative,
            Cow::Borrowed(magnitude),
        )),
        OwnedDynamicValue::Integer(OwnedDynamicInteger::from_parts(
            negative,
            Box::from(magnitude),
        )),
    )
}

fn make_dyn_f32<'a>(val: f32) -> (DynamicValue<'a>, OwnedDynamicValue) {
    (DynamicValue::Float32(val), OwnedDynamicValue::Float32(val))
}

fn make_dyn_f64<'a>(val: f64) -> (DynamicValue<'a>, OwnedDynamicValue) {
    (DynamicValue::Float64(val), OwnedDynamicValue::Float64(val))
}

fn power_of_two_mag(k: usize) -> Vec<u8> {
    let mut v = Vec::with_capacity(k / 8 + 1);
    v.push(1u8 << (k % 8));
    v.resize(k / 8 + 1, 0u8);
    v
}

#[test]
fn test_fixed_to_dynamic_integer_all_ten_families() {
    // Zero checks
    let dyn_0_i8 = to_dynamic_integer_from_i8(0);
    match dyn_0_i8 {
        OwnedDynamicValue::Integer(i) => {
            assert!(!i.negative());
            assert_eq!(i.magnitude(), &[]);
        }
        _ => panic!("expected Integer"),
    }
    let dyn_0_u128 = to_dynamic_integer_from_u128(0);
    match dyn_0_u128 {
        OwnedDynamicValue::Integer(i) => {
            assert!(!i.negative());
            assert_eq!(i.magnitude(), &[]);
        }
        _ => panic!("expected Integer"),
    }

    // Positive signed
    let dyn_pos_i32 = to_dynamic_integer_from_i32(42);
    match dyn_pos_i32 {
        OwnedDynamicValue::Integer(i) => {
            assert!(!i.negative());
            assert_eq!(i.magnitude(), &[42]);
        }
        _ => panic!("expected Integer"),
    }

    // Negative signed
    let dyn_neg_i32 = to_dynamic_integer_from_i32(-42);
    match dyn_neg_i32 {
        OwnedDynamicValue::Integer(i) => {
            assert!(i.negative());
            assert_eq!(i.magnitude(), &[42]);
        }
        _ => panic!("expected Integer"),
    }

    // Signed MIN checks (no overflow)
    let dyn_min_i8 = to_dynamic_integer_from_i8(i8::MIN);
    match dyn_min_i8 {
        OwnedDynamicValue::Integer(i) => {
            assert!(i.negative());
            assert_eq!(i.magnitude(), &[128]);
        }
        _ => panic!("expected Integer"),
    }
    let dyn_min_i16 = to_dynamic_integer_from_i16(i16::MIN);
    match dyn_min_i16 {
        OwnedDynamicValue::Integer(i) => {
            assert!(i.negative());
            assert_eq!(i.magnitude(), &[0x80, 0x00]);
        }
        _ => panic!("expected Integer"),
    }
    let dyn_min_i32 = to_dynamic_integer_from_i32(i32::MIN);
    match dyn_min_i32 {
        OwnedDynamicValue::Integer(i) => {
            assert!(i.negative());
            assert_eq!(i.magnitude(), &[0x80, 0x00, 0x00, 0x00]);
        }
        _ => panic!("expected Integer"),
    }
    let dyn_min_i64 = to_dynamic_integer_from_i64(i64::MIN);
    match dyn_min_i64 {
        OwnedDynamicValue::Integer(i) => {
            assert!(i.negative());
            assert_eq!(i.magnitude(), &[0x80, 0, 0, 0, 0, 0, 0, 0]);
        }
        _ => panic!("expected Integer"),
    }
    let dyn_min_i128 = to_dynamic_integer_from_i128(i128::MIN);
    match dyn_min_i128 {
        OwnedDynamicValue::Integer(i) => {
            assert!(i.negative());
            assert_eq!(
                i.magnitude(),
                &[0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            );
        }
        _ => panic!("expected Integer"),
    }

    // Signed MAX
    let dyn_max_i8 = to_dynamic_integer_from_i8(i8::MAX);
    match dyn_max_i8 {
        OwnedDynamicValue::Integer(i) => {
            assert!(!i.negative());
            assert_eq!(i.magnitude(), &[127]);
        }
        _ => panic!("expected Integer"),
    }

    // Unsigned MAX
    let dyn_max_u8 = to_dynamic_integer_from_u8(u8::MAX);
    match dyn_max_u8 {
        OwnedDynamicValue::Integer(i) => {
            assert!(!i.negative());
            assert_eq!(i.magnitude(), &[255]);
        }
        _ => panic!("expected Integer"),
    }
    let dyn_max_u16 = to_dynamic_integer_from_u16(u16::MAX);
    match dyn_max_u16 {
        OwnedDynamicValue::Integer(i) => {
            assert!(!i.negative());
            assert_eq!(i.magnitude(), &[255, 255]);
        }
        _ => panic!("expected Integer"),
    }
    let dyn_max_u32 = to_dynamic_integer_from_u32(u32::MAX);
    match dyn_max_u32 {
        OwnedDynamicValue::Integer(i) => {
            assert!(!i.negative());
            assert_eq!(i.magnitude(), &[255, 255, 255, 255]);
        }
        _ => panic!("expected Integer"),
    }
    let dyn_max_u64 = to_dynamic_integer_from_u64(u64::MAX);
    match dyn_max_u64 {
        OwnedDynamicValue::Integer(i) => {
            assert!(!i.negative());
            assert_eq!(i.magnitude(), &[255; 8]);
        }
        _ => panic!("expected Integer"),
    }
    let dyn_max_u128 = to_dynamic_integer_from_u128(u128::MAX);
    match dyn_max_u128 {
        OwnedDynamicValue::Integer(i) => {
            assert!(!i.negative());
            assert_eq!(i.magnitude(), &[255; 16]);
        }
        _ => panic!("expected Integer"),
    }
}

#[test]
fn test_fixed_to_dynamic_float() {
    // f32
    match to_dynamic_float32_from_f32(42.5) {
        OwnedDynamicValue::Float32(f) => assert_eq!(f, 42.5),
        _ => panic!("expected Float32"),
    }
    match to_dynamic_float32_from_f32(f32::NAN) {
        OwnedDynamicValue::Float32(f) => assert!(f.is_nan()),
        _ => panic!("expected Float32"),
    }
    match to_dynamic_float32_from_f32(f32::INFINITY) {
        OwnedDynamicValue::Float32(f) => assert_eq!(f, f32::INFINITY),
        _ => panic!("expected Float32"),
    }
    match to_dynamic_float32_from_f32(f32::NEG_INFINITY) {
        OwnedDynamicValue::Float32(f) => assert_eq!(f, f32::NEG_INFINITY),
        _ => panic!("expected Float32"),
    }
    match to_dynamic_float32_from_f32(0.0f32) {
        OwnedDynamicValue::Float32(f) => assert_eq!(f.to_bits(), 0.0f32.to_bits()),
        _ => panic!("expected Float32"),
    }
    match to_dynamic_float32_from_f32(-0.0f32) {
        OwnedDynamicValue::Float32(f) => assert_eq!(f.to_bits(), (-0.0f32).to_bits()),
        _ => panic!("expected Float32"),
    }

    // f64
    match to_dynamic_float64_from_f64(42.5) {
        OwnedDynamicValue::Float64(f) => assert_eq!(f, 42.5),
        _ => panic!("expected Float64"),
    }
    match to_dynamic_float64_from_f64(f64::NAN) {
        OwnedDynamicValue::Float64(f) => assert!(f.is_nan()),
        _ => panic!("expected Float64"),
    }
    match to_dynamic_float64_from_f64(f64::INFINITY) {
        OwnedDynamicValue::Float64(f) => assert_eq!(f, f64::INFINITY),
        _ => panic!("expected Float64"),
    }
    match to_dynamic_float64_from_f64(f64::NEG_INFINITY) {
        OwnedDynamicValue::Float64(f) => assert_eq!(f, f64::NEG_INFINITY),
        _ => panic!("expected Float64"),
    }
    match to_dynamic_float64_from_f64(0.0f64) {
        OwnedDynamicValue::Float64(f) => assert_eq!(f.to_bits(), 0.0f64.to_bits()),
        _ => panic!("expected Float64"),
    }
    match to_dynamic_float64_from_f64(-0.0f64) {
        OwnedDynamicValue::Float64(f) => assert_eq!(f.to_bits(), (-0.0f64).to_bits()),
        _ => panic!("expected Float64"),
    }
}

#[test]
fn test_dynamic_integer_to_fixed_integers() {
    // Zero to all
    let (b_zero, o_zero) = make_dyn_int(false, &[]);
    assert_eq!(to_int8_from_dynamic(&b_zero), Ok(0));
    assert_eq!(to_int8_from_owned_dynamic(&o_zero), Ok(0));
    assert_eq!(to_int16_from_dynamic(&b_zero), Ok(0));
    assert_eq!(to_int16_from_owned_dynamic(&o_zero), Ok(0));
    assert_eq!(to_int32_from_dynamic(&b_zero), Ok(0));
    assert_eq!(to_int32_from_owned_dynamic(&o_zero), Ok(0));
    assert_eq!(to_int64_from_dynamic(&b_zero), Ok(0));
    assert_eq!(to_int64_from_owned_dynamic(&o_zero), Ok(0));
    assert_eq!(to_int128_from_dynamic(&b_zero), Ok(0));
    assert_eq!(to_int128_from_owned_dynamic(&o_zero), Ok(0));

    assert_eq!(to_uint8_from_dynamic(&b_zero), Ok(0));
    assert_eq!(to_uint8_from_owned_dynamic(&o_zero), Ok(0));
    assert_eq!(to_uint16_from_dynamic(&b_zero), Ok(0));
    assert_eq!(to_uint16_from_owned_dynamic(&o_zero), Ok(0));
    assert_eq!(to_uint32_from_dynamic(&b_zero), Ok(0));
    assert_eq!(to_uint32_from_owned_dynamic(&o_zero), Ok(0));
    assert_eq!(to_uint64_from_dynamic(&b_zero), Ok(0));
    assert_eq!(to_uint64_from_owned_dynamic(&o_zero), Ok(0));
    assert_eq!(to_uint128_from_dynamic(&b_zero), Ok(0));
    assert_eq!(to_uint128_from_owned_dynamic(&o_zero), Ok(0));

    // Section 19 explicit examples:
    // Dynamic 127 -> i8: Ok(127)
    let (b127, o127) = make_dyn_int(false, &[127]);
    assert_eq!(to_int8_from_dynamic(&b127), Ok(127));
    assert_eq!(to_int8_from_owned_dynamic(&o127), Ok(127));

    // Dynamic 128 -> i8: NotExactlyRepresentable
    let (b128, o128) = make_dyn_int(false, &[128]);
    assert_eq!(
        to_int8_from_dynamic(&b128),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_int8_from_owned_dynamic(&o128),
        Err(ConversionFailure::NotExactlyRepresentable)
    );

    // Dynamic -128 -> i8: Ok(-128)
    let (bn128, on128) = make_dyn_int(true, &[128]);
    assert_eq!(to_int8_from_dynamic(&bn128), Ok(-128));
    assert_eq!(to_int8_from_owned_dynamic(&on128), Ok(-128));

    // Dynamic -129 -> i8: NotExactlyRepresentable (magnitude 129 is [0x00, 0x81] or [0x81])
    let (bn129, on129) = make_dyn_int(true, &[129]);
    assert_eq!(
        to_int8_from_dynamic(&bn129),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_int8_from_owned_dynamic(&on129),
        Err(ConversionFailure::NotExactlyRepresentable)
    );

    // Dynamic -1 -> u8: NotExactlyRepresentable
    let (bn1, on1) = make_dyn_int(true, &[1]);
    assert_eq!(
        to_uint8_from_dynamic(&bn1),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_uint8_from_owned_dynamic(&on1),
        Err(ConversionFailure::NotExactlyRepresentable)
    );

    // Negative unsigned failure across all unsigned
    assert_eq!(
        to_uint16_from_dynamic(&bn1),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_uint32_from_dynamic(&bn1),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_uint64_from_dynamic(&bn1),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_uint128_from_dynamic(&bn1),
        Err(ConversionFailure::NotExactlyRepresentable)
    );

    // Exact MIN/MAX for remaining types
    let (b_min_i16, o_min_i16) = make_dyn_int(true, &[0x80, 0x00]);
    assert_eq!(to_int16_from_dynamic(&b_min_i16), Ok(-32768));
    assert_eq!(to_int16_from_owned_dynamic(&o_min_i16), Ok(-32768));

    let (b_max_i16, o_max_i16) = make_dyn_int(false, &[0x7F, 0xFF]);
    assert_eq!(to_int16_from_dynamic(&b_max_i16), Ok(32767));
    assert_eq!(to_int16_from_owned_dynamic(&o_max_i16), Ok(32767));

    let (b_oob_i16, o_oob_i16) = make_dyn_int(false, &[0x80, 0x00]);
    assert_eq!(
        to_int16_from_dynamic(&b_oob_i16),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_int16_from_owned_dynamic(&o_oob_i16),
        Err(ConversionFailure::NotExactlyRepresentable)
    );

    let (b_min_i32, o_min_i32) = make_dyn_int(true, &[0x80, 0, 0, 0]);
    assert_eq!(to_int32_from_dynamic(&b_min_i32), Ok(i32::MIN));
    assert_eq!(to_int32_from_owned_dynamic(&o_min_i32), Ok(i32::MIN));

    let (b_max_i32, o_max_i32) = make_dyn_int(false, &[0x7F, 0xFF, 0xFF, 0xFF]);
    assert_eq!(to_int32_from_dynamic(&b_max_i32), Ok(i32::MAX));
    assert_eq!(to_int32_from_owned_dynamic(&o_max_i32), Ok(i32::MAX));

    let (b_min_i64, o_min_i64) = make_dyn_int(true, &[0x80, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(to_int64_from_dynamic(&b_min_i64), Ok(i64::MIN));
    assert_eq!(to_int64_from_owned_dynamic(&o_min_i64), Ok(i64::MIN));

    let (b_min_i128, o_min_i128) =
        make_dyn_int(true, &[0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(to_int128_from_dynamic(&b_min_i128), Ok(i128::MIN));
    assert_eq!(to_int128_from_owned_dynamic(&o_min_i128), Ok(i128::MIN));

    let (b_max_u128, o_max_u128) = make_dyn_int(false, &[0xFF; 16]);
    assert_eq!(to_uint128_from_dynamic(&b_max_u128), Ok(u128::MAX));
    assert_eq!(to_uint128_from_owned_dynamic(&o_max_u128), Ok(u128::MAX));

    // Arbitrary magnitude > u128 (Section 20): 17 bytes, 32 bytes
    let large_17 = [1u8; 17];
    let (b_l17, o_l17) = make_dyn_int(false, &large_17);
    assert_eq!(
        to_int8_from_dynamic(&b_l17),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_int8_from_owned_dynamic(&o_l17),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_int16_from_dynamic(&b_l17),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_int32_from_dynamic(&b_l17),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_int64_from_dynamic(&b_l17),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_int128_from_dynamic(&b_l17),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_uint8_from_dynamic(&b_l17),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_uint16_from_dynamic(&b_l17),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_uint32_from_dynamic(&b_l17),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_uint64_from_dynamic(&b_l17),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_uint128_from_dynamic(&b_l17),
        Err(ConversionFailure::NotExactlyRepresentable)
    );

    let large_32 = [0xFF; 32];
    let (b_l32, o_l32) = make_dyn_int(false, &large_32);
    assert_eq!(
        to_uint128_from_dynamic(&b_l32),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_uint128_from_owned_dynamic(&o_l32),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
}

#[test]
fn test_dynamic_integer_to_float32_exactness_section_23() {
    // 0 -> 0.0 exact
    let (b0, o0) = make_dyn_int(false, &[]);
    assert_eq!(to_float32_from_dynamic(&b0), Ok(0.0f32));
    assert_eq!(to_float32_from_owned_dynamic(&o0), Ok(0.0f32));

    // 2^24 -> exact
    let mag_2_24 = power_of_two_mag(24);
    let (b24, o24) = make_dyn_int(false, &mag_2_24);
    assert_eq!(to_float32_from_dynamic(&b24), Ok(16777216.0f32));
    assert_eq!(to_float32_from_owned_dynamic(&o24), Ok(16777216.0f32));

    // Negative 2^24 -> exact
    let (bn24, on24) = make_dyn_int(true, &mag_2_24);
    assert_eq!(to_float32_from_dynamic(&bn24), Ok(-16777216.0f32));
    assert_eq!(to_float32_from_owned_dynamic(&on24), Ok(-16777216.0f32));

    // 2^24 + 1 -> NotExactlyRepresentable
    let mut mag_2_24_plus_1 = mag_2_24.clone();
    *mag_2_24_plus_1.last_mut().unwrap() = 1;
    let (b24_p1, o24_p1) = make_dyn_int(false, &mag_2_24_plus_1);
    assert_eq!(
        to_float32_from_dynamic(&b24_p1),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_float32_from_owned_dynamic(&o24_p1),
        Err(ConversionFailure::NotExactlyRepresentable)
    );

    // 2^120 -> exact
    let mag_2_120 = power_of_two_mag(120);
    let (b120, o120) = make_dyn_int(false, &mag_2_120);
    let expected_f32_120 = f32::from_bits((120u32 + 127) << 23);
    assert_eq!(to_float32_from_dynamic(&b120), Ok(expected_f32_120));
    assert_eq!(to_float32_from_owned_dynamic(&o120), Ok(expected_f32_120));

    // 2^127 -> exact
    let mag_2_127 = power_of_two_mag(127);
    let (b127, o127) = make_dyn_int(false, &mag_2_127);
    let expected_f32_127 = f32::from_bits((127u32 + 127) << 23);
    assert_eq!(to_float32_from_dynamic(&b127), Ok(expected_f32_127));
    assert_eq!(to_float32_from_owned_dynamic(&o127), Ok(expected_f32_127));

    // Negative 2^127 -> exact
    let (bn127, on127) = make_dyn_int(true, &mag_2_127);
    let expected_f32_n127 = f32::from_bits((1u32 << 31) | ((127u32 + 127) << 23));
    assert_eq!(to_float32_from_dynamic(&bn127), Ok(expected_f32_n127));
    assert_eq!(to_float32_from_owned_dynamic(&on127), Ok(expected_f32_n127));

    // 2^128 -> NotExactlyRepresentable
    let mag_2_128 = power_of_two_mag(128);
    let (b128, o128) = make_dyn_int(false, &mag_2_128);
    assert_eq!(
        to_float32_from_dynamic(&b128),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_float32_from_owned_dynamic(&o128),
        Err(ConversionFailure::NotExactlyRepresentable)
    );

    // Negative 2^128 -> NotExactlyRepresentable
    let (bn128, on128) = make_dyn_int(true, &mag_2_128);
    assert_eq!(
        to_float32_from_dynamic(&bn128),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_float32_from_owned_dynamic(&on128),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
}

#[test]
fn test_dynamic_integer_to_float64_exactness_section_24() {
    // 0 -> 0.0 exact
    let (b0, o0) = make_dyn_int(false, &[]);
    assert_eq!(to_float64_from_dynamic(&b0), Ok(0.0f64));
    assert_eq!(to_float64_from_owned_dynamic(&o0), Ok(0.0f64));

    // 2^53 -> exact
    let mag_2_53 = power_of_two_mag(53);
    let (b53, o53) = make_dyn_int(false, &mag_2_53);
    assert_eq!(to_float64_from_dynamic(&b53), Ok(9007199254740992.0f64));
    assert_eq!(
        to_float64_from_owned_dynamic(&o53),
        Ok(9007199254740992.0f64)
    );

    // Negative 2^53 -> exact
    let (bn53, on53) = make_dyn_int(true, &mag_2_53);
    assert_eq!(to_float64_from_dynamic(&bn53), Ok(-9007199254740992.0f64));
    assert_eq!(
        to_float64_from_owned_dynamic(&on53),
        Ok(-9007199254740992.0f64)
    );

    // 2^53 + 1 -> NotExactlyRepresentable
    let mut mag_2_53_plus_1 = mag_2_53.clone();
    *mag_2_53_plus_1.last_mut().unwrap() = 1;
    let (b53_p1, o53_p1) = make_dyn_int(false, &mag_2_53_plus_1);
    assert_eq!(
        to_float64_from_dynamic(&b53_p1),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_float64_from_owned_dynamic(&o53_p1),
        Err(ConversionFailure::NotExactlyRepresentable)
    );

    // 2^200 -> exact (> u128)
    let mag_2_200 = power_of_two_mag(200);
    let (b200, o200) = make_dyn_int(false, &mag_2_200);
    let expected_f64_200 = f64::from_bits((200u64 + 1023) << 52);
    assert_eq!(to_float64_from_dynamic(&b200), Ok(expected_f64_200));
    assert_eq!(to_float64_from_owned_dynamic(&o200), Ok(expected_f64_200));

    // Negative 2^200 -> exact
    let (bn200, on200) = make_dyn_int(true, &mag_2_200);
    let expected_f64_n200 = f64::from_bits((1u64 << 63) | ((200u64 + 1023) << 52));
    assert_eq!(to_float64_from_dynamic(&bn200), Ok(expected_f64_n200));
    assert_eq!(to_float64_from_owned_dynamic(&on200), Ok(expected_f64_n200));

    // 2^1023 -> exact
    let mag_2_1023 = power_of_two_mag(1023);
    let (b1023, o1023) = make_dyn_int(false, &mag_2_1023);
    let expected_f64_1023 = f64::from_bits((1023u64 + 1023) << 52);
    assert_eq!(to_float64_from_dynamic(&b1023), Ok(expected_f64_1023));
    assert_eq!(to_float64_from_owned_dynamic(&o1023), Ok(expected_f64_1023));

    // 2^1024 -> NotExactlyRepresentable
    let mag_2_1024 = power_of_two_mag(1024);
    let (b1024, o1024) = make_dyn_int(false, &mag_2_1024);
    assert_eq!(
        to_float64_from_dynamic(&b1024),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_float64_from_owned_dynamic(&o1024),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
}

#[test]
fn test_dynamic_float_to_fixed_targets() {
    // Exact integer from Float32
    let (b42_f32, o42_f32) = make_dyn_f32(42.0);
    assert_eq!(to_int32_from_dynamic(&b42_f32), Ok(42));
    assert_eq!(to_int32_from_owned_dynamic(&o42_f32), Ok(42));
    assert_eq!(to_uint32_from_dynamic(&b42_f32), Ok(42));
    assert_eq!(to_uint32_from_owned_dynamic(&o42_f32), Ok(42));
    assert_eq!(to_float32_from_dynamic(&b42_f32), Ok(42.0f32));
    assert_eq!(to_float32_from_owned_dynamic(&o42_f32), Ok(42.0f32));
    assert_eq!(to_float64_from_dynamic(&b42_f32), Ok(42.0f64));
    assert_eq!(to_float64_from_owned_dynamic(&o42_f32), Ok(42.0f64));

    // Fractional from Float32
    let (b_frac_f32, o_frac_f32) = make_dyn_f32(42.5);
    assert_eq!(
        to_int32_from_dynamic(&b_frac_f32),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_int32_from_owned_dynamic(&o_frac_f32),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(to_float32_from_dynamic(&b_frac_f32), Ok(42.5f32));
    assert_eq!(to_float64_from_dynamic(&b_frac_f32), Ok(42.5f64));

    // Special values Float32
    let (b_nan_f32, o_nan_f32) = make_dyn_f32(f32::NAN);
    assert_eq!(
        to_int32_from_dynamic(&b_nan_f32),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert!(to_float32_from_dynamic(&b_nan_f32).unwrap().is_nan());
    assert!(to_float32_from_owned_dynamic(&o_nan_f32).unwrap().is_nan());
    assert!(to_float64_from_dynamic(&b_nan_f32).unwrap().is_nan());
    assert!(to_float64_from_owned_dynamic(&o_nan_f32).unwrap().is_nan());

    let (b_inf_f32, o_inf_f32) = make_dyn_f32(f32::INFINITY);
    assert_eq!(
        to_int32_from_dynamic(&b_inf_f32),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_int32_from_owned_dynamic(&o_inf_f32),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(to_float32_from_dynamic(&b_inf_f32), Ok(f32::INFINITY));
    assert_eq!(to_float32_from_owned_dynamic(&o_inf_f32), Ok(f32::INFINITY));
    assert_eq!(to_float64_from_dynamic(&b_inf_f32), Ok(f64::INFINITY));
    assert_eq!(to_float64_from_owned_dynamic(&o_inf_f32), Ok(f64::INFINITY));

    let (b_ninf_f32, o_ninf_f32) = make_dyn_f32(f32::NEG_INFINITY);
    assert_eq!(to_float32_from_dynamic(&b_ninf_f32), Ok(f32::NEG_INFINITY));
    assert_eq!(
        to_float32_from_owned_dynamic(&o_ninf_f32),
        Ok(f32::NEG_INFINITY)
    );
    assert_eq!(to_float64_from_dynamic(&b_ninf_f32), Ok(f64::NEG_INFINITY));
    assert_eq!(
        to_float64_from_owned_dynamic(&o_ninf_f32),
        Ok(f64::NEG_INFINITY)
    );

    // Signed zeros Float32
    let (bp0_f32, op0_f32) = make_dyn_f32(0.0f32);
    assert_eq!(to_int32_from_dynamic(&bp0_f32), Ok(0));
    assert_eq!(to_int32_from_owned_dynamic(&op0_f32), Ok(0));
    assert_eq!(to_uint32_from_dynamic(&bp0_f32), Ok(0));
    assert_eq!(to_uint32_from_owned_dynamic(&op0_f32), Ok(0));
    assert_eq!(
        to_float32_from_dynamic(&bp0_f32).unwrap().to_bits(),
        0.0f32.to_bits()
    );
    assert_eq!(
        to_float32_from_owned_dynamic(&op0_f32).unwrap().to_bits(),
        0.0f32.to_bits()
    );
    assert_eq!(
        to_float64_from_dynamic(&bp0_f32).unwrap().to_bits(),
        0.0f64.to_bits()
    );
    assert_eq!(
        to_float64_from_owned_dynamic(&op0_f32).unwrap().to_bits(),
        0.0f64.to_bits()
    );

    let (bn0_f32, on0_f32) = make_dyn_f32(-0.0f32);
    assert_eq!(to_int32_from_dynamic(&bn0_f32), Ok(0));
    assert_eq!(to_int32_from_owned_dynamic(&on0_f32), Ok(0));
    assert_eq!(to_uint32_from_dynamic(&bn0_f32), Ok(0));
    assert_eq!(to_uint32_from_owned_dynamic(&on0_f32), Ok(0));
    assert_eq!(
        to_float32_from_dynamic(&bn0_f32).unwrap().to_bits(),
        (-0.0f32).to_bits()
    );
    assert_eq!(
        to_float32_from_owned_dynamic(&on0_f32).unwrap().to_bits(),
        (-0.0f32).to_bits()
    );
    assert_eq!(
        to_float64_from_dynamic(&bn0_f32).unwrap().to_bits(),
        (-0.0f64).to_bits()
    );
    assert_eq!(
        to_float64_from_owned_dynamic(&on0_f32).unwrap().to_bits(),
        (-0.0f64).to_bits()
    );

    // Float64
    let (b42_f64, o42_f64) = make_dyn_f64(42.0);
    assert_eq!(to_int32_from_dynamic(&b42_f64), Ok(42));
    assert_eq!(to_int32_from_owned_dynamic(&o42_f64), Ok(42));
    assert_eq!(to_float32_from_dynamic(&b42_f64), Ok(42.0f32));
    assert_eq!(to_float64_from_dynamic(&b42_f64), Ok(42.0f64));

    // Float64 -> Float32 exact and inexact
    let (b_exact_f64, o_exact_f64) = make_dyn_f64(1.5f64);
    assert_eq!(to_float32_from_dynamic(&b_exact_f64), Ok(1.5f32));
    assert_eq!(to_float32_from_owned_dynamic(&o_exact_f64), Ok(1.5f32));

    let (b_inexact_f64, o_inexact_f64) = make_dyn_f64(1.0f64 + 1e-12f64);
    assert_eq!(
        to_float32_from_dynamic(&b_inexact_f64),
        Err(ConversionFailure::NotExactlyRepresentable)
    );
    assert_eq!(
        to_float32_from_owned_dynamic(&o_inexact_f64),
        Err(ConversionFailure::NotExactlyRepresentable)
    );

    // Special values Float64
    let (b_nan_f64, o_nan_f64) = make_dyn_f64(f64::NAN);
    assert!(to_float32_from_dynamic(&b_nan_f64).unwrap().is_nan());
    assert!(to_float32_from_owned_dynamic(&o_nan_f64).unwrap().is_nan());
    assert!(to_float64_from_dynamic(&b_nan_f64).unwrap().is_nan());
    assert!(to_float64_from_owned_dynamic(&o_nan_f64).unwrap().is_nan());

    let (b_inf_f64, o_inf_f64) = make_dyn_f64(f64::INFINITY);
    assert_eq!(to_float32_from_dynamic(&b_inf_f64), Ok(f32::INFINITY));
    assert_eq!(to_float32_from_owned_dynamic(&o_inf_f64), Ok(f32::INFINITY));
    assert_eq!(to_float64_from_dynamic(&b_inf_f64), Ok(f64::INFINITY));
    assert_eq!(to_float64_from_owned_dynamic(&o_inf_f64), Ok(f64::INFINITY));

    let (b_ninf_f64, o_ninf_f64) = make_dyn_f64(f64::NEG_INFINITY);
    assert_eq!(to_float32_from_dynamic(&b_ninf_f64), Ok(f32::NEG_INFINITY));
    assert_eq!(
        to_float32_from_owned_dynamic(&o_ninf_f64),
        Ok(f32::NEG_INFINITY)
    );
    assert_eq!(to_float64_from_dynamic(&b_ninf_f64), Ok(f64::NEG_INFINITY));
    assert_eq!(
        to_float64_from_owned_dynamic(&o_ninf_f64),
        Ok(f64::NEG_INFINITY)
    );

    let (bp0_f64, op0_f64) = make_dyn_f64(0.0f64);
    assert_eq!(
        to_float32_from_dynamic(&bp0_f64).unwrap().to_bits(),
        0.0f32.to_bits()
    );
    assert_eq!(
        to_float32_from_owned_dynamic(&op0_f64).unwrap().to_bits(),
        0.0f32.to_bits()
    );
    assert_eq!(
        to_float64_from_dynamic(&bp0_f64).unwrap().to_bits(),
        0.0f64.to_bits()
    );
    assert_eq!(
        to_float64_from_owned_dynamic(&op0_f64).unwrap().to_bits(),
        0.0f64.to_bits()
    );

    let (bn0_f64, on0_f64) = make_dyn_f64(-0.0f64);
    assert_eq!(
        to_float32_from_dynamic(&bn0_f64).unwrap().to_bits(),
        (-0.0f32).to_bits()
    );
    assert_eq!(
        to_float32_from_owned_dynamic(&on0_f64).unwrap().to_bits(),
        (-0.0f32).to_bits()
    );
    assert_eq!(
        to_float64_from_dynamic(&bn0_f64).unwrap().to_bits(),
        (-0.0f64).to_bits()
    );
    assert_eq!(
        to_float64_from_owned_dynamic(&on0_f64).unwrap().to_bits(),
        (-0.0f64).to_bits()
    );
}

#[test]
fn test_borrowed_owned_parity_matrix() {
    let test_values: Vec<(DynamicValue<'_>, OwnedDynamicValue)> = alloc::vec![
        make_dyn_int(false, &[]),
        make_dyn_int(false, &[0]),
        make_dyn_int(false, &[1]),
        make_dyn_int(true, &[1]),
        make_dyn_int(false, &[127]),
        make_dyn_int(false, &[128]),
        make_dyn_int(true, &[128]),
        make_dyn_int(true, &[129]),
        make_dyn_int(false, &[255]),
        make_dyn_int(false, &[1, 0]),
        make_dyn_f32(0.0),
        make_dyn_f32(-0.0),
        make_dyn_f32(42.0),
        make_dyn_f32(-42.0),
        make_dyn_f32(1.5),
        make_dyn_f32(f32::INFINITY),
        make_dyn_f32(f32::NEG_INFINITY),
        make_dyn_f64(0.0),
        make_dyn_f64(-0.0),
        make_dyn_f64(42.0),
        make_dyn_f64(-42.0),
        make_dyn_f64(1.5),
        make_dyn_f64(f64::INFINITY),
        make_dyn_f64(f64::NEG_INFINITY),
    ];

    for (b, o) in &test_values {
        assert_eq!(to_int8_from_dynamic(b), to_int8_from_owned_dynamic(o));
        assert_eq!(to_int16_from_dynamic(b), to_int16_from_owned_dynamic(o));
        assert_eq!(to_int32_from_dynamic(b), to_int32_from_owned_dynamic(o));
        assert_eq!(to_int64_from_dynamic(b), to_int64_from_owned_dynamic(o));
        assert_eq!(to_int128_from_dynamic(b), to_int128_from_owned_dynamic(o));
        assert_eq!(to_uint8_from_dynamic(b), to_uint8_from_owned_dynamic(o));
        assert_eq!(to_uint16_from_dynamic(b), to_uint16_from_owned_dynamic(o));
        assert_eq!(to_uint32_from_dynamic(b), to_uint32_from_owned_dynamic(o));
        assert_eq!(to_uint64_from_dynamic(b), to_uint64_from_owned_dynamic(o));
        assert_eq!(to_uint128_from_dynamic(b), to_uint128_from_owned_dynamic(o));
        assert_eq!(
            to_float32_from_dynamic(b).map(|f| f.to_bits()),
            to_float32_from_owned_dynamic(o).map(|f| f.to_bits())
        );
        assert_eq!(
            to_float64_from_dynamic(b).map(|f| f.to_bits()),
            to_float64_from_owned_dynamic(o).map(|f| f.to_bits())
        );
    }
}

#[test]
fn test_dynamic_function_pointer_contracts() {
    // Fixed -> Dynamic Integer
    let op_i8: ToDynamicInteger<i8> = TO_DYNAMIC_INTEGER_FROM_I8;
    let op_i16: ToDynamicInteger<i16> = TO_DYNAMIC_INTEGER_FROM_I16;
    let op_i32: ToDynamicInteger<i32> = TO_DYNAMIC_INTEGER_FROM_I32;
    let op_i64: ToDynamicInteger<i64> = TO_DYNAMIC_INTEGER_FROM_I64;
    let op_i128: ToDynamicInteger<i128> = TO_DYNAMIC_INTEGER_FROM_I128;
    let op_u8: ToDynamicInteger<u8> = TO_DYNAMIC_INTEGER_FROM_U8;
    let op_u16: ToDynamicInteger<u16> = TO_DYNAMIC_INTEGER_FROM_U16;
    let op_u32: ToDynamicInteger<u32> = TO_DYNAMIC_INTEGER_FROM_U32;
    let op_u64: ToDynamicInteger<u64> = TO_DYNAMIC_INTEGER_FROM_U64;
    let op_u128: ToDynamicInteger<u128> = TO_DYNAMIC_INTEGER_FROM_U128;

    assert!(matches!(op_i8(1), OwnedDynamicValue::Integer(_)));
    assert!(matches!(op_i16(1), OwnedDynamicValue::Integer(_)));
    assert!(matches!(op_i32(1), OwnedDynamicValue::Integer(_)));
    assert!(matches!(op_i64(1), OwnedDynamicValue::Integer(_)));
    assert!(matches!(op_i128(1), OwnedDynamicValue::Integer(_)));
    assert!(matches!(op_u8(1), OwnedDynamicValue::Integer(_)));
    assert!(matches!(op_u16(1), OwnedDynamicValue::Integer(_)));
    assert!(matches!(op_u32(1), OwnedDynamicValue::Integer(_)));
    assert!(matches!(op_u64(1), OwnedDynamicValue::Integer(_)));
    assert!(matches!(op_u128(1), OwnedDynamicValue::Integer(_)));

    // Fixed -> Dynamic Float
    let op_f32: ToDynamicFloat32 = TO_DYNAMIC_FLOAT32_FROM_F32;
    let op_f64: ToDynamicFloat64 = TO_DYNAMIC_FLOAT64_FROM_F64;

    assert!(matches!(op_f32(1.0), OwnedDynamicValue::Float32(_)));
    assert!(matches!(op_f64(1.0), OwnedDynamicValue::Float64(_)));

    // Dynamic -> Fixed Borrowed
    let b_i8: ToInt8FromDynamic = TO_INT8_FROM_DYNAMIC;
    let b_i16: ToInt16FromDynamic = TO_INT16_FROM_DYNAMIC;
    let b_i32: ToInt32FromDynamic = TO_INT32_FROM_DYNAMIC;
    let b_i64: ToInt64FromDynamic = TO_INT64_FROM_DYNAMIC;
    let b_i128: ToInt128FromDynamic = TO_INT128_FROM_DYNAMIC;
    let b_u8: ToUint8FromDynamic = TO_UINT8_FROM_DYNAMIC;
    let b_u16: ToUint16FromDynamic = TO_UINT16_FROM_DYNAMIC;
    let b_u32: ToUint32FromDynamic = TO_UINT32_FROM_DYNAMIC;
    let b_u64: ToUint64FromDynamic = TO_UINT64_FROM_DYNAMIC;
    let b_u128: ToUint128FromDynamic = TO_UINT128_FROM_DYNAMIC;
    let b_f32: ToFloat32FromDynamic = TO_FLOAT32_FROM_DYNAMIC;
    let b_f64: ToFloat64FromDynamic = TO_FLOAT64_FROM_DYNAMIC;

    // Dynamic -> Fixed Owned
    let o_i8: ToInt8FromOwnedDynamic = TO_INT8_FROM_OWNED_DYNAMIC;
    let o_i16: ToInt16FromOwnedDynamic = TO_INT16_FROM_OWNED_DYNAMIC;
    let o_i32: ToInt32FromOwnedDynamic = TO_INT32_FROM_OWNED_DYNAMIC;
    let o_i64: ToInt64FromOwnedDynamic = TO_INT64_FROM_OWNED_DYNAMIC;
    let o_i128: ToInt128FromOwnedDynamic = TO_INT128_FROM_OWNED_DYNAMIC;
    let o_u8: ToUint8FromOwnedDynamic = TO_UINT8_FROM_OWNED_DYNAMIC;
    let o_u16: ToUint16FromOwnedDynamic = TO_UINT16_FROM_OWNED_DYNAMIC;
    let o_u32: ToUint32FromOwnedDynamic = TO_UINT32_FROM_OWNED_DYNAMIC;
    let o_u64: ToUint64FromOwnedDynamic = TO_UINT64_FROM_OWNED_DYNAMIC;
    let o_u128: ToUint128FromOwnedDynamic = TO_UINT128_FROM_OWNED_DYNAMIC;
    let o_f32: ToFloat32FromOwnedDynamic = TO_FLOAT32_FROM_OWNED_DYNAMIC;
    let o_f64: ToFloat64FromOwnedDynamic = TO_FLOAT64_FROM_OWNED_DYNAMIC;

    let (sample_b, sample_o) = make_dyn_int(false, &[1]);
    assert_eq!(b_i8(&sample_b), Ok(1));
    assert_eq!(o_i8(&sample_o), Ok(1));
    assert_eq!(b_i16(&sample_b), Ok(1));
    assert_eq!(o_i16(&sample_o), Ok(1));
    assert_eq!(b_i32(&sample_b), Ok(1));
    assert_eq!(o_i32(&sample_o), Ok(1));
    assert_eq!(b_i64(&sample_b), Ok(1));
    assert_eq!(o_i64(&sample_o), Ok(1));
    assert_eq!(b_i128(&sample_b), Ok(1));
    assert_eq!(o_i128(&sample_o), Ok(1));
    assert_eq!(b_u8(&sample_b), Ok(1));
    assert_eq!(o_u8(&sample_o), Ok(1));
    assert_eq!(b_u16(&sample_b), Ok(1));
    assert_eq!(o_u16(&sample_o), Ok(1));
    assert_eq!(b_u32(&sample_b), Ok(1));
    assert_eq!(o_u32(&sample_o), Ok(1));
    assert_eq!(b_u64(&sample_b), Ok(1));
    assert_eq!(o_u64(&sample_o), Ok(1));
    assert_eq!(b_u128(&sample_b), Ok(1));
    assert_eq!(o_u128(&sample_o), Ok(1));
    assert_eq!(b_f32(&sample_b), Ok(1.0));
    assert_eq!(o_f32(&sample_o), Ok(1.0));
    assert_eq!(b_f64(&sample_b), Ok(1.0));
    assert_eq!(o_f64(&sample_o), Ok(1.0));
}
