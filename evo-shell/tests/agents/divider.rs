use evo_shell::agents::divider;
use evo_shell::definitions::types::number::Number;
use evo_shell::definitions::use_cases::divide;

#[test]
fn divider_success() {
    let use_case: divide::Divide = divider::divide;
    assert_eq!(
        use_case(Number::I32(20), Number::I32(4)),
        Ok(Number::I32(5))
    );
    assert_eq!(
        use_case(Number::I32(20), Number::I32(0)),
        Err(divide::Error::DivisionByZero)
    );
}
