use evo_shell_engine::scope_setter;

use crate::definitions::domain::entities::shell::Shell;
use crate::definitions::use_cases::initialize_shell::InitializeShellError;
use crate::providers;
use crate::resolvers::shell;

pub fn initialize() -> Result<Shell, InitializeShellError> {
    shell::resolve(providers::current_directory::provide, scope_setter::set)
}
