extern crate sdl2;

use crate::gamemachine::state::GameStateTrait;
use crate::gamemachine::machine::Signal;

use sdl2::event::Event;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::ttf::Font;

pub struct WinState<'a> {
    font: Option<Font<'a, 'static>>
}

impl<'a> WinState<'a> {
    pub fn new() -> WinState<'a> {
        WinState {
            font: None
        }
    }
}

impl<'a> GameStateTrait for WinState<'a> {
    fn update(&mut self) -> Signal {
        Signal::Continue
    }

    fn render(&self, _canvas: &mut Canvas<Window>) -> Result<(), String> {
        Ok(())
    }

    fn handle_event(&mut self, _event: &Event) -> Signal {
        Signal::Continue
    }

    fn load(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn is_loaded(&self) -> bool {
        true
    }
}
