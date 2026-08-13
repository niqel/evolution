#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilesystemField {
    Index,
    Name,
    Path,
    Kind,
    Size,
}

pub const FILESYSTEM_FIELDS: &[FilesystemField] = &[
    FilesystemField::Index,
    FilesystemField::Name,
    FilesystemField::Path,
    FilesystemField::Kind,
    FilesystemField::Size,
];
