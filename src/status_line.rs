use crate::color_scheme::ColorScheme;

pub struct StatusLine {
    color_scheme: ColorScheme
}

impl StatusLine {
    pub fn new(color_scheme: ColorScheme) -> Result<Self, std::io::Error> {
        Ok(StatusLine {
            color_scheme
        })
    }
}