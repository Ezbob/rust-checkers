
use crate::gamemachine::state::GameStateTrait;
use crate::gamemachine::resource::Context;
use crate::gamemachine::runtime_signal::RuntimeSignal;

use std::rc::Rc;

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

    pub fn run(&mut self, context: Context) -> Result<(), String> {
        let Context { mut canvas, mut clock, mut event_pump, mut extensions } = context;

        'running: while !self.states.is_empty() {
            let state = self.current_state_mut();

            if !state.is_loaded() {
                if let Err(err) = state.load(&mut extensions) {
                    return Err(err)
                }
            }

            'gameloop: loop {
                if !self.should_run {
                    break 'running;
                }

                let state = self.current_state_mut();

                for event in event_pump.poll_iter() {
                    match state.handle_event(&event) {
                        RuntimeSignal::Quit => break 'running,
                        RuntimeSignal::GotoState(state_index) => {
                            self.current_index = state_index;
                            break 'gameloop;
                        },
                        _ => {}
                    }
                }

                while clock.should_update() {
                    match state.update() {
                        RuntimeSignal::Quit => break 'running,
                        RuntimeSignal::GotoState(state_index) => {
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