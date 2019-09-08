extern crate sdl2;

mod gamestate;
mod gameclock;
mod boardstate;
mod winstate;

use std::rc::Rc;
use sdl2::Sdl;


fn main() -> Result<(), String> {
    let sdl_cxt: Sdl = sdl2::init()?;
    let video_sys = sdl_cxt.video()?;

    let win = video_sys.window("Rust sdl2 checkers", 840, 860)
        .position_centered()
        .build().unwrap();

    let mut canvas = win
        .into_canvas()
        .present_vsync()
        .accelerated()
        .build().unwrap();

    let mut clock = gameclock::GameClock::new(sdl_cxt.timer().unwrap(), 16.0);
    let mut machine = gamestate::GameMachine::new(&sdl_cxt);

    machine.add_state(Rc::new(boardstate::BoardState::new()));
    machine.add_state(Rc::new(winstate::WinState::new()));

    machine.run(&mut clock, &mut canvas)?;

    Ok(())
}
