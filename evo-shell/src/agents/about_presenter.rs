use crate::collaborators::about_content;
use crate::definitions::contracts::write_terminal;
use crate::definitions::use_cases::present_about;
use crate::resolvers::terminal_writer;

pub fn present(write: write_terminal::Write) -> Result<(), present_about::Error> {
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
    fn about_presenter_success() {
        BUFFER_LEN.store(0, Ordering::SeqCst);

        let result = present(mock_write_capture);
        assert_eq!(result, Ok(()));

        let len = BUFFER_LEN.load(Ordering::SeqCst);
        let output = unsafe {
            let slice = core::slice::from_raw_parts(BUFFER.0.get() as *const u8, len);
            core::str::from_utf8(slice).unwrap()
        };
        assert_eq!(
            output,
            "Evolution Shell\nVersion 0.1.0\nA lightweight functional shell.\n"
        );
    }

    #[test]
    fn about_presenter_translates_terminal_error() {
        assert_eq!(
            present(mock_write_fail),
            Err(present_about::Error::TerminalUnavailable)
        );
    }
}
