use std::io::{self, stdout, Write};
use crate::color_scheme::ColorScheme;
use crossterm::cursor::{MoveTo, SetCursorStyle, MoveLeft, MoveRight, MoveToColumn};
use crossterm::{ExecutableCommand, QueueableCommand};
use crossterm::style::{Colors, Print, SetColors};
use crossterm::terminal::{Clear, ClearType};

pub struct LineBuffer {
    buffer: String,
    prompt: String,
    color_scheme: ColorScheme,
}

impl LineBuffer {
    pub fn new(prompt: String, color_scheme: ColorScheme) -> Result<Self, std::io::Error> {
        Ok(LineBuffer {
            buffer: String::new(),
            prompt,
            color_scheme
        })
    }

    pub fn draw(&self) -> io::Result<()> {
        stdout().queue(MoveTo(0, 0))?;
        self.set_colors()?;
        stdout().queue(Clear(ClearType::CurrentLine))?;
        stdout().queue(Print(&self.prompt))?;
        stdout().queue(Print(&self.buffer))?;
        Ok(())

    }

    pub fn set_colors(&self) -> io::Result<()> {
        stdout().queue(SetColors(Colors::new(self.color_scheme.line_buffer_fg, self.color_scheme.line_buffer_bg)))?;

        Ok(())
    }
}