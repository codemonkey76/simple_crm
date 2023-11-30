use std::io::{self, stdout, Write};

use crossterm::QueueableCommand;
use crossterm::cursor::MoveTo;
use crossterm::style::{Colors, Print, SetColors};
use crossterm::terminal::{Clear, ClearType};

use crate::color_scheme::ColorScheme;

#[derive(PartialEq)]
pub enum TextMode {
    Insert,
    Overtype
}

pub struct LineBuffer {
    buffer: String,
    caret_pos: usize,
    prompt: String,
    text_mode: TextMode,
    color_scheme: ColorScheme,
}

impl LineBuffer {
    pub fn new(prompt: String, color_scheme: ColorScheme) -> Result<Self, std::io::Error> {
        Ok(LineBuffer {
            buffer: String::new(),
            caret_pos: 0,
            prompt,
            text_mode: TextMode::Insert,
            color_scheme,
        })
    }

    pub fn resize(&mut self) -> io::Result<()> {
        self.draw()?;
        Ok(())
    }

    pub fn get_string(&self) -> String {
        self.buffer.clone()
    }

    pub fn draw(&self) -> io::Result<()> {
        log::info!("Running LineBuffer Draw method");
        log::info!("Prompt: {}", &self.prompt);
        stdout().queue(MoveTo(0, 0))?;
        self.set_colors()?;
        stdout().queue(Clear(ClearType::CurrentLine))?;
        stdout().queue(Print(&self.prompt))?;
        stdout().queue(Print(&self.buffer))?;
        stdout().flush()?;
        Ok(())
    }

    pub fn add_key(&mut self, c: char) -> io::Result<()> {
        match self.text_mode {
            TextMode::Insert => self.insert_char(c),
            TextMode::Overtype => self.overtype_char(c)
        }

        self.draw()?;

        Ok(())
    }

    fn insert_char(&mut self, c: char) {
        log::info!("Inserting char: {:?}", c);
        self.buffer.insert(self.caret_pos, c);
        self.caret_pos += 1;
    }
    fn overtype_char(&mut self, c: char) {
        log::info!("Overtyping char: {:?}", c);
        if self.caret_pos < self.buffer.len() {
            self.buffer.replace_range(self.caret_pos..self.caret_pos + 1, &c.to_string());
        } else {
            self.buffer.push(c);
        }
        self.caret_pos += 1;
    }

    pub fn set_colors(&self) -> io::Result<()> {
        stdout().queue(SetColors(Colors::new(self.color_scheme.line_buffer_fg, self.color_scheme.line_buffer_bg)))?;

        Ok(())
    }
}