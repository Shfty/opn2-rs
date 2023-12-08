use crate::registers::Register;

use super::ChannelRegister;

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Channel3SupplementaryFrequencyMsb(u8);

impl Channel3SupplementaryFrequencyMsb {
    pub fn new(msb: u8) -> Self {
        Channel3SupplementaryFrequencyMsb(msb)
    }
}

impl From<u8> for Channel3SupplementaryFrequencyMsb {
    fn from(msb: u8) -> Self {
        Channel3SupplementaryFrequencyMsb::new(msb)
    }
}

impl From<Channel3SupplementaryFrequencyMsb> for u8 {
    fn from(val: Channel3SupplementaryFrequencyMsb) -> Self {
        val.0
    }
}

impl Register for Channel3SupplementaryFrequencyMsb {
    const BASE_ADDRESS: u8 = 0xAC;
}

impl ChannelRegister for Channel3SupplementaryFrequencyMsb {
    fn get_data(&self) -> u8 {
        self.0
    }
}
