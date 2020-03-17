mod boardstate;
mod pausestate;
mod winstate;

pub use boardstate::BoardState;
pub use pausestate::PauseState;
pub use winstate::WinState;

macro_rules! initialize_states {
    ($runtime: expr) => {
        $runtime.add_state(Rc::new(BoardState::new()));
        $runtime.add_state(Rc::new(WinState::new()));
        $runtime.add_state(Rc::new(PauseState::new()));
    };
}
