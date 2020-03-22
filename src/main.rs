extern crate sdl2;

mod game_states;

mod asset_loader;
mod game_events;
mod game_machine;

use crate::asset_loader::Assets;
use crate::game_machine::context::Context;
use crate::game_states::PauseState;
use game_machine::context::DefaultContext;
use game_machine::runtime::Runtime;
use game_states::BoardState;
use game_states::WinState;

fn main() -> Result<(), String> {
    let sdl_cxt = sdl2::init()?;
    let sdl_event = sdl_cxt.event()?;
    let ttf = sdl2::ttf::init().map_err(|e| e.to_string())?;

    sdl_event.register_custom_event::<game_events::WinColorEvent>()?;

    let assets = Assets::new(&ttf)?;
    let mut runtime = Runtime::new();
    let mut context = DefaultContext::new(&sdl_cxt)?;

    let text_creator = context.canvas().texture_creator();

    let mut board_state = BoardState::new(&text_creator);
    let mut win_state = WinState::new(&text_creator);
    let mut pause_state = PauseState::new(&text_creator);

    runtime.add_state(&mut board_state);
    runtime.add_state(&mut win_state);
    runtime.add_state(&mut pause_state);

    runtime.run(&mut context, &assets, &sdl_event)
}
