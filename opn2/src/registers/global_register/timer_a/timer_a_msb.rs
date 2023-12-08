use crate::registers::{GlobalRegister, Register};

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct TimerAMsb(u8);

impl TimerAMsb {
    pub fn new(timer_a: u8) -> Self {
        TimerAMsb(timer_a)
    }
}

impl From<u8> for TimerAMsb {
    fn from(msb: u8) -> Self {
        TimerAMsb::new(msb)
    }
}

impl From<TimerAMsb> for u8 {
    fn from(val: TimerAMsb) -> Self {
        val.0
    }
}

impl Register for TimerAMsb {
    const BASE_ADDRESS: u8 = 0x24;
}

impl GlobalRegister for TimerAMsb {
    fn get_data(&self) -> u8 {
        self.0
    }
}
