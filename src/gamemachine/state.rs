extern crate sdl2;

use crate::gamemachine::clock::Clock;
use crate::gamemachine::machine::Signal;
use std::rc::Rc;
use sdl2::Sdl;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;


pub trait GameStateTrait {
    fn update(&mut self) -> Signal;
    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String>;
    fn handle_event(&mut self, event: &Event) -> Signal;
    fn load(&mut self) -> Result<(), String>;
    fn is_loaded(&self) -> bool;
}

