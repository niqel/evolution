use crate::definitions::domain::value_objects::select::{ProjectedValue, StructuredProjection};
use crate::definitions::use_cases::to_value::ToValueError;

pub fn resolve(projection: StructuredProjection) -> Result<ProjectedValue, ToValueError> {
    let row_count = projection.row_count();
    let property_count = projection.property_count();

    if property_count == 0 {
        return Err(ToValueError::NoProperties);
    }

    if property_count > 1 {
        return Err(ToValueError::MultipleProperties {
            actual: property_count,
        });
    }

    if row_count == 0 {
        return Err(ToValueError::NoRows);
    }

    if row_count > 1 {
        return Err(ToValueError::MultipleRows { actual: row_count });
    }

    let mut rows = projection.into_rows().into_iter();
    let row = rows.next().expect("row_count checked to be exactly one");

    if row.len() != 1 {
        return Err(ToValueError::InconsistentRowWidth {
            row: 0,
            actual: row.len(),
        });
    }

    let mut values = row.into_values().into_iter();
    Ok(values.next().expect("row width checked to be exactly one"))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::agents::value_converter;
    use crate::definitions::domain::value_objects::select::{
        ProjectedRow, ProjectedValue, SelectProperty, StructuredProjection,
    };
    use crate::definitions::use_cases::to_value::{ToValue, ToValueError};
    use std::ffi::OsString;

    fn projection(
        properties: Vec<SelectProperty>,
        rows: Vec<ProjectedRow>,
    ) -> StructuredProjection {
        StructuredProjection::new(properties, rows)
    }

    fn single_value_projection(value: ProjectedValue) -> StructuredProjection {
        projection(
            vec![SelectProperty::Name],
            vec![ProjectedRow::new(vec![value])],
        )
    }

    #[test]
    fn value_converter_matches_use_case_function_pointer() {
        let to_value_case: ToValue = value_converter::convert;

        let value =
            to_value_case(single_value_projection(ProjectedValue::name("README.md"))).unwrap();

        assert_eq!(value, ProjectedValue::Name(OsString::from("README.md")));
    }

    #[test]
    fn to_value_returns_single_projected_value() {
        let value = resolve(single_value_projection(ProjectedValue::index(3))).unwrap();

        assert_eq!(value, ProjectedValue::Index(3));
    }

    #[test]
    fn to_value_accepts_optional_absence_as_value() {
        let value = resolve(projection(
            vec![SelectProperty::Size],
            vec![ProjectedRow::new(vec![ProjectedValue::Size(None)])],
        ))
        .unwrap();

        assert_eq!(value, ProjectedValue::Size(None));
    }

    #[test]
    fn to_value_rejects_empty_rows() {
        let result = resolve(projection(vec![SelectProperty::Name], Vec::new()));

        assert!(matches!(result, Err(ToValueError::NoRows)));
    }

    #[test]
    fn to_value_rejects_multiple_rows() {
        let result = resolve(projection(
            vec![SelectProperty::Name],
            vec![
                ProjectedRow::new(vec![ProjectedValue::name("a")]),
                ProjectedRow::new(vec![ProjectedValue::name("b")]),
            ],
        ));

        assert!(matches!(
            result,
            Err(ToValueError::MultipleRows { actual }) if actual == 2
        ));
    }

    #[test]
    fn to_value_rejects_empty_properties() {
        let result = resolve(projection(
            Vec::new(),
            vec![ProjectedRow::new(vec![ProjectedValue::name("a")])],
        ));

        assert!(matches!(result, Err(ToValueError::NoProperties)));
    }

    #[test]
    fn to_value_rejects_multiple_properties() {
        let result = resolve(projection(
            vec![SelectProperty::Name, SelectProperty::Size],
            vec![ProjectedRow::new(vec![
                ProjectedValue::name("a"),
                ProjectedValue::size(Some(1)),
            ])],
        ));

        assert!(matches!(
            result,
            Err(ToValueError::MultipleProperties { actual }) if actual == 2
        ));
    }
}
