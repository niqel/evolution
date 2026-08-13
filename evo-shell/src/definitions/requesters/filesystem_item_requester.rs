use crate::definitions::structs::borrowed::filesystem_item::FilesystemItem;
use crate::definitions::structs::flow::Flow;

pub type Request = for<'item> fn(FilesystemItem<'item>) -> Flow;
