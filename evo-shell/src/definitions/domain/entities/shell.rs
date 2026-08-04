use evo_shell_engine::FilesystemScope;

#[derive(Debug)]
pub struct Shell {
    filesystem_scope: FilesystemScope,
}

impl Shell {
    pub(crate) fn new(filesystem_scope: FilesystemScope) -> Self {
        Self { filesystem_scope }
    }

    pub fn filesystem_scope(&self) -> &FilesystemScope {
        &self.filesystem_scope
    }

    pub(crate) fn replace_filesystem_scope(&mut self, filesystem_scope: FilesystemScope) {
        self.filesystem_scope = filesystem_scope;
    }
}
