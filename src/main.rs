extern crate sdl2;

mod gamestate;
mod gameclock;

use sdl2::Sdl;
use gamestate::GameStateTrait;
use std::rc::Rc;


fn main() -> Result<(), String> {
    let sdl_cxt: Sdl = sdl2::init()?;
    let video_sys = sdl_cxt.video()?;

    let _win = video_sys.window("Rust sdl2 demo", 800, 600)
        .position_centered()
        .build().unwrap();

    let states: Vec<Rc<dyn GameStateTrait>> = vec![];

    let mut clock = gameclock::GameClock::new(sdl_cxt.timer().unwrap(), 16.0);

    let mut machine = gamestate::GameMachine::new(&sdl_cxt, states);

    machine.run(&mut clock)?;

    Ok(())
}
