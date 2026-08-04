use evo_shell_engine::SetFilesystemScope;

use crate::definitions::contracts::current_directory::CurrentDirectory;
use crate::definitions::domain::entities::shell::Shell;
use crate::definitions::use_cases::initialize_shell::InitializeShellError;

pub fn resolve(
    current_directory: CurrentDirectory,
    set_filesystem_scope: SetFilesystemScope,
) -> Result<Shell, InitializeShellError> {
    let path = current_directory().map_err(InitializeShellError::CurrentDirectory)?;
    let filesystem_scope =
        set_filesystem_scope(path.as_path()).map_err(InitializeShellError::Scope)?;

    Ok(Shell::new(filesystem_scope))
}
