extern crate sdl2;

use crate::game_machine::runtime_signal::RuntimeSignal;

use crate::asset_loader::Assets;
use sdl2::event::Event;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub trait GameStateTrait {
    fn update(&mut self, event: &sdl2::EventSubsystem) -> RuntimeSignal;
    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String>;
    fn handle_event(&mut self, event: &Event) -> RuntimeSignal;
    fn setup(&mut self, ass: &Assets) -> Result<(), String>;
    fn is_set_up(&self) -> bool;
}
