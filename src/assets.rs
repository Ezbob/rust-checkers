use sdl2::ttf::{Font, Sdl2TtfContext};
use std::path::PathBuf;

pub struct GameAssets<'a> {
    pub font_vt323_big: Font<'a, 'static>,
}

impl<'a> GameAssets<'a> {
    pub fn new(ttf: &'a mut Sdl2TtfContext) -> GameAssets<'a> {
        let mut path = PathBuf::new();
        path.push("src");
        path.push("assets");
        path.push("B612_Mono");
        path.push("B612Mono-Regular.ttf");

        let font_vt323_big = ttf.load_font(path, 23).unwrap();
        GameAssets { font_vt323_big }
    }
}
