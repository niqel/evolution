use crate::definitions::domain::entities::filesystem_iteration_item::FilesystemIterationItem;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct StructuredItems<'a> {
    items: Vec<&'a FilesystemIterationItem>,
}

impl<'a> StructuredItems<'a> {
    pub fn new(items: Vec<&'a FilesystemIterationItem>) -> Self {
        Self { items }
    }

    pub fn from_slice(items: &'a [FilesystemIterationItem]) -> Self {
        Self {
            items: items.iter().collect(),
        }
    }

    pub fn single(item: &'a FilesystemIterationItem) -> Self {
        Self { items: vec![item] }
    }

    pub fn items(&self) -> &[&'a FilesystemIterationItem] {
        &self.items
    }

    pub fn iter(&self) -> impl Iterator<Item = &'a FilesystemIterationItem> + '_ {
        self.items.iter().copied()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn into_items(self) -> Vec<&'a FilesystemIterationItem> {
        self.items
    }
}

impl<'a> From<&'a [FilesystemIterationItem]> for StructuredItems<'a> {
    fn from(items: &'a [FilesystemIterationItem]) -> Self {
        Self::from_slice(items)
    }
}

impl<'a> From<Vec<&'a FilesystemIterationItem>> for StructuredItems<'a> {
    fn from(items: Vec<&'a FilesystemIterationItem>) -> Self {
        Self::new(items)
    }
}

impl<'a> Deref for StructuredItems<'a> {
    type Target = [&'a FilesystemIterationItem];

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}
