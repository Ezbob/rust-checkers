use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::gamemachine::clock::Clock;
use sdl2::EventPump;

pub struct ExtensionLibraries {
    pub ttf_context: Option<sdl2::ttf::Sdl2TtfContext>
}

pub struct Context {
    pub canvas: Canvas<Window>,
    pub clock: Clock,
    pub event_pump: EventPump,
    pub extensions: ExtensionLibraries
}

impl Context {
    pub fn new(sdl_cxt: sdl2::Sdl) -> Result<Context, String> {
        let video_sys = match sdl_cxt.video() {
            Err(err) => return Err(err),
            Ok(video_system) => video_system
        };

        let win = match video_sys.window("Rust sdl2 checkers", 840, 860)
            .position_centered()
            .build() {
            Err(err) => return Err(err.to_string()),
            Ok(result) => result
        };

        let canvas = match win.into_canvas()
            .accelerated()
            .present_vsync()
            .build() {
            Err(err) => return Err(err.to_string()),
            Ok(result) => result
        };

        let clock = match sdl_cxt.timer() {
            Ok(timer) => Clock::new(timer, 16.0),
            Err(err) => return Err(err)
        };

        let event_pump = match sdl_cxt.event_pump() {
            Ok(pump) => pump,
            Err(err) => return Err(err)
        };

        Ok(Context {
            canvas,
            clock,
            event_pump,
            extensions: ExtensionLibraries {
                ttf_context: match sdl2::ttf::init() {
                    Ok(tff) => Some(tff),
                    Err(_) => None
                }
            }
        })
    }
}

/*
impl Context for DefaultContext {

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

    fn extensions(&mut self) -> &mut ExtensionLibraries {
        &mut self.extensions
    }
}
*/