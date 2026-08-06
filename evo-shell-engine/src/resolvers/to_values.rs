use crate::definitions::domain::value_objects::select::StructuredProjection;
use crate::definitions::domain::value_objects::values::Values;
use crate::definitions::use_cases::to_values::ToValuesError;

pub fn resolve(projection: StructuredProjection) -> Result<Values, ToValuesError> {
    let property_count = projection.property_count();

    if property_count == 0 {
        return Err(ToValuesError::InvalidPropertyCount { actual: 0 });
    }

    if property_count > 1 {
        return Err(ToValuesError::InvalidPropertyCount {
            actual: property_count,
        });
    }

    let rows = projection.into_rows();
    let mut values = Vec::with_capacity(rows.len());

    for (row_index, row) in rows.into_iter().enumerate() {
        if row.len() != 1 {
            return Err(ToValuesError::InconsistentRowWidth {
                row: row_index,
                actual: row.len(),
            });
        }

        let mut row_values = row.into_values().into_iter();
        values.push(
            row_values
                .next()
                .expect("row width checked to be exactly one"),
        );
    }

    Ok(Values::new(values))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::agents::values_converter;
    use crate::definitions::domain::value_objects::select::{
        ProjectedRow, ProjectedValue, SelectProperty, StructuredProjection,
    };
    use crate::definitions::use_cases::to_values::{ToValues, ToValuesError};

    fn projection(
        properties: Vec<SelectProperty>,
        rows: Vec<ProjectedRow>,
    ) -> StructuredProjection {
        StructuredProjection::new(properties, rows)
    }

    fn name_values_projection() -> StructuredProjection {
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
    fn values_converter_matches_use_case_function_pointer() {
        let to_values_case: ToValues = values_converter::convert;
        let values = to_values_case(name_values_projection()).unwrap();

        assert_eq!(values.len(), 3);
    }

    #[test]
    fn to_values_returns_collection_of_projected_values() {
        let values = resolve(name_values_projection()).unwrap();

        assert_eq!(
            values.items(),
            &[
                ProjectedValue::name("README.md"),
                ProjectedValue::name("src"),
                ProjectedValue::name("notes.txt"),
            ]
        );
    }

    #[test]
    fn to_values_accepts_empty_rows() {
        let values = resolve(projection(vec![SelectProperty::Name], Vec::new())).unwrap();

        assert!(values.is_empty());
    }

    #[test]
    fn to_values_rejects_empty_properties() {
        let result = resolve(projection(Vec::new(), Vec::new()));

        assert!(matches!(
            result,
            Err(ToValuesError::InvalidPropertyCount { actual }) if actual == 0
        ));
    }

    #[test]
    fn to_values_rejects_multiple_properties() {
        let result = resolve(projection(
            vec![SelectProperty::Name, SelectProperty::Size],
            vec![ProjectedRow::new(vec![
                ProjectedValue::name("README.md"),
                ProjectedValue::size(Some(1)),
            ])],
        ));

        assert!(matches!(
            result,
            Err(ToValuesError::InvalidPropertyCount { actual }) if actual == 2
        ));
    }

    #[test]
    fn to_values_preserves_optional_absence_and_row_order() {
        let values = resolve(projection(
            vec![SelectProperty::Size],
            vec![
                ProjectedRow::new(vec![ProjectedValue::size(Some(10))]),
                ProjectedRow::new(vec![ProjectedValue::size(None)]),
                ProjectedRow::new(vec![ProjectedValue::size(Some(30))]),
            ],
        ))
        .unwrap();

        assert_eq!(
            values.items(),
            &[
                ProjectedValue::Size(Some(10)),
                ProjectedValue::Size(None),
                ProjectedValue::Size(Some(30)),
            ]
        );
    }

    #[test]
    fn to_values_rejects_inconsistent_row_width() {
        let result = resolve(projection(
            vec![SelectProperty::Name],
            vec![ProjectedRow::new(vec![
                ProjectedValue::name("README.md"),
                ProjectedValue::size(Some(1)),
            ])],
        ));

        assert!(matches!(
            result,
            Err(ToValuesError::InconsistentRowWidth { row, actual }) if row == 0 && actual == 2
        ));
    }
}
