use evo_shell::definitions::contracts::write_terminal;
use std::sync::Mutex;

static SERIAL_LOCK: Mutex<()> = Mutex::new(());
static STATE_LOCK: Mutex<CaptureState> = Mutex::new(CaptureState {
    buffer: String::new(),
    should_fail: false,
});

struct CaptureState {
    buffer: String,
    should_fail: bool,
}

pub fn mock_write_capture(content: &str) -> Result<(), write_terminal::Error> {
    let mut state = STATE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    if state.should_fail {
        Err(write_terminal::Error::Unavailable)
    } else {
        state.buffer.push_str(content);
        Ok(())
    }
}

pub fn mock_write_fail(_content: &str) -> Result<(), write_terminal::Error> {
    Err(write_terminal::Error::Unavailable)
}

pub fn run_with_capture<F, R>(f: F) -> (R, String)
where
    F: FnOnce(write_terminal::Write) -> R,
{
    let _serial = SERIAL_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    {
        let mut state = STATE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        state.buffer.clear();
        state.should_fail = false;
    }

    let result = f(mock_write_capture);

    let output = {
        let state = STATE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        state.buffer.clone()
    };
    (result, output)
}

pub fn run_with_fail<F, R>(f: F) -> R
where
    F: FnOnce(write_terminal::Write) -> R,
{
    let _serial = SERIAL_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    {
        let mut state = STATE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        state.buffer.clear();
        state.should_fail = true;
    }
    f(mock_write_fail)
}
