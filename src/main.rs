extern crate sdl2;

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

    let assets= GameAssets::new(&mut ttf);
    let mut runtime = Runtime::new();
    let mut context = DefaultContext::new(&sdl_cxt)?;


    runtime.add_state(Rc::new(BoardState::new() ) );
    runtime.add_state( Rc::new(WinState::new() ) );
    runtime.add_state( Rc::new(PauseState::new() ) );

    runtime.run(&mut context, &assets, &sdl_event)
}
