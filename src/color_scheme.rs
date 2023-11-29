use crossterm::style::Color;

#[derive(Debug, Clone)]
pub struct ColorScheme {
    pub line_buffer_fg: Color,
    pub line_buffer_bg: Color,
    pub status_line_fg: Color,
    pub status_line_bg: Color
}

impl Default for ColorScheme {
    fn default() -> Self {
        ColorScheme {
            line_buffer_fg: Color::Rgb { r: 49, g: 51, b: 70 },
            line_buffer_bg: Color::Rgb { r: 69, g: 71, b: 90 },
            status_line_fg: Color::Rgb { r: 49, g: 51, b: 70 },
            status_line_bg: Color::Rgb { r: 69, g: 71, b: 90 },
        }
    }
}