extern crate sdl2;

mod gamestates;
mod gamemachine;
mod assets;
mod game_events;

use std::rc::Rc;
use gamemachine::runtime::Runtime;
use gamemachine::context::DefaultContext;
use gamestates::WinState;
use gamestates::BoardState;
use crate::assets::GameAssets;


fn main() -> Result<(), String> {
    let sdl_cxt = sdl2::init()?;
    let sdl_event = sdl_cxt.event()?;
    let mut ttf = sdl2::ttf::init().map_err(|e| e.to_string())?;

    sdl_event.register_custom_event::<game_events::WinColorEvent>()?;

    let mut runtime = Runtime::new();
    let context = DefaultContext::new(&sdl_cxt)?;

    let assets = GameAssets::new(&mut ttf);

    runtime.add_state(Rc::new(BoardState::new()));
    runtime.add_state(Rc::new(WinState::new()));

    runtime.run(context, &assets, &sdl_event)
}
