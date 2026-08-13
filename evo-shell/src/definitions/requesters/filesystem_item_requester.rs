use crate::definitions::structs::borrowed::filesystem_item::FilesystemItem;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flow {
    Continue,
    Stop,
}

pub type Request = for<'item> fn(FilesystemItem<'item>) -> Flow;
