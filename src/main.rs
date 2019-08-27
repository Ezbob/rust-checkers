extern crate sdl2;

mod gamestate;
mod gameclock;
mod boardstate;

use gamestate::GameStateTrait;
use gamestate::Signal;
use std::rc::Rc;

use sdl2::Sdl;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

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

    machine.run(&mut clock, &mut canvas)?;

    Ok(())
}
