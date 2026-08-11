use evo_shell::agents::summator;
use evo_shell::definitions::types::number::Number;
use evo_shell::definitions::use_cases::sum;

#[test]
fn summator_success() {
    let use_case: sum::Sum = summator::sum;
    assert_eq!(
        use_case(Number::I32(10), Number::I32(20)),
        Ok(Number::I32(30))
    );
}
