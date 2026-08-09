use crate::collaborators::{about_content, welcome_content};
use crate::definitions::contracts::write_terminal;
use crate::definitions::use_cases::welcome;
use crate::resolvers::terminal_writer;

pub fn welcome(write: write_terminal::Write) -> Result<(), welcome::Error> {
    terminal_writer::resolve(write, welcome_content::COMPANY)
        .map_err(|_| welcome::Error::TerminalUnavailable)?;
    terminal_writer::resolve(write, "\n\n").map_err(|_| welcome::Error::TerminalUnavailable)?;
    terminal_writer::resolve(write, about_content::NAME)
        .map_err(|_| welcome::Error::TerminalUnavailable)?;
    terminal_writer::resolve(write, "\nVersion ")
        .map_err(|_| welcome::Error::TerminalUnavailable)?;
    terminal_writer::resolve(write, about_content::VERSION)
        .map_err(|_| welcome::Error::TerminalUnavailable)?;
    terminal_writer::resolve(write, "\n").map_err(|_| welcome::Error::TerminalUnavailable)?;
    terminal_writer::resolve(write, about_content::DESCRIPTION)
        .map_err(|_| welcome::Error::TerminalUnavailable)?;
    terminal_writer::resolve(write, "\n\n").map_err(|_| welcome::Error::TerminalUnavailable)?;
    terminal_writer::resolve(write, welcome_content::MESSAGE)
        .map_err(|_| welcome::Error::TerminalUnavailable)?;
    terminal_writer::resolve(write, "\n").map_err(|_| welcome::Error::TerminalUnavailable)?;

    Ok(())
}
