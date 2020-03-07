extern crate sdl2;

mod gamestates;
mod gamemachine;

use std::rc::Rc;
use sdl2::Sdl;
use gamemachine::runtime::Runtime;
use gamemachine::resource::Context;
use gamestates::WinState;
use gamestates::BoardState;


fn main() -> Result<(), String> {
    let sdl_cxt: Sdl = sdl2::init()?;

    let mut runtime = Runtime::new();

    runtime.add_state(Rc::new(BoardState::new()));
    runtime.add_state(Rc::new(WinState::new()));

    runtime.run(match Context::new(sdl_cxt) {
        Ok(ctx) => ctx,
        Err(e) => return Err(e)
    })
}
