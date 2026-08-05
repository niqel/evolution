use crate::definitions::providers::welcome_presenter::Provide;
use crate::definitions::use_cases::welcome_presenter::WelcomePresenterError;

pub fn resolve(provide: Provide) -> Result<(), WelcomePresenterError> {
    provide(env!("CARGO_PKG_VERSION"))
}
