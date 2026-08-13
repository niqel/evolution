#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilesystemItemKind {
    File,
    Directory,
    Symlink,
    Other,
}
