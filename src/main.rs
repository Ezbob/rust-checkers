extern crate sdl2;

mod game_states;

mod assets;
mod game_events;
mod game_machine;


use crate::assets::GameAssets;
use crate::game_states::PauseState;
use game_machine::context::DefaultContext;
use game_machine::runtime::Runtime;
use game_states::BoardState;
use game_states::WinState;
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
