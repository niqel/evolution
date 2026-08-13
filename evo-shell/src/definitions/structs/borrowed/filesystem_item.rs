use crate::definitions::structs::filesystem_item_kind::FilesystemItemKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FilesystemItem<'item> {
    pub index: usize,
    pub name: &'item str,
    pub path: &'item str,
    pub kind: FilesystemItemKind,
    pub size: Option<u64>,
}
