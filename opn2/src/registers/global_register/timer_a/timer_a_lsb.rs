use crate::registers::{GlobalRegister, Register};

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct TimerALsb(u8);

impl TimerALsb {
    pub fn new(timer_a: u8) -> Self {
        TimerALsb(timer_a)
    }
}

impl From<u8> for TimerALsb {
    fn from(lsb: u8) -> Self {
        TimerALsb::new(lsb)
    }
}

impl From<TimerALsb> for u8 {
    fn from(val: TimerALsb) -> Self {
        val.0
    }
}

impl Register for TimerALsb {
    const BASE_ADDRESS: u8 = 0x25;
}

impl GlobalRegister for TimerALsb {
    fn get_data(&self) -> u8 {
        self.0
    }
}
