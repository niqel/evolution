use crate::definitions::structs::borrowed::filesystem_item::FilesystemItem;

pub type Request = for<'item> fn(FilesystemItem<'item>);
