mod channel_3_mode;
mod timers;

pub use channel_3_mode::*;
pub use timers::*;

use super::{GlobalRegister, Register};

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct TimersAndChannel3Mode {
    timers: Timers,
    channel_3_mode: Channel3Mode,
}

impl TimersAndChannel3Mode {
    pub fn new(timers: Timers, channel_3_mode: Channel3Mode) -> Self {
        TimersAndChannel3Mode {
            timers,
            channel_3_mode,
        }
    }
}

// Getters
#[allow(dead_code)]
impl TimersAndChannel3Mode {
    pub fn get_timers(&self) -> Timers {
        self.timers
    }

    pub fn get_channel_3_mode(&self) -> Channel3Mode {
        self.channel_3_mode
    }
}

// Setters
#[allow(dead_code)]
impl TimersAndChannel3Mode {
    pub fn set_timers<T>(&mut self, timers: T) where T: Into<Timers> {
        self.timers = timers.into();
    }

    pub fn set_channel_3_mode<T>(&mut self, channel_3_mode: T) where T: Into<Channel3Mode> {
        self.channel_3_mode = channel_3_mode.into();
    }
}

impl From<u8> for TimersAndChannel3Mode {
    fn from(tc: u8) -> Self {
        let load_a = tc & 0b1 > 0;
        let load_b = (tc >> 1) & 0b1 > 0;
        let enable_a = (tc >> 2) & 0b1 > 0;
        let enable_b = (tc >> 3) & 0b1 > 0;
        let reset_a = (tc >> 4) & 0b1 > 0;
        let reset_b = (tc >> 5) & 0b1 > 0;
        let channel_3_mode = tc >> 6;

        TimersAndChannel3Mode::new(
            Timers::new(load_a, load_b, enable_a, enable_b, reset_a, reset_b),
            channel_3_mode.into(),
        )
    }
}

impl From<TimersAndChannel3Mode> for u8 {
    fn from(val: TimersAndChannel3Mode) -> Self {
        let (load_a, load_b, enable_a, enable_b, reset_a, reset_b) = val.timers.get();
        let data = val.channel_3_mode as u8;
        let data = data << 1;
        let data = data | (reset_b as u8);
        let data = data << 1;
        let data = data | (reset_a as u8);
        let data = data << 1;
        let data = data | (enable_b as u8);
        let data = data << 1;
        let data = data | (enable_a as u8);
        let data = data << 1;
        let data = data | (load_b as u8);
        let data = data << 1;
        data | (load_a as u8)
    }
}

impl Register for TimersAndChannel3Mode {
    const BASE_ADDRESS: u8 = 0x27;
}

impl GlobalRegister for TimersAndChannel3Mode {
    fn get_data(&self) -> u8 {
        (*self).into()
    }
}
