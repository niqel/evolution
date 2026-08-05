use crate::definitions::contracts::next_directory_entry::NextDirectoryEntry;
use crate::definitions::domain::entities::filesystem_entry::{
    FilesystemEntry, FilesystemEntryKind,
};
use crate::definitions::domain::entities::filesystem_iteration::FilesystemIteration;
use crate::definitions::use_cases::iter::IterError;

pub fn resolve(
    iteration: &mut FilesystemIteration,
    next_directory_entry: NextDirectoryEntry,
) -> Result<Option<FilesystemEntry>, IterError> {
    let Some(entry) = next_directory_entry(iteration).map_err(IterError::NextEntry)? else {
        return Ok(None);
    };

    let file_type = entry.file_type().map_err(IterError::MaterializeEntry)?;
    let metadata = entry.metadata().map_err(IterError::MaterializeEntry)?;
    let kind = if file_type.is_file() {
        FilesystemEntryKind::File
    } else if file_type.is_dir() {
        FilesystemEntryKind::Directory
    } else if file_type.is_symlink() {
        FilesystemEntryKind::Symlink
    } else {
        FilesystemEntryKind::Other
    };
    let created = metadata.created().ok();
    let modified = metadata.modified().ok();
    let size = if kind == FilesystemEntryKind::File {
        Some(metadata.len())
    } else {
        None
    };

    Ok(Some(FilesystemEntry::new(
        entry.file_name(),
        entry.path(),
        kind,
        created,
        modified,
        size,
    )))
}
