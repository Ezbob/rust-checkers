
use crate::gamemachine::state::GameStateTrait;
use crate::gamemachine::resource::Context;

use std::rc::Rc;

pub enum Signal {
    Quit,
    GotoState(usize),
    Continue
}

pub struct Runtime {
    should_run: bool,
    states: Vec<Rc<dyn GameStateTrait>>,
    current_index: usize
}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {
            should_run: true,
            current_index: 0,
            states: vec![]
        }
    }

    pub fn add_state(&mut self, state: Rc<dyn GameStateTrait>) {
        self.states.push(state);
    }

    fn current_state_mut(&mut self) -> &mut dyn GameStateTrait {
        let state_rc= &mut self.states[self.current_index];
        Rc::get_mut(state_rc).unwrap()
    }

    pub fn run<ContextType>(&mut self, context: &mut ContextType) -> Result<(), String> where ContextType: Context {
        let mut pump = match context.event_pump() {
            Ok(pump) => pump,
            Err(error) => return Err(format!("Error when initializing pump: {}", error))
        };
        let mut clock = match context.clock() {
            Ok(clock) => clock,
            Err(error) => return Err(format!("Error when initializing clock: {}", error))
        };
        let mut canvas = match context.canvas() {
            Ok(canvas) => canvas,
            Err(error) => return Err(format!("Error when initializing canvas: {}", error))
        };

        'running: while !self.states.is_empty() {
            let state = self.current_state_mut();

            if !state.is_loaded() {
                if let Err(err) = state.load(context.extensions()) {
                    return Err(err)
                }
            }

            'gameloop: loop {
                if !self.should_run {
                    break 'running;
                }

                let state = self.current_state_mut();

                for event in pump.poll_iter() {
                    match state.handle_event(&event) {
                        Signal::Quit => break 'running,
                        Signal::GotoState(state_index) => {
                            self.current_index = state_index;
                            break 'gameloop;
                        },
                        _ => {}
                    }
                }

                while clock.should_update() {
                    match state.update() {
                        Signal::Quit => break 'running,
                        Signal::GotoState(state_index) => {
                            self.current_index = state_index;
                            break 'gameloop;
                        }
                        _ => {}
                    }

                    clock.lag_update();
                }

                state.render(&mut canvas)?;

                clock.tick();
            }
        }

        Ok(())
    }
}