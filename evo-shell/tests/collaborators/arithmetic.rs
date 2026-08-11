use evo_shell::collaborators::arithmetic::{self, Error};
use evo_shell::definitions::types::number::Number;

#[test]
fn same_integer_type_add_signed() {
    assert_eq!(
        arithmetic::add(Number::I8(10), Number::I8(20)),
        Ok(Number::I8(30))
    );
    assert_eq!(
        arithmetic::add(Number::I16(100), Number::I16(200)),
        Ok(Number::I16(300))
    );
    assert_eq!(
        arithmetic::add(Number::I32(1000), Number::I32(2000)),
        Ok(Number::I32(3000))
    );
    assert_eq!(
        arithmetic::add(Number::I64(10000), Number::I64(20000)),
        Ok(Number::I64(30000))
    );
    assert_eq!(
        arithmetic::add(Number::I128(100000), Number::I128(200000)),
        Ok(Number::I128(300000))
    );
}

#[test]
fn same_integer_type_add_unsigned() {
    assert_eq!(
        arithmetic::add(Number::U8(10), Number::U8(20)),
        Ok(Number::U8(30))
    );
    assert_eq!(
        arithmetic::add(Number::U16(100), Number::U16(200)),
        Ok(Number::U16(300))
    );
    assert_eq!(
        arithmetic::add(Number::U32(1000), Number::U32(2000)),
        Ok(Number::U32(3000))
    );
    assert_eq!(
        arithmetic::add(Number::U64(10000), Number::U64(20000)),
        Ok(Number::U64(30000))
    );
    assert_eq!(
        arithmetic::add(Number::U128(100000), Number::U128(200000)),
        Ok(Number::U128(300000))
    );
}

#[test]
fn float_addition() {
    assert_eq!(
        arithmetic::add(Number::F32(1.5), Number::F32(2.5)),
        Ok(Number::F32(4.0))
    );
    assert_eq!(
        arithmetic::add(Number::F64(1.5), Number::F64(2.5)),
        Ok(Number::F64(4.0))
    );
    assert_eq!(
        arithmetic::add(Number::F32(1.5), Number::F64(2.5)),
        Ok(Number::F64(4.0))
    );
    assert_eq!(
        arithmetic::add(Number::F64(1.5), Number::F32(2.5)),
        Ok(Number::F64(4.0))
    );
}

#[test]
fn mixed_integer_float_addition() {
    assert_eq!(
        arithmetic::add(Number::I32(10), Number::F64(2.5)),
        Ok(Number::F64(12.5))
    );
    assert_eq!(
        arithmetic::add(Number::F64(2.5), Number::I32(10)),
        Ok(Number::F64(12.5))
    );
    assert_eq!(
        arithmetic::add(Number::U8(10), Number::F32(2.5)),
        Ok(Number::F32(12.5))
    );
}

#[test]
fn subtraction_examples() {
    assert_eq!(
        arithmetic::subtract(Number::I32(10), Number::I32(3)),
        Ok(Number::I32(7))
    );
    assert_eq!(
        arithmetic::subtract(Number::I32(3), Number::I32(10)),
        Ok(Number::I32(-7))
    );
    assert_eq!(
        arithmetic::subtract(Number::F64(10.0), Number::I32(2)),
        Ok(Number::F64(8.0))
    );
    assert_eq!(
        arithmetic::subtract(Number::I32(2), Number::F64(10.0)),
        Ok(Number::F64(-8.0))
    );
}

#[test]
fn multiplication_examples() {
    assert_eq!(
        arithmetic::multiply(Number::I32(3), Number::I32(4)),
        Ok(Number::I32(12))
    );
    assert_eq!(
        arithmetic::multiply(Number::F64(2.5), Number::I32(4)),
        Ok(Number::F64(10.0))
    );
}

#[test]
fn division_examples() {
    assert_eq!(
        arithmetic::divide(Number::I32(7), Number::I32(3)),
        Ok(Number::I32(2))
    );
    assert_eq!(
        arithmetic::divide(Number::I32(-7), Number::I32(3)),
        Ok(Number::I32(-2))
    );
    assert_eq!(
        arithmetic::divide(Number::F64(10.0), Number::I32(4)),
        Ok(Number::F64(2.5))
    );
    assert_eq!(
        arithmetic::divide(Number::I32(10), Number::F64(4.0)),
        Ok(Number::F64(2.5))
    );
}

#[test]
fn remainder_examples() {
    assert_eq!(
        arithmetic::remainder(Number::I32(7), Number::I32(3)),
        Ok(Number::I32(1))
    );
    assert_eq!(
        arithmetic::remainder(Number::I32(-7), Number::I32(3)),
        Ok(Number::I32(-1))
    );
    assert_eq!(
        arithmetic::remainder(Number::U8(7), Number::U8(3)),
        Ok(Number::U8(1))
    );
}

#[test]
fn integer_overflow_errors() {
    assert_eq!(
        arithmetic::add(Number::U8(255), Number::U8(1)),
        Err(Error::Overflow)
    );
    assert_eq!(
        arithmetic::add(Number::I8(127), Number::I8(1)),
        Err(Error::Overflow)
    );
    assert_eq!(
        arithmetic::subtract(Number::U8(0), Number::U8(1)),
        Err(Error::Overflow)
    );
    assert_eq!(
        arithmetic::multiply(Number::U64(u64::MAX), Number::U64(2)),
        Err(Error::Overflow)
    );
}

#[test]
fn division_by_zero_and_overflow_errors() {
    assert_eq!(
        arithmetic::divide(Number::I32(10), Number::I32(0)),
        Err(Error::DivisionByZero)
    );
    assert_eq!(
        arithmetic::remainder(Number::I32(10), Number::I32(0)),
        Err(Error::DivisionByZero)
    );
    assert_eq!(
        arithmetic::divide(Number::I8(i8::MIN), Number::I8(-1)),
        Err(Error::Overflow)
    );
}

#[test]
fn unsupported_integer_mix_errors() {
    assert_eq!(
        arithmetic::add(Number::I8(1), Number::I16(2)),
        Err(Error::UnsupportedTypes)
    );
    assert_eq!(
        arithmetic::add(Number::I32(1), Number::U32(2)),
        Err(Error::UnsupportedTypes)
    );
    assert_eq!(
        arithmetic::multiply(Number::U16(2), Number::U32(3)),
        Err(Error::UnsupportedTypes)
    );
}

#[test]
fn float_remainder_unsupported() {
    assert_eq!(
        arithmetic::remainder(Number::F64(7.0), Number::F64(3.0)),
        Err(Error::UnsupportedTypes)
    );
}

#[test]
fn float_ieee754_behavior() {
    let div_inf = arithmetic::divide(Number::F64(1.0), Number::F64(0.0));
    assert_eq!(div_inf, Ok(Number::F64(f64::INFINITY)));

    let div_nan = arithmetic::divide(Number::F64(0.0), Number::F64(0.0));
    match div_nan {
        Ok(Number::F64(val)) => assert!(val.is_nan()),
        _ => panic!("Expected F64(NaN)"),
    }
}

#[test]
fn stable_type_result_preservation() {
    assert_eq!(
        arithmetic::add(Number::F64(2.5), Number::F64(2.5)),
        Ok(Number::F64(5.0))
    );
}

#[test]
fn negate_behavior() {
    assert_eq!(arithmetic::negate(Number::I8(5)), Ok(Number::I8(-5)));
    assert_eq!(
        arithmetic::negate(Number::I8(i8::MIN)),
        Err(Error::Overflow)
    );
    assert_eq!(
        arithmetic::negate(Number::U8(5)),
        Err(Error::UnsupportedTypes)
    );
    assert_eq!(
        arithmetic::negate(Number::U8(0)),
        Err(Error::UnsupportedTypes)
    );
    assert_eq!(arithmetic::negate(Number::F64(2.5)), Ok(Number::F64(-2.5)));
}

#[test]
fn number_size_of_check() {
    let size = std::mem::size_of::<Number>();
    assert!(size > 0);
}
