use sdl2::render::{Texture, TextureCreator, TextureQuery};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;

pub struct TextureWithInfo<'a> {
    texture: Texture<'a>,
    queried: TextureQuery,
}

impl<'a> TextureWithInfo<'a> {
    pub fn new_from(
        t: &'a TextureCreator<WindowContext>,
        surf: &Surface,
    ) -> Result<TextureWithInfo<'a>, String> {
        let b = t
            .create_texture_from_surface(surf)
            .map_err(|e| e.to_string())?;
        let q = b.query();
        Ok(TextureWithInfo {
            texture: b,
            queried: q,
        })
    }

    pub fn get_texture_ref(&self) -> &Texture {
        &self.texture
    }

    pub fn get_texture_info_ref(&self) -> &TextureQuery {
        &self.queried
    }
}
