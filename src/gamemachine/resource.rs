use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::gamemachine::clock::Clock;
use sdl2::EventPump;

pub trait Context {
    fn canvas(&mut self) -> Result<Canvas<Window>, String>;
    fn clock(&mut self) -> Result<Clock, String>;
    fn event_pump(&mut self) -> Result<EventPump, String>;
}

pub struct DefaultContext<'a> {
    sdl_cxt: &'a sdl2::Sdl
}

impl<'a> DefaultContext<'a> {
    pub fn new(sdl_cxt: &'a sdl2::Sdl) -> DefaultContext<'a> {
        DefaultContext {
            sdl_cxt: &sdl_cxt
        }
    }
}

impl<'a> Context for DefaultContext<'a> {

    fn canvas(&mut self) -> Result<Canvas<Window>, String> {
        let video_sys = match self.sdl_cxt.video() {
            Err(err) => return Err(err),
            Ok(video_system) => video_system
        };

        let win = match video_sys.window("Rust sdl2 checkers", 840, 860)
            .position_centered()
            .build() {
            Err(err) => return Err(err.to_string()),
            Ok(result) => result
        };

        match win.into_canvas()
            .accelerated()
            .present_vsync()
            .build() {
            Err(err) => Err(err.to_string()),
            Ok(result) => Ok(result)
        }
    }

    fn clock(&mut self) -> Result<Clock, String> {
        match self.sdl_cxt.timer() {
            Ok(timer) => Ok(Clock::new(timer, 16.0)),
            Err(err) => Err(err)
        }
    }

    fn event_pump(&mut self) -> Result<EventPump, String> {
        self.sdl_cxt.event_pump()
    }
}
