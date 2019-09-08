
extern crate sdl2;

mod boardstate;
mod winstate;
mod gamemachine;

use std::rc::Rc;
use sdl2::Sdl;
use gamemachine::machine;
use gamemachine::clock;


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

    let mut clock = clock::Clock::new(sdl_cxt.timer().unwrap(), 16.0);
    let mut machine = machine::Machine::new(&sdl_cxt);

    machine.add_state(Rc::new(boardstate::BoardState::new()));
    machine.add_state(Rc::new(winstate::WinState::new()));

    machine.run(&mut clock, &mut canvas)?;

    Ok(())
}
