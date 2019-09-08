extern crate sdl2;

use sdl2::TimerSubsystem;

pub struct Clock {
    milli_sec_per_update: f64,
    now: u64,
    last: u64,
    frame_elapsed: f64,
    update_lag: f64,
    timer: TimerSubsystem
}

impl Clock {
    pub fn new(timer: TimerSubsystem, ms: f64) -> Clock {
        Clock {
            milli_sec_per_update: ms,
            now: 0,
            last: 0,
            frame_elapsed: 0.0,
            update_lag: 0.0,
            timer
        }
    }

    pub fn tick(&mut self) {
        self.last = self.now;
        self.now = self.timer.performance_counter();
        self.frame_elapsed = ((self.now - self.last) * 1000 / self.timer.performance_frequency()) as f64;
        self.update_lag += self.frame_elapsed;
    }

    pub fn lag_update(&mut self) {
        self.update_lag -= self.milli_sec_per_update;
    }

    pub fn should_update(&self) -> bool {
        self.update_lag >= self.milli_sec_per_update
    }
}