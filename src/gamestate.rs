extern crate sdl2;

use crate::gameclock::GameClock;
use std::rc::Rc;
use sdl2::Sdl;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;

pub trait GameStateTrait {
    fn update(&mut self) -> Signal;
    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String>;
    fn handle_event(&mut self, event: &Event) -> Signal;
    fn load(&mut self) -> Result<(), String>;
    fn is_loaded(&self) -> bool;
}

pub enum Signal {
    Quit,
    Continue
}

pub struct GameMachine<'a> {
    should_run: bool,
    states: Vec<Rc<dyn GameStateTrait>>,
    current_index: usize,
    sdl: &'a Sdl
}

impl<'a> GameMachine<'a> {
    pub fn new(sdl_ctx: &'a Sdl) -> GameMachine<'a> {
        GameMachine {
            should_run: true,
            current_index: 0,
            states: vec![],
            sdl: sdl_ctx
        }
    }

    pub fn add_state(&mut self, state: Rc<dyn GameStateTrait>) {
        self.states.push(state);
    }

    fn get_curr_mut_state(&mut self) -> &mut dyn GameStateTrait {
        let state_rc : &mut Rc<dyn GameStateTrait> = self.states.get_mut(self.current_index).unwrap();
        Rc::get_mut(state_rc).unwrap()
    }

    fn get_curr_state(&self) -> &dyn GameStateTrait {
        let state_rc : &Rc<dyn GameStateTrait> = self.states.get(self.current_index).unwrap();
        state_rc.as_ref()
    }

    pub fn run(&mut self, clock: &mut GameClock, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let mut pump = self.sdl.event_pump().unwrap();

        'running: loop {
            {
                let state = self.get_curr_mut_state();

                if !state.is_loaded() {
                    match state.load() {
                        Err(err) => return Err(err),
                        _ => {}
                    }
                }
            }

            loop {
                if !self.should_run {
                    break 'running;
                }

                let state = self.get_curr_mut_state();

                for event in pump.poll_iter() {
                    match state.handle_event(&event) {
                        Signal::Quit => break 'running,
                        Signal::Continue => {}
                    }
                }

                while clock.should_update() {
                    match state.update() {
                        Signal::Quit => break 'running,
                        Signal::Continue => {}
                    }

                    clock.lag_update();
                }

                state.render(canvas)?;

                clock.tick();
            }
        }

        Ok(())
    }
}