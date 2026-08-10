use crate::definitions::continuations::consume_scope;
use crate::definitions::contracts::write_terminal;
use crate::definitions::structs::borrowed::scope::Scope;
use crate::resolvers::terminal_writer;

pub fn handle(write: write_terminal::Write, scope: Scope<'_>) -> Result<(), consume_scope::Error> {
    let location = match scope.item {
        Some(item) => item,
        None => scope.source,
    };

    let last_segment = extract_last_segment(location);

    terminal_writer::resolve(write, "scope-")
        .map_err(|_| consume_scope::Error::TerminalUnavailable)?;
    terminal_writer::resolve(write, scope.scope_type)
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
