use super::{OperatorRegister, Register};

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct SecondDecayRate(u8);

impl SecondDecayRate {
    pub fn new(d2r: u8) -> Self {
        SecondDecayRate(d2r)
    }
}

impl From<u8> for SecondDecayRate {
    fn from(d2r: u8) -> Self {
        SecondDecayRate::new(d2r)
    }
}

impl From<SecondDecayRate> for u8 {
    fn from(val: SecondDecayRate) -> Self {
        val.0
    }
}

impl Register for SecondDecayRate {
    const BASE_ADDRESS: u8 = 0x70;
}

impl OperatorRegister for SecondDecayRate {
    fn get_data(&self) -> u8 {
        (*self).into()
    }
}
