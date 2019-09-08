extern crate sdl2;

mod gamestates;
mod gamemachine;

use std::rc::Rc;
use sdl2::{Sdl, EventPump};
use gamemachine::runtime;
use gamemachine::clock;
use sdl2::render::Canvas;
use sdl2::video::Window;
use gamemachine::resource::DefaultContext;
use gamestates::WinState;
use gamestates::BoardState;


fn main() -> Result<(), String> {
    let sdl_cxt: Sdl = sdl2::init()?;

    let mut runtime = runtime::Runtime::new();

    runtime.add_state(Rc::new(BoardState::new()));
    runtime.add_state(Rc::new(WinState::new()));

    let mut context = DefaultContext::new(&sdl_cxt);

    runtime.run(&mut context)
}
