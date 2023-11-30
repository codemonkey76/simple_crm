use std::io;
use std::path::PathBuf;

use crossterm::event::{Event, KeyCode, KeyModifiers, poll, read};

use crate::color_scheme::ColorScheme;
use crate::line_buffer::LineBuffer;
use crate::scroll_buffer::ScrollBuffer;
use crate::status_line::StatusLine;
use crate::utils::RawMode;

#[derive(Debug, PartialEq)]
pub enum EditorMode {
    Normal,
    SplashScreen,
    DeleteConfirmation,
}

pub struct Editor {
    pub file_path: PathBuf,
    pub config_path: PathBuf,
    line_buffer: LineBuffer,
    scroll_buffer: ScrollBuffer,
    status_line: StatusLine,
    color_scheme: ColorScheme,
    no_splash: bool,
    sample_data: bool,
    _raw_mode: RawMode,
    mode: EditorMode,
}

impl Editor {
    pub fn new(file_path: PathBuf, config_path: PathBuf, no_splash: bool, sample_data: bool) -> Result<Editor, std::io::Error> {
        let _raw_mode = RawMode::new()?;
        let color_scheme = ColorScheme::default();
        let line_buffer = LineBuffer::new("Query: ".to_string(), color_scheme.clone())?;
        let scroll_buffer = ScrollBuffer::new(color_scheme.clone())?;
        let status_line = StatusLine::new(color_scheme.clone())?;


        Ok(Editor {
            file_path,
            config_path,
            line_buffer,
            scroll_buffer,
            status_line,
            color_scheme,
            no_splash,
            sample_data,
            _raw_mode,
            mode: EditorMode::Normal,
        })
    }

    pub fn init(&mut self) -> io::Result<()> {
        log::info!("Loading Config...");

        log::info!("Finished loading config...");

        if self.sample_data {
            log::info!("Loading sample data...");
        } else {
            log::info!("Loading customers...");
        }

        if self.no_splash {
            self.set_mode(EditorMode::Normal)?;
        }

        self.draw()?;

        Ok(())
    }

    pub fn draw(&self) -> io::Result<()> {
        self.line_buffer.draw()?;
        self.scroll_buffer.draw()?;
        self.status_line.draw()?;
        Ok(())
    }

    pub fn resize(&mut self) -> io::Result<()> {
        self.status_line.resize()?;
        self.scroll_buffer.resize()?;
        self.line_buffer.resize()?;

        Ok(())
    }

    pub fn run(&mut self) -> io::Result<()> {
        loop {
            if poll(std::time::Duration::from_millis(500))? {
                match read()? {
                    Event::Key(event) => {
                        match event.code {
                            KeyCode::Char('q') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                                log::info!("Exiting editor loop, received CTRL+Q");
                                break;
                            }
                            KeyCode::Char(c) => {
                                self.add_key(c)?;
                                log::info!("Received key: {:?}", c);
                            }
                            _ => {}
                        }
                    }
                    Event::Resize(width, height) => {
                        self.resize()?;
                        log::info!("Window resized to {}x{}", width, height);
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    pub fn add_key(&mut self, c: char) -> io::Result<()> {
        log::info!("Key Pressed: {}", c);

        if self.mode == EditorMode::SplashScreen {
            return Ok(())
        }

        if self.mode == EditorMode::DeleteConfirmation {
            if c == 'y' {
                log::info!("Deleting customer");
                if self.scroll_buffer.get_selected_customer().is_some() {
                    log::info!("Found a valid selected customer");
                    self.scroll_buffer.delete_customer()?;
                    self.set_mode(EditorMode::Normal)?;
                    self.filter()?;
                }
            } else if c == 'n' {
                log::info!("Not deleting customer");
                self.set_mode(EditorMode::Normal)?;
                self.filter()?;
            }
            return Ok(())
        }

        self.line_buffer.add_key(c)?;

        if self.mode == EditorMode::Normal {
            self.filter()?;
        }
        Ok(())
    }

    pub fn filter(&mut self) -> io::Result<()> {
        self.scroll_buffer.set_filter(self.line_buffer.get_string())?;
        self.status_line.set_results_count(self.scroll_buffer.get_results_count())?;

        Ok(())
    }

    pub fn set_mode(&mut self, mode: EditorMode) -> io::Result<()> {
        log::info!("Setting mode to {:?}", mode);
        self.mode = mode;

        Ok(())
    }
}