use std::io::{self, Write};

use crate::definitions::use_cases::welcome_presenter::WelcomePresenterError;
use crate::presentation_style::{FILE_STYLE, LOCATION_STYLE, PRIMARY_STYLE, RESET};

pub fn provide(version: &str) -> Result<(), WelcomePresenterError> {
    let mut stdout = io::stdout();
    provide_to(&mut stdout, version)?;
    stdout.flush().map_err(WelcomePresenterError::from)
}

pub(crate) fn provide_to(
    writer: &mut impl Write,
    version: &str,
) -> Result<(), WelcomePresenterError> {
    write!(writer, "{}CatarinaSoft{}\n", PRIMARY_STYLE, RESET)?;
    write!(
        writer,
        "{}evo-shell{} {}{}{}\n",
        LOCATION_STYLE, RESET, FILE_STYLE, version, RESET
    )?;
    write!(writer, "{}evo-shell is a life :){}\n\n", FILE_STYLE, RESET)?;

    Ok(())
}
