pub enum RuntimeSignal {
    Quit,
    GotoState(usize),
    Continue,
}
