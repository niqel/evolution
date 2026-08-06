use std::path::Path;

use crate::definitions::domain::entities::filesystem_scope::FilesystemScope;
use crate::definitions::use_cases::copy_filesystem_entries::CopyError;
use crate::resolvers::filesystem_copy;

pub fn copy(
    scope: &FilesystemScope,
    sources: &[&Path],
    destination: &Path,
) -> Result<(), CopyError> {
    filesystem_copy::resolve(scope, sources, destination)
}
