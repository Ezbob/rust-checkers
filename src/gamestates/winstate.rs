extern crate sdl2;

use crate::gamemachine::state::GameStateTrait;
use crate::gamemachine::runtime_signal::RuntimeSignal;

use sdl2::event::Event;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::keyboard::Keycode;

use sdl2::pixels::Color;

pub struct WinState {}

impl WinState {
    pub fn new() -> WinState {
        WinState {}
    }
}

impl GameStateTrait for WinState {
    fn update(&mut self) -> RuntimeSignal {
        RuntimeSignal::Continue
    }

    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(0xff,0xff,0xff));
        canvas.clear();

        canvas.present();
        Ok(())
    }

    fn handle_event(&mut self, event: &Event) -> RuntimeSignal {
        match event {
            Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => RuntimeSignal::Quit,
            _ => RuntimeSignal::Continue
        }
    }

    fn setup(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn is_set_up(&self) -> bool {
        true
    }
}
