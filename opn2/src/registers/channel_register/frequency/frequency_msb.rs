use crate::registers::{ChannelRegister, Register};

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct FrequencyMsb(u8);

impl FrequencyMsb {
    pub fn new(msb: u8) -> Self {
        FrequencyMsb(msb)
    }
}

impl From<u8> for FrequencyMsb {
    fn from(lsb: u8) -> Self {
        FrequencyMsb::new(lsb)
    }
}

impl From<FrequencyMsb> for u8 {
    fn from(val: FrequencyMsb) -> Self {
        val.0
    }
}

impl Register for FrequencyMsb {
    const BASE_ADDRESS: u8 = 0xA4;
}

impl ChannelRegister for FrequencyMsb {
    fn get_data(&self) -> u8 {
        self.0
    }
}
