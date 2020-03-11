
use crate::gamemachine::state::GameStateTrait;
use crate::gamemachine::context::Context;
use crate::gamemachine::runtime_signal::RuntimeSignal;

use std::rc::Rc;
use crate::gamemachine::clock::Clock;
use sdl2::EventPump;

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
        Rc::get_mut(&mut self.states[self.current_index]).unwrap()
    }

    fn current_state(&self) -> &dyn GameStateTrait {
        self.states[self.current_index].as_ref()
    }

    fn handle_update(&mut self, clock: &mut Clock) -> RuntimeSignal {
        while clock.should_update() {
            match self.current_state_mut().update() {
                RuntimeSignal::GotoState(i) => return RuntimeSignal::GotoState(i),
                RuntimeSignal::Quit => return RuntimeSignal::Quit,
                _ => {}
            }
            clock.lag_update();
        }
        RuntimeSignal::Continue
    }

    fn handle_events(&mut self, event_pump: &mut EventPump) -> RuntimeSignal {
        let state = self.current_state_mut();
        for event in event_pump.poll_iter() {
            match state.handle_event(&event) {
                RuntimeSignal::Quit => {
                    return RuntimeSignal::Quit;
                },
                RuntimeSignal::GotoState(state_index) => {
                    return RuntimeSignal::GotoState(state_index);
                },
                _ => {}
            }
        }
        RuntimeSignal::Continue
    }

    fn handle_setup(&mut self) -> Result<(), String> {
        let state = self.current_state_mut();

        if !state.is_set_up() {
            state.setup()?;
        }

        Ok(())
    }

    pub fn run<T>(&mut self, mut context: T) -> Result<(), String> where T: Context {

        'running: while !self.states.is_empty() {

            self.handle_setup()?;

            'gameloop: loop {
                if !self.should_run {
                    break 'running;
                }

                match self.handle_events(context.event_pump()) {
                    RuntimeSignal::Quit => {
                        break 'running;
                    },
                    RuntimeSignal::GotoState(i) =>  {
                        self.current_index = i;
                        break 'gameloop;
                    },
                    _ => {}
                }

                match self.handle_update(context.clock()) {
                    RuntimeSignal::Quit => {
                        break 'running;
                    },
                    RuntimeSignal::GotoState(i) =>  {
                        self.current_index = i;
                        break 'gameloop;
                    },
                    _ => {}
                }

                self.current_state().render(context.canvas())?;

                context.clock().tick();
            }
        }

        Ok(())
    }
}