extern crate sdl2;

#[macro_use]
mod gamestates;

mod assets;
mod game_events;
mod gamemachine;

use crate::assets::GameAssets;
use crate::gamestates::PauseState;
use gamemachine::context::DefaultContext;
use gamemachine::runtime::Runtime;
use gamestates::BoardState;
use gamestates::WinState;
use std::rc::Rc;

fn main() -> Result<(), String> {
    let sdl_cxt = sdl2::init()?;
    let sdl_event = sdl_cxt.event()?;
    let mut ttf = sdl2::ttf::init().map_err(|e| e.to_string())?;

    sdl_event.register_custom_event::<game_events::WinColorEvent>()?;

    let mut runtime = Runtime::new();
    let context = DefaultContext::new(&sdl_cxt)?;

    let assets = GameAssets::new(&mut ttf);

    initialize_states!(runtime);

    runtime.run(context, &assets, &sdl_event)
}
