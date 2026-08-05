use crate::definitions::providers::welcome_presenter::Provide;
use crate::definitions::use_cases::welcome_presenter::WelcomePresenterError;

pub type Resolve = fn(Provide) -> Result<(), WelcomePresenterError>;
