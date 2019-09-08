
use crate::gamemachine::state::GameStateTrait;
use crate::gamemachine::resource::Context;

use std::rc::Rc;

pub enum Signal {
    Quit,
    GotoState(usize),
    Continue
}

pub struct Machine {
    should_run: bool,
    states: Vec<Rc<dyn GameStateTrait>>,
    current_index: usize
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
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
        let mut pump = context.event_pump();

        'running: while !self.states.is_empty() {

            let state = self.current_state_mut();

            if !state.is_loaded() {
                match state.load() {
                    Err(err) => return Err(err),
                    _ => {}
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

                {
                    let clock = context.clock_mut();
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
                }

                state.render(context.canvas_mut())?;

                context.clock_mut().tick();
            }
        }

        Ok(())
    }
}