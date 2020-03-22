use crate::game_machine::context::Context;
use crate::game_machine::runtime_signal::RuntimeSignal;
use crate::game_machine::state::GameStateTrait;

use crate::asset_loader::Assets;
use crate::game_machine::clock::Clock;
use sdl2::EventPump;
use sdl2::EventSubsystem;

pub struct Runtime<'state> {
    should_run: bool,
    states: Vec<&'state mut dyn GameStateTrait>,
    current_index: usize,
}

impl<'state> Runtime<'state> {
    pub fn new() -> Runtime<'state> {
        Runtime {
            should_run: true,
            current_index: 0,
            states: vec![],
        }
    }

    pub fn add_state(&mut self, state: &'state mut dyn GameStateTrait) {
        self.states.push(state);
    }

    fn current_state_mut(&mut self) -> &mut dyn GameStateTrait {
        let current_index = self.current_index;
        *self.states.get_mut(current_index).unwrap()
    }

    fn handle_update(&mut self, clock: &mut Clock, events: &EventSubsystem) -> RuntimeSignal {
        while clock.should_update() {
            match self.current_state_mut().update(events) {
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
                }
                RuntimeSignal::GotoState(state_index) => {
                    return RuntimeSignal::GotoState(state_index);
                }
                _ => {}
            }
        }
        RuntimeSignal::Continue
    }

    fn handle_setup(&mut self, ass: &Assets) -> Result<(), String> {
        let state = self.current_state_mut();

        if !state.is_set_up() {
            state.setup(ass)?;
        }

        Ok(())
    }

    pub fn run(
        &mut self,
        context: &mut dyn Context,
        ass: &Assets,
        event_sys: &EventSubsystem,
    ) -> Result<(), String> {
        'running: while !self.states.is_empty() {
            self.handle_setup(ass)?;

            'gameloop: loop {
                if !self.should_run {
                    break 'running;
                }

                match self.handle_events(context.event_pump()) {
                    RuntimeSignal::Quit => {
                        break 'running;
                    }
                    RuntimeSignal::GotoState(i) => {
                        self.current_index = i;
                        break 'gameloop;
                    }
                    _ => {}
                }

                match self.handle_update(context.clock(), event_sys) {
                    RuntimeSignal::Quit => {
                        break 'running;
                    }
                    RuntimeSignal::GotoState(i) => {
                        self.current_index = i;
                        break 'gameloop;
                    }
                    _ => {}
                }

                self.current_state_mut().render(context.canvas())?;

                context.clock().tick();
            }
        }

        Ok(())
    }
}
