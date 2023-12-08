use super::{OperatorRegister, Register};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct TotalLevel(u8);

impl Default for TotalLevel {
    fn default() -> Self {
        TotalLevel(127)
    }
}

impl TotalLevel {
    pub fn new(tl: u8) -> Self {
        TotalLevel(tl)
    }
}

impl From<u8> for TotalLevel {
    fn from(tl: u8) -> Self {
        TotalLevel::new(tl)
    }
}

impl From<TotalLevel> for u8 {
    fn from(val: TotalLevel) -> Self {
        val.0
    }
}

impl Register for TotalLevel {
    const BASE_ADDRESS: u8 = 0x40;
}

impl OperatorRegister for TotalLevel {
    fn get_data(&self) -> u8 {
        self.0
    }
}
