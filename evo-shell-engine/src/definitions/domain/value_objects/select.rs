use std::ffi::OsString;
use std::time::SystemTime;

use crate::definitions::domain::entities::filesystem_entry::FilesystemEntryKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectProperty {
    Index,
    Created,
    Modified,
    Type,
    Size,
    Name,
    Unsupported(String),
}

impl SelectProperty {
    pub fn unsupported(value: impl Into<String>) -> Self {
        Self::Unsupported(value.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectedValue {
    Index(usize),
    Created(Option<SystemTime>),
    Modified(Option<SystemTime>),
    Type(FilesystemEntryKind),
    Size(Option<u64>),
    Name(OsString),
}

impl ProjectedValue {
    pub fn index(value: usize) -> Self {
        Self::Index(value)
    }

    pub fn created(value: Option<SystemTime>) -> Self {
        Self::Created(value)
    }

    pub fn modified(value: Option<SystemTime>) -> Self {
        Self::Modified(value)
    }

    pub fn kind(value: FilesystemEntryKind) -> Self {
        Self::Type(value)
    }

    pub fn size(value: Option<u64>) -> Self {
        Self::Size(value)
    }

    pub fn name(value: impl Into<OsString>) -> Self {
        Self::Name(value.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectedRow {
    values: Vec<ProjectedValue>,
}

impl ProjectedRow {
    pub fn new(values: Vec<ProjectedValue>) -> Self {
        Self { values }
    }

    pub fn values(&self) -> &[ProjectedValue] {
        &self.values
    }

    pub fn into_values(self) -> Vec<ProjectedValue> {
        self.values
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuredProjection {
    properties: Vec<SelectProperty>,
    rows: Vec<ProjectedRow>,
}

impl StructuredProjection {
    pub fn new(properties: Vec<SelectProperty>, rows: Vec<ProjectedRow>) -> Self {
        Self { properties, rows }
    }

    pub fn properties(&self) -> &[SelectProperty] {
        &self.properties
    }

    pub fn rows(&self) -> &[ProjectedRow] {
        &self.rows
    }

    pub fn property_count(&self) -> usize {
        self.properties.len()
    }

    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    pub fn into_rows(self) -> Vec<ProjectedRow> {
        self.rows
    }

    pub fn into_properties(self) -> Vec<SelectProperty> {
        self.properties
    }
}
