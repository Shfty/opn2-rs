use super::{OperatorRegister, Register};

use bitflags::bitflags;

bitflags! {
    pub struct SoftwareSoundGeneratorMode: u8 {
        const DISABLED = 0;
        const HOLD = 1;
        const ALTERNATE = 2;
        const ATTACK = 4;
        const ENABLE = 8;
    }
}

impl From<u8> for SoftwareSoundGeneratorMode {
    fn from(ssg_eg: u8) -> Self {
        let mut out = SoftwareSoundGeneratorMode::DISABLED;
        if ssg_eg & SoftwareSoundGeneratorMode::HOLD.bits() > 0 {
            out |= SoftwareSoundGeneratorMode::HOLD;
        }
        if ssg_eg & SoftwareSoundGeneratorMode::ALTERNATE.bits() > 0 {
            out |= SoftwareSoundGeneratorMode::ALTERNATE;
        }
        if ssg_eg & SoftwareSoundGeneratorMode::ATTACK.bits() > 0 {
            out |= SoftwareSoundGeneratorMode::ATTACK;
        }
        if ssg_eg & SoftwareSoundGeneratorMode::ENABLE.bits() > 0 {
            out |= SoftwareSoundGeneratorMode::ENABLE;
        }
        out
    }
}

impl From<SoftwareSoundGeneratorMode> for u8 {
    fn from(val: SoftwareSoundGeneratorMode) -> Self {
        val.bits()
    }
}

impl Default for SoftwareSoundGeneratorMode {
    fn default() -> Self {
        SoftwareSoundGeneratorMode::DISABLED
    }
}

impl Register for SoftwareSoundGeneratorMode {
    const BASE_ADDRESS: u8 = 0x90;
}

impl OperatorRegister for SoftwareSoundGeneratorMode {
    fn get_data(&self) -> u8 {
        (*self).into()
    }
}
