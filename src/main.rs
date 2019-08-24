extern crate sdl2;

mod gamestate;
mod gameclock;

use gamestate::GameStateTrait;
use gamestate::Signal;
use std::rc::Rc;

use sdl2::Sdl;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

struct InitialState {
    is_loaded: bool
}

impl InitialState {
    pub fn new() -> InitialState {
        InitialState {
            is_loaded: false
        }
    }
}

impl GameStateTrait for InitialState {

    fn update(&mut self) -> Signal {
        Signal::Continue
    }

    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.clear();
        canvas.set_draw_color(Color::RGB(0xff, 0xff, 0xff));
        canvas.draw_rect(canvas.viewport())?;

        canvas.present();
        Ok(())
    }

    fn handle_event(&mut self, event: &Event) -> gamestate::Signal {
        match event {
            Event::Quit {..}
            | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => Signal::Quit,
            _ => Signal::Continue
        }
    }

    fn load(&mut self) -> Result<(), String> {
        self.is_loaded = true;
        Ok(())
    }

    fn is_loaded(&self) -> bool {
        self.is_loaded
    }
}


fn main() -> Result<(), String> {
    let sdl_cxt: Sdl = sdl2::init()?;
    let video_sys = sdl_cxt.video()?;

    let win = video_sys.window("Rust sdl2 demo", 800, 600)
        .position_centered()
        .build().unwrap();

    let mut canvas = win.into_canvas().build().unwrap();
    let mut clock = gameclock::GameClock::new(sdl_cxt.timer().unwrap(), 16.0);
    let mut machine = gamestate::GameMachine::new(&sdl_cxt);

    machine.add_state(Rc::new(InitialState::new()));

    machine.run(&mut clock, &mut canvas)?;

    Ok(())
}
