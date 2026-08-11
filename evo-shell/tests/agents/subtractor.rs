use evo_shell::agents::subtractor;
use evo_shell::definitions::types::number::Number;
use evo_shell::definitions::use_cases::subtract;

#[test]
fn subtractor_success() {
    let use_case: subtract::Subtract = subtractor::subtract;
    assert_eq!(
        use_case(Number::I32(50), Number::I32(20)),
        Ok(Number::I32(30))
    );
}
