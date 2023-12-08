use crate::registers::Register;

use super::ChannelRegister;

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Channel3SupplementaryFrequencyLsb(u8);

impl Channel3SupplementaryFrequencyLsb {
    pub fn new(freq: u8) -> Self {
        Channel3SupplementaryFrequencyLsb(freq)
    }
}

impl From<u8> for Channel3SupplementaryFrequencyLsb {
    fn from(lsb: u8) -> Self {
        Channel3SupplementaryFrequencyLsb::new(lsb)
    }
}

impl From<Channel3SupplementaryFrequencyLsb> for u8 {
    fn from(val: Channel3SupplementaryFrequencyLsb) -> Self {
        val.0
    }
}

impl Register for Channel3SupplementaryFrequencyLsb {
    const BASE_ADDRESS: u8 = 0xA8;
}

impl ChannelRegister for Channel3SupplementaryFrequencyLsb {
    fn get_data(&self) -> u8 {
        self.0
    }
}
