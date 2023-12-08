#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Timer {
    load: bool,
    enable: bool,
    reset: bool,
}

impl Timer {
    pub fn new(load: bool, enable: bool, reset: bool) -> Self {
        Timer {
            load,
            enable,
            reset,
        }
    }
}

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Timers {
    timer_a: Timer,
    timer_b: Timer,
}

impl Timers {
    pub fn new(
        load_a: bool,
        load_b: bool,
        enable_a: bool,
        enable_b: bool,
        reset_a: bool,
        reset_b: bool,
    ) -> Self {
        Timers {
            timer_a: Timer::new(load_a, enable_a, reset_a),
            timer_b: Timer::new(load_b, enable_b, reset_b),
        }
    }

    pub fn get(&self) -> (bool, bool, bool, bool, bool, bool) {
        (
            self.timer_a.load,
            self.timer_b.load,
            self.timer_a.enable,
            self.timer_b.enable,
            self.timer_a.reset,
            self.timer_b.reset,
        )
    }
}
