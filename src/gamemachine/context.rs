use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::gamemachine::clock::Clock;
use sdl2::EventPump;


pub trait Context {
    fn canvas(&mut self) -> &mut Canvas<Window>;
    fn clock(&mut self) -> &mut Clock;
    fn event_pump(&mut self) -> &mut EventPump;
}

pub struct DefaultContext {
    pub canvas: Canvas<Window>,
    pub clock: Clock,
    pub event_pump: EventPump
}

impl Context for DefaultContext {
    fn canvas(&mut self) -> &mut Canvas<Window> {
        &mut self.canvas
    }

    fn clock(&mut self) -> &mut Clock {
        &mut self.clock
    }

    fn event_pump(&mut self) -> &mut EventPump {
        &mut self.event_pump
    }
}

impl DefaultContext {
    pub fn new(sdl_cxt: &sdl2::Sdl) -> Result<DefaultContext, String> {
        let video_sys = sdl_cxt.video()?;

        let win = video_sys.window("Rust sdl2 checkers", 840, 860)
            .position_centered()
            .build().map_err(|e| e.to_string())?;

        let canvas = win.into_canvas()
            .accelerated()
            .present_vsync()
            .build().map_err(|e| e.to_string())?;

        let clock = sdl_cxt.timer().map(|timer| Clock::new(timer, 16.0))?;

        let event_pump = sdl_cxt.event_pump()?;

        Ok(DefaultContext {
            canvas,
            clock,
            event_pump
        })
    }
}
