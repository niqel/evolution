use std::fs;
use std::io::ErrorKind;
use std::path::Path;

use crate::definitions::domain::entities::filesystem_scope::FilesystemScope;
use crate::definitions::use_cases::copy_filesystem_entries::CopyError;
use crate::resolvers::filesystem_path;

pub fn resolve(
    scope: &FilesystemScope,
    sources: &[&Path],
    destination: &Path,
) -> Result<(), CopyError> {
    let dest_path = filesystem_path::resolve(scope, destination);

    let dest_meta = match fs::metadata(&dest_path) {
        Ok(meta) => meta,
        Err(err) if err.kind() == ErrorKind::NotFound => {
            return Err(CopyError::DestinationNotFound(dest_path));
        }
        Err(err) => return Err(CopyError::Filesystem(err)),
    };

    if !dest_meta.is_dir() {
        return Err(CopyError::DestinationNotDirectory(dest_path));
    }

    let dest_canonical = dest_path.canonicalize().map_err(CopyError::Filesystem)?;

    for source in sources {
        let source_path = filesystem_path::resolve(scope, source);

        let source_meta = match fs::symlink_metadata(&source_path) {
            Ok(meta) => meta,
            Err(err) if err.kind() == ErrorKind::NotFound => {
                return Err(CopyError::SourceNotFound(source_path));
            }
            Err(err) => return Err(CopyError::Filesystem(err)),
        };

        if source_meta.is_symlink() {
            return Err(CopyError::UnsupportedSourceType(source_path));
        }

        let source_name = source_path
            .file_name()
            .ok_or_else(|| CopyError::Filesystem(std::io::Error::other("invalid source path")))?;

        let target_path = dest_path.join(source_name);

        if source_meta.is_file() {
            if fs::symlink_metadata(&target_path).is_ok() {
                return Err(CopyError::DestinationAlreadyExists(target_path));
            }
        } else if source_meta.is_dir() {
            let source_canonical = source_path.canonicalize().map_err(CopyError::Filesystem)?;

            if dest_canonical == source_canonical || dest_canonical.starts_with(&source_canonical) {
                return Err(CopyError::RecursiveSelfCopy(source_path));
            }

            if fs::symlink_metadata(&target_path).is_ok() {
                return Err(CopyError::DestinationAlreadyExists(target_path));
            }
        }
    }

    for source in sources {
        let source_path = filesystem_path::resolve(scope, source);
        let source_meta = fs::symlink_metadata(&source_path).map_err(CopyError::Filesystem)?;
        let source_name = source_path
            .file_name()
            .ok_or_else(|| CopyError::Filesystem(std::io::Error::other("invalid source path")))?;
        let target_path = dest_path.join(source_name);

        if source_meta.is_file() {
            fs::copy(&source_path, &target_path).map_err(CopyError::Filesystem)?;
        } else if source_meta.is_dir() {
            copy_directory_recursive(&source_path, &target_path)?;
        }
    }

    Ok(())
}

fn copy_directory_recursive(src: &Path, dst: &Path) -> Result<(), CopyError> {
    fs::create_dir(dst).map_err(CopyError::Filesystem)?;

    let entries = fs::read_dir(src).map_err(CopyError::Filesystem)?;

    for entry in entries {
        let entry = entry.map_err(CopyError::Filesystem)?;
        let entry_path = entry.path();
        let entry_meta = fs::symlink_metadata(&entry_path).map_err(CopyError::Filesystem)?;
        let child_dst = dst.join(entry.file_name());

        if entry_meta.is_symlink() {
            return Err(CopyError::UnsupportedSourceType(entry_path));
        }

        if entry_meta.is_file() {
            fs::copy(&entry_path, &child_dst).map_err(CopyError::Filesystem)?;
        } else if entry_meta.is_dir() {
            copy_directory_recursive(&entry_path, &child_dst)?;
        }
    }

    Ok(())
}
