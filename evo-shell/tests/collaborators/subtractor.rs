use evo_shell::collaborators::subtractor;
use evo_shell::definitions::types::number::Number;
use evo_shell::definitions::use_cases::subtract::Error;

#[test]
fn subtraction_examples() {
    assert_eq!(
        subtractor::collaborate(Number::I32(10), Number::I32(3)),
        Ok(Number::I32(7))
    );
    assert_eq!(
        subtractor::collaborate(Number::I32(3), Number::I32(10)),
        Ok(Number::I32(-7))
    );
    assert_eq!(
        subtractor::collaborate(Number::F64(10.0), Number::I32(2)),
        Ok(Number::F64(8.0))
    );
    assert_eq!(
        subtractor::collaborate(Number::I32(2), Number::F64(10.0)),
        Ok(Number::F64(-8.0))
    );
}

#[test]
fn subtraction_overflow() {
    assert_eq!(
        subtractor::collaborate(Number::U8(0), Number::U8(1)),
        Err(Error::Overflow)
    );
}
