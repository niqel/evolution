use evo_shell::agents::multiplier;
use evo_shell::definitions::types::number::Number;
use evo_shell::definitions::use_cases::multiply;

#[test]
fn multiplier_success() {
    let use_case: multiply::Multiply = multiplier::multiply;
    assert_eq!(
        use_case(Number::I32(5), Number::I32(6)),
        Ok(Number::I32(30))
    );
}
