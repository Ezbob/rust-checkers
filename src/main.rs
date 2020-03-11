extern crate sdl2;

mod gamestates;
mod gamemachine;

use std::rc::Rc;
use sdl2::Sdl;
use gamemachine::runtime::Runtime;
use gamemachine::context::DefaultContext;
use gamestates::WinState;
use gamestates::BoardState;


fn main() -> Result<(), String> {
    let sdl_cxt: Sdl = sdl2::init()?;

    let mut runtime = Runtime::new();

    runtime.add_state(Rc::new(BoardState::new()));
    runtime.add_state(Rc::new(WinState::new()));

    runtime.run(DefaultContext::new(&sdl_cxt)?)
}
