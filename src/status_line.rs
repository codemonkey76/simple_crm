use std::io;
use std::io::{stdout, Write};
use crossterm::cursor::{MoveTo, MoveToColumn, RestorePosition, SavePosition};
use crossterm::QueueableCommand;
use crossterm::style::{Colors, Print, SetColors};
use crossterm::terminal::{Clear, ClearType, size};
use crate::color_scheme::ColorScheme;

pub struct StatusLine {
    message: String,
    rows: usize,
    cols: usize,
    color_scheme: ColorScheme,
    results: usize,
}

impl StatusLine {
    pub fn new(color_scheme: ColorScheme) -> Result<Self, std::io::Error> {
        let size = size()?;
        let (cols, rows) = (size.0 as usize, size.1 as usize - 1);

        Ok(StatusLine {
            message: String::new(),
            rows,
            cols,
            color_scheme,
            results: 0,
        })
    }

    pub fn set_results_count(&mut self, count: usize) -> io::Result<()> {
        self.results = count;
        self.draw()?;

        Ok(())
    }

    pub fn resize(&mut self) -> io::Result<()> {
       let size = size()?;
        let (cols, rows) = (size.0 as usize, size.1 as usize - 1);

        self.rows = rows;
        self.cols = cols;

        self.draw()?;

        Ok(())
    }

    pub fn draw(&self) -> io::Result<()> {
        let results_string = format!("Results: {}", self.results);
        let results_offset = results_string.len();
        stdout().queue(SetColors(Colors::new(self.color_scheme.status_line_fg, self.color_scheme.status_line_bg)))?;
        stdout().queue(SavePosition)?;
        stdout().queue(MoveTo(0, self.rows as u16))?;
        stdout().queue(Clear(ClearType::CurrentLine))?;
        stdout().queue(Print(&self.message))?;
        stdout().queue(MoveToColumn((self.cols - results_offset) as u16))?;
        stdout().queue(Print(results_string))?;
        stdout().queue(RestorePosition)?;
        stdout().flush()?;

        Ok(())
    }
}