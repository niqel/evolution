use crate::definitions::domain::entities::shell::Shell;
use crate::definitions::use_cases::initialize_shell::{InitializeShell, InitializeShellError};
use crate::definitions::use_cases::terminal_clearer::{TerminalClearError, TerminalClearer};
use crate::definitions::use_cases::welcome_presenter::{WelcomePresenter, WelcomePresenterError};

pub type Start =
    fn(InitializeShell, TerminalClearer, WelcomePresenter) -> Result<Shell, StartError>;

#[derive(Debug)]
pub enum StartError {
    InitializeShell(InitializeShellError),
    TerminalClear(TerminalClearError),
    WelcomePresent(WelcomePresenterError),
}

impl From<InitializeShellError> for StartError {
    fn from(error: InitializeShellError) -> Self {
        Self::InitializeShell(error)
    }
}

impl From<TerminalClearError> for StartError {
    fn from(error: TerminalClearError) -> Self {
        Self::TerminalClear(error)
    }
}

impl From<WelcomePresenterError> for StartError {
    fn from(error: WelcomePresenterError) -> Self {
        Self::WelcomePresent(error)
    }
}
