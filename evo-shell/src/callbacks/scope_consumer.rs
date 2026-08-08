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

#[cfg(test)]
mod tests {
    use super::*;
    use core::cell::UnsafeCell;
    use core::sync::atomic::{AtomicUsize, Ordering};

    struct TestBuffer(UnsafeCell<[u8; 128]>);
    unsafe impl Sync for TestBuffer {}

    static BUFFER: TestBuffer = TestBuffer(UnsafeCell::new([0; 128]));
    static BUFFER_LEN: AtomicUsize = AtomicUsize::new(0);

    fn mock_write_capture(content: &str) -> Result<(), write_terminal::Error> {
        let bytes = content.as_bytes();
        let len = BUFFER_LEN.load(Ordering::SeqCst);
        if len + bytes.len() <= 128 {
            unsafe {
                let ptr = (BUFFER.0.get() as *mut u8).add(len);
                core::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len());
            }
            BUFFER_LEN.store(len + bytes.len(), Ordering::SeqCst);
            Ok(())
        } else {
            Err(write_terminal::Error::Unavailable)
        }
    }

    fn mock_write_fail(_content: &str) -> Result<(), write_terminal::Error> {
        Err(write_terminal::Error::Unavailable)
    }

    #[test]
    fn scope_consumer_writes_fragments_in_order() {
        BUFFER_LEN.store(0, Ordering::SeqCst);

        let result = consume(mock_write_capture, "fs", "/home/user/downloads");
        assert_eq!(result, Ok(()));

        let len = BUFFER_LEN.load(Ordering::SeqCst);
        let output = unsafe {
            let slice = core::slice::from_raw_parts(BUFFER.0.get() as *const u8, len);
            core::str::from_utf8(slice).unwrap()
        };
        assert_eq!(output, "scope-fs …/downloads>");
    }

    #[test]
    fn scope_consumer_handles_root_location() {
        BUFFER_LEN.store(0, Ordering::SeqCst);

        let result = consume(mock_write_capture, "fs", "/");
        assert_eq!(result, Ok(()));

        let len = BUFFER_LEN.load(Ordering::SeqCst);
        let output = unsafe {
            let slice = core::slice::from_raw_parts(BUFFER.0.get() as *const u8, len);
            core::str::from_utf8(slice).unwrap()
        };
        assert_eq!(output, "scope-fs …/>");
    }

    #[test]
    fn scope_consumer_translates_writer_error() {
        let result = consume(mock_write_fail, "fs", "/home/user/downloads");
        assert_eq!(result, Err(consume_scope::Error::TerminalUnavailable));
    }
}
