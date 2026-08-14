use crate::definitions::structs::borrowed::filesystem_item::FilesystemItem;
use crate::definitions::structs::owned::flow::Flow;

pub type Request = for<'item> fn(FilesystemItem<'item>) -> Flow;
