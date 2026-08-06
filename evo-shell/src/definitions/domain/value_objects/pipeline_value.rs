use std::rc::Rc;

use evo_shell_engine::{
    Arguments, FilesystemIterationItem, ProjectedValue, StructuredItems, StructuredProjection,
    Values,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineValueKind {
    StructuredItems,
    StructuredProjection,
    Value,
    Values,
    Arguments,
}

#[derive(Debug, Clone)]
pub struct PipelineItems {
    items: Vec<Rc<FilesystemIterationItem>>,
    selection: Vec<usize>,
}

impl PipelineItems {
    pub fn new(items: Vec<FilesystemIterationItem>) -> Self {
        let items = items.into_iter().map(Rc::new).collect::<Vec<_>>();
        let selection = (0..items.len()).collect::<Vec<_>>();

        Self { items, selection }
    }

    pub fn with_selection(items: Vec<Rc<FilesystemIterationItem>>, selection: Vec<usize>) -> Self {
        Self { items, selection }
    }

    pub fn items(&self) -> &[Rc<FilesystemIterationItem>] {
        &self.items
    }

    pub fn into_items(self) -> Vec<Rc<FilesystemIterationItem>> {
        self.items
    }

    pub fn selection(&self) -> &[usize] {
        &self.selection
    }

    pub fn len(&self) -> usize {
        self.selection.len()
    }

    pub fn is_empty(&self) -> bool {
        self.selection.is_empty()
    }

    pub fn structured_items(&self) -> StructuredItems<'_> {
        StructuredItems::new(
            self.selection
                .iter()
                .map(|index| self.items[*index].as_ref())
                .collect(),
        )
    }

    pub fn selection_from(&self, selected: StructuredItems<'_>) -> Vec<usize> {
        selected
            .iter()
            .map(|item| {
                self.items
                    .iter()
                    .position(|candidate| std::ptr::eq(candidate.as_ref(), item))
                    .expect("selected item should belong to the current pipeline state")
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
pub enum PipelineValue {
    StructuredItems(PipelineItems),
    StructuredProjection(StructuredProjection),
    Value(ProjectedValue),
    Values(Values),
    Arguments(Arguments),
}

impl PipelineValue {
    pub fn kind(&self) -> PipelineValueKind {
        match self {
            Self::StructuredItems(_) => PipelineValueKind::StructuredItems,
            Self::StructuredProjection(_) => PipelineValueKind::StructuredProjection,
            Self::Value(_) => PipelineValueKind::Value,
            Self::Values(_) => PipelineValueKind::Values,
            Self::Arguments(_) => PipelineValueKind::Arguments,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::definitions::domain::entities::shell::Shell;
    use evo_shell_engine::{
        ProjectedRow, SelectProperty, iteration_advancer, iterator, scope_setter,
    };
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_directory(name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after UNIX_EPOCH")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "evo_shell_pipeline_value_{name}_{}_{}",
            std::process::id(),
            unique
        ));
        fs::create_dir(&path).expect("temporary test directory should be created");
        path
    }

    fn shell_from_path(path: &PathBuf) -> Shell {
        Shell::new(scope_setter::set(path.as_path()).unwrap())
    }

    fn materialize_items(directory_name: &str) -> PipelineItems {
        let directory = temp_directory(directory_name);
        fs::write(directory.join("first.txt"), b"one").unwrap();
        fs::write(directory.join("second.txt"), b"two").unwrap();

        let shell = shell_from_path(&directory);
        let mut iteration = iterator::iter(shell.filesystem_scope()).unwrap();
        let mut items = Vec::new();

        while let Some(item) = iteration_advancer::advance(&mut iteration).unwrap() {
            items.push(item);
        }

        PipelineItems::new(items)
    }

    #[test]
    fn pipeline_items_preserve_selection_order() {
        let pipeline_items = materialize_items("selection_order");

        assert_eq!(pipeline_items.len(), 2);
        assert_eq!(pipeline_items.selection(), &[0, 1]);
    }

    #[test]
    fn structured_items_view_preserves_selected_order() {
        let pipeline_items = materialize_items("selected_order");
        let view = pipeline_items.structured_items();

        assert_eq!(view.len(), 2);
        assert_eq!(view.items()[0].index(), 0);
        assert_eq!(view.items()[1].index(), 1);
    }

    #[test]
    fn pipeline_value_kind_matches_variant() {
        let projection = StructuredProjection::new(
            vec![SelectProperty::Name],
            vec![ProjectedRow::new(vec![ProjectedValue::name("a")])],
        );

        assert_eq!(
            PipelineValue::StructuredProjection(projection).kind(),
            PipelineValueKind::StructuredProjection
        );
    }
}
