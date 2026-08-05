use std::io;

pub type WelcomePresenter = fn() -> Result<(), WelcomePresenterError>;

#[derive(Debug)]
pub enum WelcomePresenterError {
    Io(io::Error),
}

impl From<io::Error> for WelcomePresenterError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}
