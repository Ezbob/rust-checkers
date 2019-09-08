
extern crate sdl2;

mod boardstate;
mod winstate;
mod gamemachine;

use std::rc::Rc;
use sdl2::{Sdl, EventPump};
use gamemachine::runtime;
use gamemachine::clock;
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::gamemachine::resource::DefaultContext;


fn main() -> Result<(), String> {
    let sdl_cxt: Sdl = sdl2::init()?;

    let mut runtime = runtime::Runtime::new();

    runtime.add_state(Rc::new(boardstate::BoardState::new()));
    runtime.add_state(Rc::new(winstate::WinState::new()));

    let mut context = DefaultContext::new(&sdl_cxt);

    runtime.run(&mut context)
}
