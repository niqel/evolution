use std::ffi::OsString;
use std::time::SystemTime;

use crate::definitions::domain::entities::filesystem_entry::FilesystemEntryKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FilterProperty {
    Index,
    Created,
    Modified,
    Type,
    Size,
    Name,
    Unsupported(String),
}

impl FilterProperty {
    pub fn unsupported(value: impl Into<String>) -> Self {
        Self::Unsupported(value.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    AtLeast,
    AtMost,
    Between,
    NotBetween,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FilterValue {
    Index(usize),
    Time(SystemTime),
    Type(FilesystemEntryKind),
    Size(u64),
    Name(OsString),
}

impl FilterValue {
    pub fn index(value: usize) -> Self {
        Self::Index(value)
    }

    pub fn time(value: SystemTime) -> Self {
        Self::Time(value)
    }

    pub fn kind(value: FilesystemEntryKind) -> Self {
        Self::Type(value)
    }

    pub fn size(value: u64) -> Self {
        Self::Size(value)
    }

    pub fn name(value: impl Into<OsString>) -> Self {
        Self::Name(value.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FilterOperand {
    Single(FilterValue),
    Range {
        lower: FilterValue,
        upper: FilterValue,
    },
}

impl FilterOperand {
    pub fn single(value: FilterValue) -> Self {
        Self::Single(value)
    }

    pub fn range(lower: FilterValue, upper: FilterValue) -> Self {
        Self::Range { lower, upper }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilterComparison {
    property: FilterProperty,
    operator: FilterOperator,
    operand: FilterOperand,
}

impl FilterComparison {
    pub fn new(property: FilterProperty, operator: FilterOperator, operand: FilterOperand) -> Self {
        Self {
            property,
            operator,
            operand,
        }
    }

    pub fn property(&self) -> &FilterProperty {
        &self.property
    }

    pub fn operator(&self) -> FilterOperator {
        self.operator
    }

    pub fn operand(&self) -> &FilterOperand {
        &self.operand
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FilterExpression {
    Comparison(FilterComparison),
    And(Vec<FilterExpression>),
    Or(Vec<FilterExpression>),
}

impl FilterExpression {
    pub fn comparison(comparison: FilterComparison) -> Self {
        Self::Comparison(comparison)
    }

    pub fn and(expressions: Vec<FilterExpression>) -> Self {
        Self::And(expressions)
    }

    pub fn or(expressions: Vec<FilterExpression>) -> Self {
        Self::Or(expressions)
    }
}
