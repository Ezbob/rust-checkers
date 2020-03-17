pub struct WinColorEvent {
    pub color: i32,
}

impl WinColorEvent {
    pub fn is_green(&self) -> bool {
        return self.color == 1;
    }

    pub fn new_green() -> WinColorEvent {
        WinColorEvent { color: 1 }
    }

    pub fn new_red() -> WinColorEvent {
        WinColorEvent { color: 0 }
    }
}
