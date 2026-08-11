use crate::collaborators::about_content;
use crate::definitions::contracts::write_terminal;
use crate::definitions::use_cases::present_about;
use crate::resolvers::terminal_writer;

pub fn report(write: write_terminal::Write) -> Result<(), present_about::Error> {
    terminal_writer::resolve(write, about_content::NAME)
        .map_err(|_| present_about::Error::TerminalUnavailable)?;
    terminal_writer::resolve(write, "\nVersion ")
        .map_err(|_| present_about::Error::TerminalUnavailable)?;
    terminal_writer::resolve(write, about_content::VERSION)
        .map_err(|_| present_about::Error::TerminalUnavailable)?;
    terminal_writer::resolve(write, "\n").map_err(|_| present_about::Error::TerminalUnavailable)?;
    terminal_writer::resolve(write, about_content::DESCRIPTION)
        .map_err(|_| present_about::Error::TerminalUnavailable)?;
    terminal_writer::resolve(write, "\n").map_err(|_| present_about::Error::TerminalUnavailable)?;

    Ok(())
}
