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
