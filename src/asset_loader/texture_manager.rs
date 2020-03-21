use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use std::collections::HashMap;
use sdl2::surface::Surface;
use crate::asset_loader::texture_with_info::TextureWithInfo;

pub struct TextureManager<'a> {
    texture_creator: &'a TextureCreator<WindowContext>,
    cache: HashMap<usize, TextureWithInfo<'a>>,
}

impl<'a> TextureManager<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> TextureManager<'a> {
        TextureManager {
            texture_creator,
            cache: HashMap::new(),
        }
    }

    pub fn insert_surface_as_texture(&mut self, i: usize, surf: Surface) {
        self.cache
            .insert(i, TextureWithInfo::new_from(&self.texture_creator, &surf));
    }

    pub fn get_texture(&self, i: usize) -> Option<&TextureWithInfo<'a>> {
        self.cache.get(&i)
    }
}