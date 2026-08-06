use evo_shell_engine::scope_setter;

use crate::definitions::domain::entities::shell::Shell;
use crate::definitions::use_cases::initialize_shell::InitializeShellError;
use crate::providers;
use crate::resolvers::shell;

pub fn initialize() -> Result<Shell, InitializeShellError> {
    shell::resolve(providers::current_directory::provide, scope_setter::set)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::definitions::use_cases::initialize_shell::InitializeShell;

    #[test]
    fn shell_initializer_initialize_matches_initialize_shell_function_pointer() {
        let initialize_fn: InitializeShell = initialize;

        let shell = initialize_fn().unwrap();

        assert!(shell.filesystem_scope().path().is_dir());
    }
}
