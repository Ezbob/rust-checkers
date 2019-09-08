use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::gamemachine::clock::Clock;
use sdl2::EventPump;

pub trait Context {
    fn canvas_mut(&mut self) -> &mut Canvas<Window>;
    fn clock_mut(&mut self) -> &mut Clock;
    fn event_pump(&mut self) -> EventPump;
}