use sdl2::ttf::{Sdl2TtfContext, Font};
use std::path::Path;

pub struct FontWithInfo<'ttf> {
    font: sdl2::ttf::Font<'ttf, 'static>,
    size: u16,
}

impl<'ttf> FontWithInfo<'ttf> {
    pub fn load<P: AsRef<Path>>(
        ttf: &'ttf Sdl2TtfContext,
        path: P,
        size: u16,
    ) -> Result<FontWithInfo<'ttf>, String> {
        let font = ttf.load_font(path, size).map_err(|e| e.to_string())?;
        Ok(FontWithInfo { font, size })
    }

    pub fn font_ref(&self) -> &Font<'ttf, 'static> {
        &self.font
    }

    pub fn font_size(&self) -> u16 {
        self.size
    }
}