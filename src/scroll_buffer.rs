use crate::color_scheme::ColorScheme;

pub struct ScrollBuffer {
    color_scheme: ColorScheme,
}

impl ScrollBuffer {
    pub fn new(color_scheme: ColorScheme) -> Result<Self, std::io::Error> {
        Ok(ScrollBuffer {
            color_scheme
        })
    }
}