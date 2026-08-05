use crate::definitions::use_cases::welcome_presenter::WelcomePresenterError;

pub type Provide = fn(&str) -> Result<(), WelcomePresenterError>;
