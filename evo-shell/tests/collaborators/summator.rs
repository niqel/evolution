use evo_shell::collaborators::summator;
use evo_shell::definitions::types::number::Number;
use evo_shell::definitions::use_cases::sum::Error;

#[test]
fn same_integer_type_add_signed() {
    assert_eq!(
        summator::collaborate(Number::I8(10), Number::I8(20)),
        Ok(Number::I8(30))
    );
    assert_eq!(
        summator::collaborate(Number::I16(100), Number::I16(200)),
        Ok(Number::I16(300))
    );
    assert_eq!(
        summator::collaborate(Number::I32(1000), Number::I32(2000)),
        Ok(Number::I32(3000))
    );
    assert_eq!(
        summator::collaborate(Number::I64(10000), Number::I64(20000)),
        Ok(Number::I64(30000))
    );
    assert_eq!(
        summator::collaborate(Number::I128(100000), Number::I128(200000)),
        Ok(Number::I128(300000))
    );
}

#[test]
fn same_integer_type_add_unsigned() {
    assert_eq!(
        summator::collaborate(Number::U8(10), Number::U8(20)),
        Ok(Number::U8(30))
    );
    assert_eq!(
        summator::collaborate(Number::U16(100), Number::U16(200)),
        Ok(Number::U16(300))
    );
    assert_eq!(
        summator::collaborate(Number::U32(1000), Number::U32(2000)),
        Ok(Number::U32(3000))
    );
    assert_eq!(
        summator::collaborate(Number::U64(10000), Number::U64(20000)),
        Ok(Number::U64(30000))
    );
    assert_eq!(
        summator::collaborate(Number::U128(100000), Number::U128(200000)),
        Ok(Number::U128(300000))
    );
}

#[test]
fn float_addition() {
    assert_eq!(
        summator::collaborate(Number::F32(1.5), Number::F32(2.5)),
        Ok(Number::F32(4.0))
    );
    assert_eq!(
        summator::collaborate(Number::F64(1.5), Number::F64(2.5)),
        Ok(Number::F64(4.0))
    );
    assert_eq!(
        summator::collaborate(Number::F32(1.5), Number::F64(2.5)),
        Ok(Number::F64(4.0))
    );
    assert_eq!(
        summator::collaborate(Number::F64(1.5), Number::F32(2.5)),
        Ok(Number::F64(4.0))
    );
}

#[test]
fn mixed_integer_float_addition() {
    assert_eq!(
        summator::collaborate(Number::I32(10), Number::F64(2.5)),
        Ok(Number::F64(12.5))
    );
    assert_eq!(
        summator::collaborate(Number::F64(2.5), Number::I32(10)),
        Ok(Number::F64(12.5))
    );
    assert_eq!(
        summator::collaborate(Number::U8(10), Number::F32(2.5)),
        Ok(Number::F32(12.5))
    );
}

#[test]
fn integer_overflow_errors() {
    assert_eq!(
        summator::collaborate(Number::U8(255), Number::U8(1)),
        Err(Error::Overflow)
    );
    assert_eq!(
        summator::collaborate(Number::I8(127), Number::I8(1)),
        Err(Error::Overflow)
    );
}

#[test]
fn unsupported_integer_mix_errors() {
    assert_eq!(
        summator::collaborate(Number::I8(1), Number::I16(2)),
        Err(Error::UnsupportedTypes)
    );
    assert_eq!(
        summator::collaborate(Number::I32(1), Number::U32(2)),
        Err(Error::UnsupportedTypes)
    );
}
