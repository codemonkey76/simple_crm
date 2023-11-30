use crate::color_scheme::ColorScheme;
use crate::customer::Customer;
use std::io::{self, Write, stdout};
use crossterm::cursor::{MoveTo, MoveToNextLine, RestorePosition, SavePosition};
use crossterm::QueueableCommand;
use crossterm::style::{Colors, Print, SetColors};
use crossterm::terminal::{size, Clear, ClearType};

pub struct ScrollBuffer {
    buffer: Vec<Customer>,
    filter: String,
    filtered: Vec<usize>,
    color_scheme: ColorScheme,
    scroll_pos: usize,
    rows: usize,
    cols: usize
}

impl ScrollBuffer {
    pub fn new(color_scheme: ColorScheme) -> Result<Self, std::io::Error> {
        let size = size()?;
        let (cols, rows) = (size.0 as usize, size.1 as usize - 2);

        Ok(ScrollBuffer {
            buffer: Vec::new(),
            filter: String::new(),
            filtered: Vec::new(),
            color_scheme,
            scroll_pos: 0,
            rows,
            cols,
        })
    }

    pub fn resize(&mut self) -> std::io::Result<()> {
        let size = size()?;
        let (cols, rows) = (size.0 as usize, size.1 as usize - 2);
       self.cols = cols;
        self.rows = rows;
        self.draw()?;

        Ok(())
    }

    pub fn get_selected_customer(&self) -> Option<()> {
        todo!();
    }

    pub fn delete_customer(&mut self) -> std::io::Result<()> {

        Ok(())
    }

    pub fn set_filter(&mut self, filter: String) -> io::Result<()> {
        self.filter = filter;

        let filter_clone = self.filter.clone().to_lowercase();
        self.filtered = self.buffer.iter().enumerate().filter(|(_, c)| {
            c.name.to_lowercase().contains(&filter_clone) ||
                c.contact_name.as_ref().unwrap_or(&"".to_string()).to_lowercase().contains(&filter_clone) ||
                c.phone.as_ref().unwrap_or(&"".to_string()).to_lowercase().contains(&filter_clone)
        }).map(|(i, _)| i).collect();

        self.scroll_pos = 0;
        self.draw()?;

        Ok(())
    }

    pub fn draw(&self) -> io::Result<()> {
        stdout().queue(SavePosition)?;
        self.clear()?;
        stdout().queue(MoveTo(0, 1))?;
        stdout().queue(Print("Customers go here!"))?;
        stdout().queue(RestorePosition)?;
        stdout().flush()?;

        Ok(())
    }

    pub fn get_results_count(&self) -> usize {
        self.filtered.len()
    }

    pub fn set_colors(&self) -> io::Result<()> {
        stdout().queue(SetColors(Colors::new(self.color_scheme.scroll_buffer_fg, self.color_scheme.scroll_buffer_bg)))?;

        Ok(())
    }

    pub fn clear(&self) -> io::Result<()> {
        stdout().queue(MoveTo(0, 1))?;
        self.set_colors()?;

        for _ in 0..self.rows {
            stdout().queue(Clear(ClearType::CurrentLine))?;
            stdout().queue(MoveToNextLine(1))?;
        }

        stdout().flush()?;

        Ok(())
    }
}