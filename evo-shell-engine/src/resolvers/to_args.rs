use crate::definitions::domain::value_objects::arguments::Arguments;
use crate::definitions::domain::value_objects::select::StructuredProjection;
use crate::definitions::use_cases::to_args::ToArgsError;

pub fn resolve(projection: StructuredProjection) -> Result<Arguments, ToArgsError> {
    let property_count = projection.property_count();

    if property_count == 0 {
        return Err(ToArgsError::InvalidPropertyCount { actual: 0 });
    }

    if property_count > 1 {
        return Err(ToArgsError::InvalidPropertyCount {
            actual: property_count,
        });
    }

    let rows = projection.into_rows();
    let mut arguments = Vec::with_capacity(rows.len());

    for (row_index, row) in rows.into_iter().enumerate() {
        if row.len() != 1 {
            return Err(ToArgsError::InconsistentRowWidth {
                row: row_index,
                actual: row.len(),
            });
        }

        let mut row_values = row.into_values().into_iter();
        arguments.push(
            row_values
                .next()
                .expect("row width checked to be exactly one"),
        );
    }

    Ok(Arguments::new(arguments))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::agents::argument_expander;
    use crate::definitions::domain::value_objects::select::{
        ProjectedRow, ProjectedValue, SelectProperty, StructuredProjection,
    };
    use crate::definitions::use_cases::to_args::{ToArgs, ToArgsError};

    fn projection(
        properties: Vec<SelectProperty>,
        rows: Vec<ProjectedRow>,
    ) -> StructuredProjection {
        StructuredProjection::new(properties, rows)
    }

    fn name_arguments_projection() -> StructuredProjection {
        projection(
            vec![SelectProperty::Name],
            vec![
                ProjectedRow::new(vec![ProjectedValue::name("README.md")]),
                ProjectedRow::new(vec![ProjectedValue::name("src")]),
                ProjectedRow::new(vec![ProjectedValue::name("notes.txt")]),
            ],
        )
    }

    #[test]
    fn argument_expander_matches_use_case_function_pointer() {
        let to_args_case: ToArgs = argument_expander::expand;
        let arguments = to_args_case(name_arguments_projection()).unwrap();

        assert_eq!(arguments.len(), 3);
    }

    #[test]
    fn to_args_returns_argument_sequence_without_stringifying() {
        let arguments = resolve(name_arguments_projection()).unwrap();

        assert_eq!(
            arguments.items(),
            &[
                ProjectedValue::name("README.md"),
                ProjectedValue::name("src"),
                ProjectedValue::name("notes.txt"),
            ]
        );
    }

    #[test]
    fn to_args_accepts_empty_rows() {
        let arguments = resolve(projection(vec![SelectProperty::Name], Vec::new())).unwrap();

        assert!(arguments.is_empty());
    }

    #[test]
    fn to_args_rejects_empty_properties() {
        let result = resolve(projection(Vec::new(), Vec::new()));

        assert!(matches!(
            result,
            Err(ToArgsError::InvalidPropertyCount { actual }) if actual == 0
        ));
    }

    #[test]
    fn to_args_rejects_multiple_properties() {
        let result = resolve(projection(
            vec![SelectProperty::Name, SelectProperty::Size],
            vec![ProjectedRow::new(vec![
                ProjectedValue::name("README.md"),
                ProjectedValue::size(Some(1)),
            ])],
        ));

        assert!(matches!(
            result,
            Err(ToArgsError::InvalidPropertyCount { actual }) if actual == 2
        ));
    }

    #[test]
    fn to_args_preserves_optional_absence_and_row_order() {
        let arguments = resolve(projection(
            vec![SelectProperty::Size],
            vec![
                ProjectedRow::new(vec![ProjectedValue::size(Some(10))]),
                ProjectedRow::new(vec![ProjectedValue::size(None)]),
                ProjectedRow::new(vec![ProjectedValue::size(Some(30))]),
            ],
        ))
        .unwrap();

        assert_eq!(
            arguments.items(),
            &[
                ProjectedValue::Size(Some(10)),
                ProjectedValue::Size(None),
                ProjectedValue::Size(Some(30)),
            ]
        );
    }

    #[test]
    fn to_args_rejects_inconsistent_row_width() {
        let result = resolve(projection(
            vec![SelectProperty::Name],
            vec![ProjectedRow::new(vec![
                ProjectedValue::name("README.md"),
                ProjectedValue::size(Some(1)),
            ])],
        ));

        assert!(matches!(
            result,
            Err(ToArgsError::InconsistentRowWidth { row, actual }) if row == 0 && actual == 2
        ));
    }
}
