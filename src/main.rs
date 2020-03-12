extern crate sdl2;

mod gamestates;
mod gamemachine;
mod assets;

use std::rc::Rc;
use gamemachine::runtime::Runtime;
use gamemachine::context::DefaultContext;
use gamestates::WinState;
use gamestates::BoardState;
use crate::assets::GameAssets;


fn main() -> Result<(), String> {
    let sdl_cxt = sdl2::init()?;
    let mut ttf = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let mut runtime = Runtime::new();
    let context = DefaultContext::new(&sdl_cxt)?;

    let assets = GameAssets::new(&mut ttf);

    runtime.add_state(Rc::new(BoardState::new()));
    runtime.add_state(Rc::new(WinState::new()));

    runtime.run(context, &assets)
}
