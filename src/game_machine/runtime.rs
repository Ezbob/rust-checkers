use crate::game_machine::context::Context;
use crate::game_machine::runtime_signal::RuntimeSignal;
use crate::game_machine::state::GameStateTrait;

use crate::asset_loader::Assets;
use crate::game_machine::clock::Clock;
use sdl2::EventPump;
use sdl2::EventSubsystem;

pub struct Runtime<'state> {
    states: Vec<&'state mut dyn GameStateTrait>,
    assets: &'state Assets<'state>,
    event_system: &'state EventSubsystem,
    current_index: usize,
}

impl<'state> Runtime<'state> {
    pub fn new(
        assets: &'state Assets<'state>,
        event_system: &'state EventSubsystem,
    ) -> Runtime<'state> {
        Runtime {
            current_index: 0,
            states: vec![],
            assets,
            event_system,
        }
    }

    pub fn add_state(&mut self, state: &'state mut dyn GameStateTrait) {
        self.states.push(state);
    }

    fn current_state_mut(&mut self) -> Result<&mut dyn GameStateTrait, String> {
        let current_index = self.current_index;

        return if let Some(state) = self.states.get_mut(current_index) {
            Ok(*state)
        } else {
            Err(String::from(format!(
                "No such state at index {}",
                current_index
            )))
        };
    }

    fn handle_update(&mut self, clock: &mut Clock) -> Result<RuntimeSignal, String> {
        let event_system = self.event_system;
        while clock.should_update() {
            match self.current_state_mut()?.update(event_system) {
                RuntimeSignal::GotoState(i) => return Ok(RuntimeSignal::GotoState(i)),
                RuntimeSignal::Quit => return Ok(RuntimeSignal::Quit),
                _ => {}
            }
            clock.lag_update();
        }
        Ok(RuntimeSignal::Continue)
    }

    fn handle_events(&mut self, event_pump: &mut EventPump) -> Result<RuntimeSignal, String> {
        for event in event_pump.poll_iter() {
            match self.current_state_mut()?.handle_event(&event) {
                RuntimeSignal::Quit => {
                    return Ok(RuntimeSignal::Quit);
                }
                RuntimeSignal::GotoState(state_index) => {
                    return Ok(RuntimeSignal::GotoState(state_index));
                }
                _ => {}
            }
        }
        Ok(RuntimeSignal::Continue)
    }

    fn handle_setup(&mut self) -> Result<(), String> {
        let ass = self.assets;
        let state = self.current_state_mut()?;

        if !state.is_set_up() {
            state.setup(ass)?;
        }

        Ok(())
    }

    pub fn run(&mut self, context: &mut dyn Context) -> Result<(), String> {
        'running: while !self.states.is_empty() {
            self.handle_setup()?;

            'gameloop: loop {
                match self.handle_events(context.event_pump())? {
                    RuntimeSignal::Quit => {
                        break 'running;
                    }
                    RuntimeSignal::GotoState(i) => {
                        self.current_index = i;
                        break 'gameloop;
                    }
                    _ => {}
                }

                match self.handle_update(context.clock())? {
                    RuntimeSignal::Quit => {
                        break 'running;
                    }
                    RuntimeSignal::GotoState(i) => {
                        self.current_index = i;
                        break 'gameloop;
                    }
                    _ => {}
                }

                self.current_state_mut()?.render(context.canvas())?;

                context.clock().tick();
            }
        }

        Ok(())
    }
}
