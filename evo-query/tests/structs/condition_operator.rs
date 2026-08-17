use evo_query::definitions::structs::owned::condition_operator::ConditionOperator;

#[test]
fn condition_operator_variants() {
    assert_eq!(ConditionOperator::Equal, ConditionOperator::Equal);
    assert_eq!(
        ConditionOperator::GreaterThan,
        ConditionOperator::GreaterThan
    );
    assert_eq!(ConditionOperator::LessThan, ConditionOperator::LessThan);
    assert_eq!(ConditionOperator::Contains, ConditionOperator::Contains);
    assert_eq!(ConditionOperator::StartsWith, ConditionOperator::StartsWith);
    assert_eq!(ConditionOperator::EndsWith, ConditionOperator::EndsWith);
}

#[test]
fn condition_operator_inequality() {
    assert_ne!(ConditionOperator::Equal, ConditionOperator::GreaterThan);
    assert_ne!(ConditionOperator::GreaterThan, ConditionOperator::LessThan);
    assert_ne!(ConditionOperator::Contains, ConditionOperator::StartsWith);
    assert_ne!(ConditionOperator::StartsWith, ConditionOperator::EndsWith);
    assert_ne!(ConditionOperator::Contains, ConditionOperator::EndsWith);
    assert_ne!(ConditionOperator::Contains, ConditionOperator::Equal);
}

#[test]
fn condition_operator_copy() {
    let original = ConditionOperator::GreaterThan;
    let copied = original;

    assert_eq!(original, copied);
}

#[test]
fn condition_operator_text_operator_copy() {
    let original = ConditionOperator::Contains;
    let copied = original;

    assert_eq!(original, copied);
}
