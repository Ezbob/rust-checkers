use crate::asset_loader::font_collection::FontCollection;
use sdl2::ttf::Sdl2TtfContext;

pub struct Assets<'ttf> {
    pub font_collection: FontCollection<'ttf>,
}

impl<'ttf> Assets<'ttf> {
    pub fn new(ttf: &'ttf Sdl2TtfContext) -> Result<Assets<'ttf>, String> {
        Ok(Assets {
            font_collection: FontCollection::new(ttf)?,
        })
    }
}
