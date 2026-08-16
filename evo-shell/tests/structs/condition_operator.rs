use evo_shell::definitions::structs::owned::condition_operator::ConditionOperator;

#[test]
fn condition_operator_variants() {
    assert_eq!(ConditionOperator::Equal, ConditionOperator::Equal);
    assert_eq!(ConditionOperator::NotEqual, ConditionOperator::NotEqual);
    assert_eq!(
        ConditionOperator::GreaterThan,
        ConditionOperator::GreaterThan
    );
    assert_eq!(
        ConditionOperator::GreaterThanOrEqual,
        ConditionOperator::GreaterThanOrEqual
    );
    assert_eq!(ConditionOperator::LessThan, ConditionOperator::LessThan);
    assert_eq!(
        ConditionOperator::LessThanOrEqual,
        ConditionOperator::LessThanOrEqual
    );
}

#[test]
fn condition_operator_inequality() {
    assert_ne!(ConditionOperator::Equal, ConditionOperator::NotEqual);
    assert_ne!(ConditionOperator::GreaterThan, ConditionOperator::LessThan);
    assert_ne!(
        ConditionOperator::GreaterThanOrEqual,
        ConditionOperator::LessThanOrEqual
    );
}

#[test]
fn condition_operator_copy() {
    let original = ConditionOperator::GreaterThan;
    let copied = original;

    assert_eq!(original, copied);
}
