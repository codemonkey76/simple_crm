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
            mode: EditorMode::SplashScreen,
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
        Ok(())
    }

    pub fn run(&mut self) -> io::Result<()> {
        loop {
            if poll(std::time::Duration::from_millis(500))? {
                if let Event::Key(event) = read()? {
                    match event.code {
                        KeyCode::Char('q') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                            log::info!("Exiting editor loop, received CTRL+Q");
                            break;
                        }
                        KeyCode::Char(c) => {
                            log::info!("Received key: {:?}", c);
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    pub fn set_mode(&mut self, mode: EditorMode) -> io::Result<()> {
        log::info!("Setting mode to {:?}", mode);
        self.mode = mode;

        Ok(())
    }
}