extern crate sdl2;

use crate::gamemachine::runtime_signal::RuntimeSignal;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;
use crate::assets::GameAssets;


pub trait GameStateTrait {
    fn update(&mut self, event: &sdl2::EventSubsystem) -> RuntimeSignal;
    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String>;
    fn handle_event(&mut self, event: &Event) -> RuntimeSignal;
    fn setup(&mut self, ass: &GameAssets) -> Result<(), String>;
    fn is_set_up(&self) -> bool;
}
