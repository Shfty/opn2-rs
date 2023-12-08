use crate::registers::Register;

use super::ChannelRegister;

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct FrequencyLsb(u8);

impl FrequencyLsb {
    pub fn new(lsb: u8) -> Self {
        FrequencyLsb(lsb)
    }
}

impl From<u8> for FrequencyLsb {
    fn from(lsb: u8) -> Self {
        FrequencyLsb::new(lsb)
    }
}

impl From<FrequencyLsb> for u8 {
    fn from(val: FrequencyLsb) -> Self {
        val.0
    }
}

impl Register for FrequencyLsb {
    const BASE_ADDRESS: u8 = 0xA0;
}

impl ChannelRegister for FrequencyLsb {
    fn get_data(&self) -> u8 {
        self.0
    }
}
