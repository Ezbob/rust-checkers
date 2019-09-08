
extern crate sdl2;

mod boardstate;
mod winstate;
mod gamemachine;

use std::rc::Rc;
use sdl2::{Sdl, EventPump};
use gamemachine::machine;
use gamemachine::clock;
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::gamemachine::clock::Clock;
use crate::gamemachine::resource;

struct MyContext<'a> {
    canvas: Canvas<Window>,
    clock: Clock,
    sdl_cxt: &'a sdl2::Sdl
}

impl<'a> resource::Context for MyContext<'a> {

    fn canvas_mut(&mut self) -> &mut Canvas<Window> {
        &mut self.canvas
    }

    fn clock_mut(&mut self) -> &mut Clock {
        &mut self.clock
    }

    fn event_pump(&mut self) -> EventPump {
        self.sdl_cxt.event_pump().unwrap()
    }
}


fn main() -> Result<(), String> {
    let sdl_cxt: Sdl = sdl2::init()?;
    let video_sys = sdl_cxt.video()?;


    let win = video_sys.window("Rust sdl2 checkers", 840, 860)
        .position_centered()
        .build().unwrap();

    let canvas = win
        .into_canvas()
        .present_vsync()
        .accelerated()
        .build().unwrap();

    let mut context = MyContext {
        sdl_cxt: &sdl_cxt,
        canvas,
        clock:  clock::Clock::new(sdl_cxt.timer().unwrap(), 16.0)
    };

    let mut machine = machine::Machine::new();

    machine.add_state(Rc::new(boardstate::BoardState::new()));
    machine.add_state(Rc::new(winstate::WinState::new()));

    machine.run(&mut context)?;

    Ok(())
}
