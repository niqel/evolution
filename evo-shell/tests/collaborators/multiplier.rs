use evo_shell::collaborators::multiplier;
use evo_shell::definitions::types::number::Number;
use evo_shell::definitions::use_cases::multiply::Error;

#[test]
fn multiplication_examples() {
    assert_eq!(
        multiplier::collaborate(Number::I32(3), Number::I32(4)),
        Ok(Number::I32(12))
    );
    assert_eq!(
        multiplier::collaborate(Number::F64(2.5), Number::I32(4)),
        Ok(Number::F64(10.0))
    );
}

#[test]
fn multiplication_overflow() {
    assert_eq!(
        multiplier::collaborate(Number::U64(u64::MAX), Number::U64(2)),
        Err(Error::Overflow)
    );
}

#[test]
fn multiplication_unsupported_mix() {
    assert_eq!(
        multiplier::collaborate(Number::U16(2), Number::U32(3)),
        Err(Error::UnsupportedTypes)
    );
}
