use crate::collaborators::{shell_informant, welcome_content};
use crate::definitions::contracts::write_terminal;
use crate::definitions::use_cases::welcome;
use crate::resolvers::terminal_writer;

pub fn welcome(write: write_terminal::Write) -> Result<(), welcome::Error> {
    let information = shell_informant::collaborate();

    terminal_writer::resolve(write, welcome_content::COMPANY)
        .map_err(|_| welcome::Error::TerminalUnavailable)?;
    terminal_writer::resolve(write, "\n\n").map_err(|_| welcome::Error::TerminalUnavailable)?;
    terminal_writer::resolve(write, information.name)
        .map_err(|_| welcome::Error::TerminalUnavailable)?;
    terminal_writer::resolve(write, "\nVersion ")
        .map_err(|_| welcome::Error::TerminalUnavailable)?;
    terminal_writer::resolve(write, information.version)
        .map_err(|_| welcome::Error::TerminalUnavailable)?;
    terminal_writer::resolve(write, "\n").map_err(|_| welcome::Error::TerminalUnavailable)?;
    terminal_writer::resolve(write, information.description)
        .map_err(|_| welcome::Error::TerminalUnavailable)?;
    terminal_writer::resolve(write, "\n\n").map_err(|_| welcome::Error::TerminalUnavailable)?;
    terminal_writer::resolve(write, welcome_content::MESSAGE)
        .map_err(|_| welcome::Error::TerminalUnavailable)?;
    terminal_writer::resolve(write, "\n").map_err(|_| welcome::Error::TerminalUnavailable)?;

    Ok(())
}
