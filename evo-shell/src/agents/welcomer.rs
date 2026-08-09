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

#[cfg(test)]
mod tests {
    use super::*;
    use core::cell::UnsafeCell;
    use core::sync::atomic::{AtomicUsize, Ordering};

    struct TestBuffer(UnsafeCell<[u8; 256]>);
    unsafe impl Sync for TestBuffer {}

    static BUFFER: TestBuffer = TestBuffer(UnsafeCell::new([0; 256]));
    static BUFFER_LEN: AtomicUsize = AtomicUsize::new(0);

    fn mock_write_capture(content: &str) -> Result<(), write_terminal::Error> {
        let bytes = content.as_bytes();
        let len = BUFFER_LEN.load(Ordering::SeqCst);
        if len + bytes.len() <= 256 {
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
    fn welcomer_success() {
        BUFFER_LEN.store(0, Ordering::SeqCst);

        let result = welcome(mock_write_capture);
        assert_eq!(result, Ok(()));

        let len = BUFFER_LEN.load(Ordering::SeqCst);
        let output = unsafe {
            let slice = core::slice::from_raw_parts(BUFFER.0.get() as *const u8, len);
            core::str::from_utf8(slice).unwrap()
        };

        let expected_header = "CatarinaSoft\n\nEvolution Shell\nVersion ";
        let expected_version = about_content::VERSION;
        let expected_footer = "\nA lightweight functional shell.\n\nEvo shell is a life :)\n";

        assert!(output.starts_with(expected_header));
        assert!(output.contains(expected_version));
        assert!(output.ends_with(expected_footer));
    }

    #[test]
    fn welcomer_translates_terminal_error() {
        assert_eq!(
            welcome(mock_write_fail),
            Err(welcome::Error::TerminalUnavailable)
        );
    }
}
