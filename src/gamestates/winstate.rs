extern crate sdl2;

use crate::gamemachine::state::GameStateTrait;
use crate::gamemachine::runtime::Signal;

use sdl2::event::Event;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::ttf::{Font, Sdl2TtfContext};
use std::path::PathBuf;
use std::rc::Rc;
use crate::gamemachine::resource::ExtensionLibraries;
use sdl2::pixels::Color;

pub struct WinState;

impl WinState {
    pub fn new() -> WinState {
        WinState
    }
}

impl GameStateTrait for WinState {
    fn update(&mut self) -> Signal {
        Signal::Continue
    }

    fn render(&self, _canvas: &mut Canvas<Window>) -> Result<(), String> {


        Ok(())
    }

    fn handle_event(&mut self, _event: &Event) -> Signal {
        Signal::Continue
    }

    fn load(&mut self, libs: &mut ExtensionLibraries) -> Result<(), String> {

        let ttf = libs.ttf_context.as_ref().unwrap();

        let font = ttf.load_font("./src/assets/B612_Mono/B612Mono-Regular.ttf", 20).unwrap();

        let surface = font.render("Hello world")
            .solid(Color::RGB(0xff, 0xff, 0xff)).unwrap();

        /*
        let mut asset_path = PathBuf::new();
        asset_path.push("src");
        asset_path.push("assets");
        asset_path.push("B612_Mono");
        asset_path.push("B612Mono-Regular.ttf");
        if let Ok(font) = self.text_module.load_font(asset_path.as_path(), 18) {
            self.font = Some(Rc::new(font));
        }
        */
        Ok(())
    }

    fn is_loaded(&self) -> bool {
        true
    }
}
