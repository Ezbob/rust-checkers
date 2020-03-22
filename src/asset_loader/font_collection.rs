use crate::asset_loader::font_with_info::FontWithInfo;
use sdl2::ttf::Sdl2TtfContext;
use std::collections::HashMap;
use std::path::PathBuf;

macro_rules! font_sizes_map {
    ($ttf: expr, $path: expr, $( $key: literal ),+) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, FontWithInfo::load($ttf, $path, $key)?); )+
         map
    }}
}

pub struct FontCollection<'ttf> {
    pub b612_regular: HashMap<u16, FontWithInfo<'ttf>>,
    pub vt323_regular: HashMap<u16, FontWithInfo<'ttf>>,
    pub share_tech_mono_regular: HashMap<u16, FontWithInfo<'ttf>>,
}

impl<'ttf> FontCollection<'ttf> {
    pub fn new(ttf: &'ttf Sdl2TtfContext) -> Result<FontCollection<'ttf>, String> {
        let stm_path = PathBuf::from("assets/Share_Tech_Mono/ShareTechMono-Regular.ttf");
        let b_path = PathBuf::from("assets/B612_Mono/B612Mono-Regular.ttf");
        let vt_path = PathBuf::from("assets/VT323/VT323-Regular.ttf");

        Ok(FontCollection {
            b612_regular: font_sizes_map!(ttf, &b_path, 18, 30, 42),
            vt323_regular: font_sizes_map!(ttf, &vt_path, 18, 24, 30, 52),
            share_tech_mono_regular: font_sizes_map!(ttf, &stm_path, 14, 30, 52),
        })
    }
}
