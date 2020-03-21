use sdl2::render::{TextureCreator, Texture, TextureQuery};
use sdl2::video::WindowContext;
use sdl2::surface::Surface;

pub struct TextureWithInfo<'a> {
    texture: sdl2::render::Texture<'a>,
    queried: sdl2::render::TextureQuery,
}

impl<'a> TextureWithInfo<'a> {
    pub fn new_from(t: &'a TextureCreator<WindowContext>, surf: &Surface) -> TextureWithInfo<'a> {
        let b = t.create_texture_from_surface(surf).unwrap();
        let q = b.query();
        TextureWithInfo {
            texture: b,
            queried: q,
        }
    }

    pub fn get_texture_ref(&self) -> &Texture {
        &self.texture
    }

    pub fn get_texture_info_ref(&self) -> &TextureQuery {
        &self.queried
    }
}
