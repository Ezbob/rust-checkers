
mod boardstate;
mod winstate;
mod pausestate;

pub use winstate::WinState;
pub use boardstate::BoardState;
pub use pausestate::PauseState;

macro_rules! initialize_states{
    ($runtime: expr) => {
        $runtime.add_state(Rc::new(BoardState::new()));
        $runtime.add_state(Rc::new(WinState::new()));
        $runtime.add_state(Rc::new(PauseState::new()));
    }
}
