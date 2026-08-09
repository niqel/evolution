use crate::definitions::callbacks::consume_scope;
use crate::definitions::contracts::write_terminal;
use crate::definitions::value_objects::scope::Scope;
use crate::resolvers::terminal_writer;

pub fn consume(write: write_terminal::Write, scope: Scope<'_>) -> Result<(), consume_scope::Error> {
    terminal_writer::resolve(write, scope.server)
        .map_err(|_| consume_scope::Error::TerminalUnavailable)?;
    terminal_writer::resolve(write, "/").map_err(|_| consume_scope::Error::TerminalUnavailable)?;
    terminal_writer::resolve(write, scope.user)
        .map_err(|_| consume_scope::Error::TerminalUnavailable)?;

    write_path(write, scope.source, scope.item)?;

    terminal_writer::resolve(write, "\n").map_err(|_| consume_scope::Error::TerminalUnavailable)?;

    Ok(())
}

fn write_path(
    write: write_terminal::Write,
    source: &str,
    item: Option<&str>,
) -> Result<(), consume_scope::Error> {
    if source == "/" {
        if item.is_none() {
            terminal_writer::resolve(write, "/")
                .map_err(|_| consume_scope::Error::TerminalUnavailable)?;
        }
    } else {
        let clean_source = trim_slashes(source);
        if !clean_source.is_empty() {
            terminal_writer::resolve(write, "/")
                .map_err(|_| consume_scope::Error::TerminalUnavailable)?;
            terminal_writer::resolve(write, clean_source)
                .map_err(|_| consume_scope::Error::TerminalUnavailable)?;
        }
    }

    if let Some(item_path) = item {
        let clean_item = trim_leading_slashes(item_path);
        if !clean_item.is_empty() {
            terminal_writer::resolve(write, "/")
                .map_err(|_| consume_scope::Error::TerminalUnavailable)?;
            terminal_writer::resolve(write, clean_item)
                .map_err(|_| consume_scope::Error::TerminalUnavailable)?;
        }
    }

    Ok(())
}

fn trim_leading_slashes(s: &str) -> &str {
    s.trim_start_matches('/')
}

fn trim_slashes(s: &str) -> &str {
    s.trim_matches('/')
}
