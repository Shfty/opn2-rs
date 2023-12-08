use super::{GlobalRegister, Register};

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct DacEnable(bool);

impl DacEnable {
    pub fn new(enable: bool) -> Self {
        DacEnable(enable)
    }
}

impl From<bool> for DacEnable {
    fn from(enable: bool) -> Self {
        DacEnable::new(enable)
    }
}

impl From<u8> for DacEnable {
    fn from(enable: u8) -> Self {
        DacEnable::new(enable > 0)
    }
}

impl From<DacEnable> for bool {
    fn from(val: DacEnable) -> Self {
        val.0
    }
}

impl From<DacEnable> for u8 {
    fn from(val: DacEnable) -> Self {
        val.0 as u8
    }
}

impl Register for DacEnable {
    const BASE_ADDRESS: u8 = 0x2A;
}

impl GlobalRegister for DacEnable {
    fn get_data(&self) -> u8 {
        self.0 as u8
    }
}

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct DacAmplitude(u8);

impl DacAmplitude {
    pub fn new(amplitude: u8) -> Self {
        DacAmplitude(amplitude)
    }
}

impl From<u8> for DacAmplitude {
    fn from(amplitude: u8) -> Self {
        DacAmplitude::new(amplitude)
    }
}

impl From<DacAmplitude> for u8 {
    fn from(val: DacAmplitude) -> Self {
        val.0
    }
}

impl Register for DacAmplitude {
    const BASE_ADDRESS: u8 = 0x2B;
}

impl GlobalRegister for DacAmplitude {
    fn get_data(&self) -> u8 {
        self.0
    }
}
