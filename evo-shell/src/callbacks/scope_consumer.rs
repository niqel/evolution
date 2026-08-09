use crate::definitions::callbacks::consume_scope;
use crate::definitions::contracts::write_terminal;
use crate::resolvers::terminal_writer;

pub fn consume(
    write: write_terminal::Write,
    scope_type: &str,
    location: &str,
) -> Result<(), consume_scope::Error> {
    let last_segment = extract_last_segment(location);

    terminal_writer::resolve(write, "scope-")
        .map_err(|_| consume_scope::Error::TerminalUnavailable)?;
    terminal_writer::resolve(write, scope_type)
        .map_err(|_| consume_scope::Error::TerminalUnavailable)?;
    terminal_writer::resolve(write, " ").map_err(|_| consume_scope::Error::TerminalUnavailable)?;
    terminal_writer::resolve(write, "…/").map_err(|_| consume_scope::Error::TerminalUnavailable)?;
    terminal_writer::resolve(write, last_segment)
        .map_err(|_| consume_scope::Error::TerminalUnavailable)?;
    terminal_writer::resolve(write, ">").map_err(|_| consume_scope::Error::TerminalUnavailable)?;

    Ok(())
}

fn extract_last_segment(location: &str) -> &str {
    if location.is_empty() {
        return "";
    }
    let bytes = location.as_bytes();
    let mut end = bytes.len();
    while end > 1 && bytes[end - 1] == b'/' {
        end -= 1;
    }
    let slice = &location[..end];
    if let Some(pos) = slice.rfind('/') {
        &slice[pos + 1..]
    } else {
        slice
    }
}
