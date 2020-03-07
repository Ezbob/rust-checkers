extern crate sdl2;

use crate::gamemachine::runtime_signal::RuntimeSignal;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;
use crate::gamemachine::resource::ExtensionLibraries;


pub trait GameStateTrait {
    fn update(&mut self) -> RuntimeSignal;
    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String>;
    fn handle_event(&mut self, event: &Event) -> RuntimeSignal;
    fn load(&mut self, libs: &mut ExtensionLibraries) -> Result<(), String>;
    fn is_loaded(&self) -> bool;
}

