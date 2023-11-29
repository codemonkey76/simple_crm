use crate::color_scheme::ColorScheme;
use crate::customer::Customer;
use std::io::{self, Write, stdout};
use crossterm::terminal::{size, Clear, ClearType};

pub struct ScrollBuffer {
    buffer: Vec<Customer>,
    filter: String,
    filtered: Vec<usize>,
    color_scheme: ColorScheme,
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
            rows,
            cols,
        })
    }

    pub fn get_selected_customer(&self) -> Option<()> {
        todo!();
    }

    pub fn delete_customer(&mut self) -> std::io::Result<()> {

        Ok(())
    }

}