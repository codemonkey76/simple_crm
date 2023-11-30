use crossterm::style::Color;

#[derive(Debug, Clone)]
pub struct ColorScheme {
    pub line_buffer_fg: Color,
    pub line_buffer_bg: Color,
    pub status_line_fg: Color,
    pub status_line_bg: Color,
    pub scroll_buffer_fg: Color,
    pub scroll_buffer_bg: Color
}

impl Default for ColorScheme {
    fn default() -> Self {
        ColorScheme {
            line_buffer_fg: Color::Rgb { r: 118, g: 159, b: 240 },
            line_buffer_bg: Color::Rgb { r: 57, g: 66, b: 96 },
            status_line_fg: Color::Rgb { r: 49, g: 51, b: 70 },
            status_line_bg: Color::Rgb { r: 69, g: 71, b: 90 },
            scroll_buffer_fg: Color::Rgb { r: 163, g: 174, b: 210 },
            scroll_buffer_bg: Color::Rgb { r: 9, g: 12, b: 12 },
        }
    }
}