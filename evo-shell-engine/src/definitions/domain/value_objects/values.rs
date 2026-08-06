use std::ops::Deref;

use crate::definitions::domain::value_objects::select::ProjectedValue;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Values {
    items: Vec<ProjectedValue>,
}

impl Values {
    pub fn new(items: Vec<ProjectedValue>) -> Self {
        Self { items }
    }

    pub fn items(&self) -> &[ProjectedValue] {
        &self.items
    }

    pub fn into_items(self) -> Vec<ProjectedValue> {
        self.items
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl Deref for Values {
    type Target = [ProjectedValue];

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}
