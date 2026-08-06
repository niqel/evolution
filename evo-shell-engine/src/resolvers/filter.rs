use crate::definitions::domain::entities::filesystem_entry::FilesystemEntryKind;
use crate::definitions::domain::entities::filesystem_iteration_item::FilesystemIterationItem;
use crate::definitions::domain::value_objects::filter::{
    FilterComparison, FilterExpression, FilterOperand, FilterOperator, FilterProperty, FilterValue,
};
use crate::definitions::use_cases::filter::FilterError;

pub fn resolve<'a>(
    items: &'a [FilesystemIterationItem],
    expression: &FilterExpression,
) -> Result<Vec<&'a FilesystemIterationItem>, FilterError> {
    let mut filtered = Vec::new();

    for item in items {
        if evaluate_expression(item, expression)? {
            filtered.push(item);
        }
    }

    Ok(filtered)
}

fn evaluate_expression(
    item: &FilesystemIterationItem,
    expression: &FilterExpression,
) -> Result<bool, FilterError> {
    match expression {
        FilterExpression::Comparison(comparison) => evaluate_comparison(item, comparison),
        FilterExpression::And(expressions) => {
            for expression in expressions {
                if !evaluate_expression(item, expression)? {
                    return Ok(false);
                }
            }

            Ok(true)
        }
        FilterExpression::Or(expressions) => {
            for expression in expressions {
                if evaluate_expression(item, expression)? {
                    return Ok(true);
                }
            }

            Ok(false)
        }
    }
}

fn evaluate_comparison(
    item: &FilesystemIterationItem,
    comparison: &FilterComparison,
) -> Result<bool, FilterError> {
    let property = comparison.property();

    if let FilterProperty::Unsupported(name) = property {
        return Err(FilterError::UnsupportedProperty(name.clone()));
    }

    if !property_supports_operator(property, comparison.operator()) {
        return Err(FilterError::InvalidOperatorForProperty {
            property: property.clone(),
            operator: comparison.operator(),
        });
    }

    match property {
        FilterProperty::Index => compare_usize(item.index(), comparison),
        FilterProperty::Created => compare_time(
            item.entry()
                .created()
                .ok_or_else(|| FilterError::MissingComparableValue {
                    property: property.clone(),
                })?,
            comparison,
        ),
        FilterProperty::Modified => compare_time(
            item.entry()
                .modified()
                .ok_or_else(|| FilterError::MissingComparableValue {
                    property: property.clone(),
                })?,
            comparison,
        ),
        FilterProperty::Type => compare_kind(item.entry().kind(), comparison),
        FilterProperty::Size => compare_u64(
            item.entry()
                .size()
                .ok_or_else(|| FilterError::MissingComparableValue {
                    property: property.clone(),
                })?,
            comparison,
        ),
        FilterProperty::Name => compare_name(item.entry().name(), comparison),
        FilterProperty::Unsupported(_) => unreachable!(),
    }
}

fn property_supports_operator(property: &FilterProperty, operator: FilterOperator) -> bool {
    match property {
        FilterProperty::Index
        | FilterProperty::Created
        | FilterProperty::Modified
        | FilterProperty::Size => {
            matches!(
                operator,
                FilterOperator::Equals
                    | FilterOperator::NotEquals
                    | FilterOperator::GreaterThan
                    | FilterOperator::LessThan
                    | FilterOperator::AtLeast
                    | FilterOperator::AtMost
                    | FilterOperator::Between
                    | FilterOperator::NotBetween
            )
        }
        FilterProperty::Type | FilterProperty::Name => {
            matches!(operator, FilterOperator::Equals | FilterOperator::NotEquals)
        }
        FilterProperty::Unsupported(_) => false,
    }
}

macro_rules! compare_ordered_value {
    ($value:expr, $comparison:expr, $variant:ident) => {{
        let operand = $comparison.operand();

        match $comparison.operator() {
            FilterOperator::Equals => match operand {
                FilterOperand::Single(FilterValue::$variant(expected)) => Ok($value == *expected),
                _ => Err(FilterError::InvalidOperatorForProperty {
                    property: $comparison.property().clone(),
                    operator: $comparison.operator(),
                }),
            },
            FilterOperator::NotEquals => match operand {
                FilterOperand::Single(FilterValue::$variant(expected)) => Ok($value != *expected),
                _ => Err(FilterError::InvalidOperatorForProperty {
                    property: $comparison.property().clone(),
                    operator: $comparison.operator(),
                }),
            },
            FilterOperator::GreaterThan => match operand {
                FilterOperand::Single(FilterValue::$variant(expected)) => Ok($value > *expected),
                _ => Err(FilterError::InvalidOperatorForProperty {
                    property: $comparison.property().clone(),
                    operator: $comparison.operator(),
                }),
            },
            FilterOperator::LessThan => match operand {
                FilterOperand::Single(FilterValue::$variant(expected)) => Ok($value < *expected),
                _ => Err(FilterError::InvalidOperatorForProperty {
                    property: $comparison.property().clone(),
                    operator: $comparison.operator(),
                }),
            },
            FilterOperator::AtLeast => match operand {
                FilterOperand::Single(FilterValue::$variant(expected)) => Ok($value >= *expected),
                _ => Err(FilterError::InvalidOperatorForProperty {
                    property: $comparison.property().clone(),
                    operator: $comparison.operator(),
                }),
            },
            FilterOperator::AtMost => match operand {
                FilterOperand::Single(FilterValue::$variant(expected)) => Ok($value <= *expected),
                _ => Err(FilterError::InvalidOperatorForProperty {
                    property: $comparison.property().clone(),
                    operator: $comparison.operator(),
                }),
            },
            FilterOperator::Between => match operand {
                FilterOperand::Range { lower, upper } => match (lower, upper) {
                    (FilterValue::$variant(lower), FilterValue::$variant(upper)) => {
                        Ok($value >= *lower && $value <= *upper)
                    }
                    _ => Err(FilterError::InvalidOperatorForProperty {
                        property: $comparison.property().clone(),
                        operator: $comparison.operator(),
                    }),
                },
                _ => Err(FilterError::InvalidOperatorForProperty {
                    property: $comparison.property().clone(),
                    operator: $comparison.operator(),
                }),
            },
            FilterOperator::NotBetween => match operand {
                FilterOperand::Range { lower, upper } => match (lower, upper) {
                    (FilterValue::$variant(lower), FilterValue::$variant(upper)) => {
                        Ok($value < *lower || $value > *upper)
                    }
                    _ => Err(FilterError::InvalidOperatorForProperty {
                        property: $comparison.property().clone(),
                        operator: $comparison.operator(),
                    }),
                },
                _ => Err(FilterError::InvalidOperatorForProperty {
                    property: $comparison.property().clone(),
                    operator: $comparison.operator(),
                }),
            },
        }
    }};
}

fn compare_usize(value: usize, comparison: &FilterComparison) -> Result<bool, FilterError> {
    compare_ordered_value!(value, comparison, Index)
}

fn compare_u64(value: u64, comparison: &FilterComparison) -> Result<bool, FilterError> {
    compare_ordered_value!(value, comparison, Size)
}

fn compare_time(
    value: std::time::SystemTime,
    comparison: &FilterComparison,
) -> Result<bool, FilterError> {
    compare_ordered_value!(value, comparison, Time)
}

fn compare_kind(
    value: FilesystemEntryKind,
    comparison: &FilterComparison,
) -> Result<bool, FilterError> {
    let operand = comparison.operand();

    match comparison.operator() {
        FilterOperator::Equals => match operand {
            FilterOperand::Single(FilterValue::Type(expected)) => Ok(value == *expected),
            _ => Err(FilterError::InvalidOperatorForProperty {
                property: comparison.property().clone(),
                operator: comparison.operator(),
            }),
        },
        FilterOperator::NotEquals => match operand {
            FilterOperand::Single(FilterValue::Type(expected)) => Ok(value != *expected),
            _ => Err(FilterError::InvalidOperatorForProperty {
                property: comparison.property().clone(),
                operator: comparison.operator(),
            }),
        },
        _ => Err(FilterError::InvalidOperatorForProperty {
            property: comparison.property().clone(),
            operator: comparison.operator(),
        }),
    }
}

fn compare_name(
    value: &std::ffi::OsStr,
    comparison: &FilterComparison,
) -> Result<bool, FilterError> {
    let operand = comparison.operand();

    match comparison.operator() {
        FilterOperator::Equals => match operand {
            FilterOperand::Single(FilterValue::Name(expected)) => Ok(value == expected),
            _ => Err(FilterError::InvalidOperatorForProperty {
                property: comparison.property().clone(),
                operator: comparison.operator(),
            }),
        },
        FilterOperator::NotEquals => match operand {
            FilterOperand::Single(FilterValue::Name(expected)) => Ok(value != expected),
            _ => Err(FilterError::InvalidOperatorForProperty {
                property: comparison.property().clone(),
                operator: comparison.operator(),
            }),
        },
        _ => Err(FilterError::InvalidOperatorForProperty {
            property: comparison.property().clone(),
            operator: comparison.operator(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::agents::filterer;
    use crate::definitions::domain::entities::filesystem_entry::{
        FilesystemEntry, FilesystemEntryKind,
    };
    use crate::definitions::domain::entities::filesystem_iteration_item::FilesystemIterationItem;
    use crate::definitions::domain::value_objects::filter::{
        FilterComparison, FilterExpression, FilterOperand, FilterOperator, FilterProperty,
        FilterValue,
    };
    use crate::definitions::use_cases::filter::Filter;
    use std::ffi::OsString;
    use std::path::PathBuf;
    use std::time::{Duration, SystemTime};

    fn time(seconds: u64) -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::from_secs(seconds)
    }

    fn entry(
        name: &str,
        kind: FilesystemEntryKind,
        created: Option<SystemTime>,
        modified: Option<SystemTime>,
        size: Option<u64>,
    ) -> FilesystemEntry {
        FilesystemEntry::new(
            OsString::from(name),
            PathBuf::from(format!("/tmp/{name}")),
            kind,
            created,
            modified,
            size,
        )
    }

    fn item(
        index: usize,
        name: &str,
        kind: FilesystemEntryKind,
        created: Option<SystemTime>,
        modified: Option<SystemTime>,
        size: Option<u64>,
    ) -> FilesystemIterationItem {
        FilesystemIterationItem::new(index, entry(name, kind, created, modified, size))
    }

    fn sample_items() -> Vec<FilesystemIterationItem> {
        vec![
            item(
                0,
                "README.md",
                FilesystemEntryKind::File,
                Some(time(10)),
                Some(time(20)),
                Some(10),
            ),
            item(
                1,
                "src",
                FilesystemEntryKind::Directory,
                Some(time(30)),
                Some(time(40)),
                None,
            ),
            item(
                2,
                "notes.txt",
                FilesystemEntryKind::File,
                Some(time(50)),
                Some(time(60)),
                Some(30),
            ),
            item(
                3,
                "temp.txt",
                FilesystemEntryKind::File,
                None,
                None,
                Some(5),
            ),
        ]
    }

    fn readme_only_items() -> Vec<FilesystemIterationItem> {
        vec![item(
            0,
            "README.md",
            FilesystemEntryKind::File,
            Some(time(10)),
            Some(time(20)),
            Some(10),
        )]
    }

    fn sized_items() -> Vec<FilesystemIterationItem> {
        vec![
            item(
                0,
                "README.md",
                FilesystemEntryKind::File,
                Some(time(10)),
                Some(time(20)),
                Some(10),
            ),
            item(
                2,
                "notes.txt",
                FilesystemEntryKind::File,
                Some(time(50)),
                Some(time(60)),
                Some(30),
            ),
            item(
                3,
                "temp.txt",
                FilesystemEntryKind::File,
                None,
                None,
                Some(5),
            ),
        ]
    }

    fn name_equals(value: &str) -> FilterExpression {
        FilterExpression::comparison(FilterComparison::new(
            FilterProperty::Name,
            FilterOperator::Equals,
            FilterOperand::single(FilterValue::name(value)),
        ))
    }

    fn name_not_equals(value: &str) -> FilterExpression {
        FilterExpression::comparison(FilterComparison::new(
            FilterProperty::Name,
            FilterOperator::NotEquals,
            FilterOperand::single(FilterValue::name(value)),
        ))
    }

    fn kind_equals(kind: FilesystemEntryKind) -> FilterExpression {
        FilterExpression::comparison(FilterComparison::new(
            FilterProperty::Type,
            FilterOperator::Equals,
            FilterOperand::single(FilterValue::kind(kind)),
        ))
    }

    fn index_lt(value: usize) -> FilterExpression {
        FilterExpression::comparison(FilterComparison::new(
            FilterProperty::Index,
            FilterOperator::LessThan,
            FilterOperand::single(FilterValue::index(value)),
        ))
    }

    fn index_gt(value: usize) -> FilterExpression {
        FilterExpression::comparison(FilterComparison::new(
            FilterProperty::Index,
            FilterOperator::GreaterThan,
            FilterOperand::single(FilterValue::index(value)),
        ))
    }

    fn size_at_least(value: u64) -> FilterExpression {
        FilterExpression::comparison(FilterComparison::new(
            FilterProperty::Size,
            FilterOperator::AtLeast,
            FilterOperand::single(FilterValue::size(value)),
        ))
    }

    fn size_at_most(value: u64) -> FilterExpression {
        FilterExpression::comparison(FilterComparison::new(
            FilterProperty::Size,
            FilterOperator::AtMost,
            FilterOperand::single(FilterValue::size(value)),
        ))
    }

    fn size_between(lower: u64, upper: u64) -> FilterExpression {
        FilterExpression::comparison(FilterComparison::new(
            FilterProperty::Size,
            FilterOperator::Between,
            FilterOperand::range(FilterValue::size(lower), FilterValue::size(upper)),
        ))
    }

    fn size_not_between(lower: u64, upper: u64) -> FilterExpression {
        FilterExpression::comparison(FilterComparison::new(
            FilterProperty::Size,
            FilterOperator::NotBetween,
            FilterOperand::range(FilterValue::size(lower), FilterValue::size(upper)),
        ))
    }

    fn unsupported_property_expression() -> FilterExpression {
        FilterExpression::comparison(FilterComparison::new(
            FilterProperty::unsupported("unsupported"),
            FilterOperator::Equals,
            FilterOperand::single(FilterValue::name("ignored")),
        ))
    }

    #[test]
    fn filter_matches_use_case_function_pointer() {
        let filter_case: Filter = filterer::filter;
        let items = sample_items();

        let result = filter_case(&items, &name_equals("README.md")).unwrap();

        assert_eq!(result.len(), 1);
        assert!(std::ptr::eq(result[0], &items[0]));
    }

    #[test]
    fn empty_collection_returns_empty_success() {
        let result = resolve(&[], &name_equals("README.md")).unwrap();

        assert!(result.is_empty());
    }

    #[test]
    fn no_matches_returns_empty_success() {
        let items = sample_items();

        let result = resolve(&items, &name_equals("does-not-exist.md")).unwrap();

        assert!(result.is_empty());
    }

    #[test]
    fn filter_preserves_complete_elements_as_borrowed_references() {
        let items = sample_items();

        let result = resolve(&items, &kind_equals(FilesystemEntryKind::File)).unwrap();

        assert_eq!(result.len(), 3);
        assert!(std::ptr::eq(result[0], &items[0]));
        assert!(std::ptr::eq(result[1], &items[2]));
        assert!(std::ptr::eq(result[2], &items[3]));
    }

    #[test]
    fn not_equals_on_name_excludes_only_the_matching_item() {
        let items = sample_items();

        let result = resolve(&items, &name_not_equals("temp.txt")).unwrap();

        assert_eq!(result.len(), 3);
        assert!(result.iter().all(|item| item.entry().name() != "temp.txt"));
    }

    #[test]
    fn index_comparisons_respect_position() {
        let items = sample_items();

        let less_than_two = resolve(&items, &index_lt(2)).unwrap();
        let greater_than_one = resolve(&items, &index_gt(1)).unwrap();

        assert_eq!(less_than_two.len(), 2);
        assert_eq!(greater_than_one.len(), 2);
        assert!(std::ptr::eq(less_than_two[0], &items[0]));
        assert!(std::ptr::eq(less_than_two[1], &items[1]));
        assert!(std::ptr::eq(greater_than_one[0], &items[2]));
        assert!(std::ptr::eq(greater_than_one[1], &items[3]));
    }

    #[test]
    fn size_comparisons_respect_numeric_boundaries() {
        let items = sized_items();

        let at_least = resolve(&items, &size_at_least(10)).unwrap();
        let at_most = resolve(&items, &size_at_most(10)).unwrap();

        assert_eq!(at_least.len(), 2);
        assert_eq!(at_most.len(), 2);
        assert_eq!(at_least[0].index(), 0);
        assert_eq!(at_least[1].index(), 2);
        assert_eq!(at_least[0].entry().size(), Some(10));
        assert_eq!(at_least[1].entry().size(), Some(30));
        assert_eq!(at_most[0].index(), 0);
        assert_eq!(at_most[1].index(), 3);
        assert_eq!(at_most[0].entry().size(), Some(10));
        assert_eq!(at_most[1].entry().size(), Some(5));
    }

    #[test]
    fn between_and_not_between_cover_inclusive_bounds() {
        let items = sized_items();

        let between = resolve(&items, &size_between(10, 30)).unwrap();
        let not_between = resolve(&items, &size_not_between(10, 30)).unwrap();

        assert_eq!(between.len(), 2);
        assert_eq!(between[0].index(), 0);
        assert_eq!(between[1].index(), 2);
        assert_eq!(not_between.len(), 1);
        assert_eq!(not_between[0].index(), 3);
    }

    #[test]
    fn and_expression_combines_predicates() {
        let items = sized_items();
        let expression =
            FilterExpression::and(vec![kind_equals(FilesystemEntryKind::File), size_gt(10)]);

        let result = resolve(&items, &expression).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].index(), 2);
    }

    #[test]
    fn and_short_circuits_when_first_branch_is_false() {
        let items = sample_items();
        let expression = FilterExpression::and(vec![
            name_equals("does-not-exist"),
            unsupported_property_expression(),
        ]);

        let result = resolve(&items, &expression).unwrap();

        assert!(result.is_empty());
    }

    #[test]
    fn and_evaluates_second_branch_when_first_is_true() {
        let items = sample_items();
        let expression = FilterExpression::and(vec![
            kind_equals(FilesystemEntryKind::File),
            name_equals("src"),
        ]);

        let result = resolve(&items, &expression).unwrap();

        assert!(result.is_empty());
    }

    #[test]
    fn or_expression_combines_predicates() {
        let items = sample_items();
        let expression =
            FilterExpression::or(vec![name_equals("README.md"), name_equals("notes.txt")]);

        let result = resolve(&items, &expression).unwrap();

        assert_eq!(result.len(), 2);
        assert!(std::ptr::eq(result[0], &items[0]));
        assert!(std::ptr::eq(result[1], &items[2]));
    }

    #[test]
    fn or_short_circuits_when_first_branch_is_true() {
        let items = readme_only_items();
        let expression = FilterExpression::or(vec![
            name_equals("README.md"),
            unsupported_property_expression(),
        ]);

        let result = resolve(&items, &expression).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].index(), 0);
    }

    #[test]
    fn or_evaluates_second_branch_when_first_is_false() {
        let items = sample_items();
        let expression = FilterExpression::or(vec![
            name_equals("does-not-exist"),
            name_equals("README.md"),
        ]);

        let result = resolve(&items, &expression).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].index(), 0);
    }

    #[test]
    fn and_chain_short_circuits_before_third_branch() {
        let items = readme_only_items();
        let expression = FilterExpression::and(vec![
            kind_equals(FilesystemEntryKind::File),
            name_equals("does-not-exist"),
            unsupported_property_expression(),
        ]);

        let result = resolve(&items, &expression).unwrap();

        assert!(result.is_empty());
    }

    #[test]
    fn or_chain_short_circuits_before_third_branch() {
        let items = readme_only_items();
        let expression = FilterExpression::or(vec![
            name_equals("does-not-exist"),
            name_equals("README.md"),
            unsupported_property_expression(),
        ]);

        let result = resolve(&items, &expression).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].index(), 0);
    }

    #[test]
    fn grouped_expression_can_mix_and_or_explicitly() {
        let items = sample_items();
        let expression = FilterExpression::and(vec![
            FilterExpression::or(vec![name_equals("README.md"), name_equals("notes.txt")]),
            kind_equals(FilesystemEntryKind::File),
        ]);

        let result = resolve(&items, &expression).unwrap();

        assert_eq!(result.len(), 2);
        assert!(std::ptr::eq(result[0], &items[0]));
        assert!(std::ptr::eq(result[1], &items[2]));
    }

    #[test]
    fn invalid_property_returns_error() {
        let items = sample_items();
        let expression = FilterExpression::comparison(FilterComparison::new(
            FilterProperty::unsupported("mystery"),
            FilterOperator::Equals,
            FilterOperand::single(FilterValue::name("README.md")),
        ));

        let result = resolve(&items, &expression);

        assert!(
            matches!(result, Err(FilterError::UnsupportedProperty(property)) if property == "mystery")
        );
    }

    #[test]
    fn incompatible_operator_returns_error() {
        let items = sample_items();
        let expression = FilterExpression::comparison(FilterComparison::new(
            FilterProperty::Name,
            FilterOperator::Between,
            FilterOperand::range(FilterValue::name("a"), FilterValue::name("z")),
        ));

        let result = resolve(&items, &expression);

        assert!(matches!(
            result,
            Err(FilterError::InvalidOperatorForProperty {
                property: FilterProperty::Name,
                operator: FilterOperator::Between
            })
        ));
    }

    #[test]
    fn missing_comparable_value_returns_error() {
        let items = vec![item(
            1,
            "src",
            FilesystemEntryKind::Directory,
            Some(time(30)),
            Some(time(40)),
            None,
        )];
        let expression = size_gt(0);

        let result = resolve(&items, &expression);

        assert!(matches!(
            result,
            Err(FilterError::MissingComparableValue {
                property: FilterProperty::Size
            })
        ));
    }

    fn size_gt(value: u64) -> FilterExpression {
        FilterExpression::comparison(FilterComparison::new(
            FilterProperty::Size,
            FilterOperator::GreaterThan,
            FilterOperand::single(FilterValue::size(value)),
        ))
    }
}
