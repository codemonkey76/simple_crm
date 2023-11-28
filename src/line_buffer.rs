use crate::color_scheme::ColorScheme;

pub struct LineBuffer {
    prompt: String,
    color_scheme: ColorScheme,
}

impl LineBuffer {
    pub fn new(prompt: String, color_scheme: ColorScheme) -> Result<Self, std::io::Error> {
        Ok(LineBuffer {
            prompt,
            color_scheme
        })
    }
}