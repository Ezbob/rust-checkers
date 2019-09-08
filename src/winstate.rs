extern crate sdl2;

use crate::gamestate::GameStateTrait;
use crate::gamestate::Signal;

use sdl2::event::Event;
use sdl2::video::Window;
use sdl2::render::Canvas;

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

    fn load(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn is_loaded(&self) -> bool {
        true
    }
}
