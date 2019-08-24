extern crate sdl2;

use std::rc::Rc;
use sdl2::Sdl;

pub trait GameStateTrait {
    fn update(&mut self);
    fn render(&self);
    fn handle_event(&mut self, event: &sdl2::event::Event);
    fn load(&mut self) -> Result<(), String>;
    fn is_loaded(&self) -> bool;
}

pub struct GameMachine<'a> {
    should_run: bool,
    states: Vec<Rc<dyn GameStateTrait>>,
    current_index: usize,
    sdl: &'a Sdl
}

impl<'a> GameMachine<'a> {
    pub fn new(sdl_ctx: & Sdl, states: Vec<Rc<dyn GameStateTrait>>) -> GameMachine {
        GameMachine {
            should_run: false,
            current_index: 0,
            states,
            sdl: sdl_ctx
        }
    }

    pub fn goto_state(&mut self, i: usize) {
        self.current_index = i;
        self.should_run = false;
    }

    fn get_curr_mut_state(&mut self) -> &mut dyn GameStateTrait {
        let state_rc : &mut Rc<dyn GameStateTrait> = self.states.get_mut(self.current_index).unwrap();
        Rc::get_mut(state_rc).unwrap()
    }

    fn get_curr_state(&self) -> &dyn GameStateTrait {
        let state_rc : &Rc<dyn GameStateTrait> = self.states.get(self.current_index).unwrap();
        state_rc.as_ref()
    }

    pub fn run(&mut self) -> Result<(), String> {
        let mut pump = self.sdl.event_pump().unwrap();
        'running: loop {
            if !self.should_run {
                break 'running;
            }

            {
                let state = self.get_curr_mut_state();

                if !state.is_loaded() {
                    match state.load() {
                        Err(err) => return Err(err),
                        _ => {}
                    }
                }
            }

            'stateloop: loop {
                let state = self.get_curr_mut_state();

                for event in pump.poll_iter() {
                    state.handle_event(&event);
                }
            }
        }

        Ok(())
    }
}