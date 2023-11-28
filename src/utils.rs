use std::io::stdout;

use crossterm::cursor::SetCursorStyle;
use crossterm::ExecutableCommand;
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};

pub struct RawMode;

impl RawMode {
    pub fn new() -> Result<Self, std::io::Error> {
        enable_raw_mode()?;
        execute!(
            stdout(),
            EnterAlternateScreen,
            SetCursorStyle::BlinkingBar
        )?;

        Ok(RawMode)
    }
}

impl Drop for RawMode {
    fn drop(&mut self) {
        let _ = stdout().execute(LeaveAlternateScreen);
        let _ = disable_raw_mode();
    }
}