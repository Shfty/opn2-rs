use super::{GlobalRegister, Register};

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct TimerB(u8);

impl TimerB {
    pub fn new(timer_b: u8) -> Self {
        TimerB(timer_b)
    }
}

impl From<u8> for TimerB {
    fn from(timer_b: u8) -> Self {
        TimerB::new(timer_b)
    }
}

impl From<TimerB> for u8 {
    fn from(val: TimerB) -> Self {
        val.0
    }
}

impl Register for TimerB {
    const BASE_ADDRESS: u8 = 0x26;
}

impl GlobalRegister for TimerB {
    fn get_data(&self) -> u8 {
        self.0
    }
}
