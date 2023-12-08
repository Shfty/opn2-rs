use super::{GlobalRegister, Register};

#[repr(u8)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[allow(dead_code)]
pub enum LfoFrequency {
    Hz3_98 = 0,
    Hz5_56 = 1,
    Hz6_02 = 2,
    Hz6_37 = 3,
    Hz6_88 = 4,
    Hz9_67 = 5,
    Hz48_1 = 6,
    Hz72_2 = 7,
}

impl From<u8> for LfoFrequency {
    fn from(freq: u8) -> Self {
        match freq {
            0 => LfoFrequency::Hz3_98,
            1 => LfoFrequency::Hz5_56,
            2 => LfoFrequency::Hz6_02,
            3 => LfoFrequency::Hz6_37,
            4 => LfoFrequency::Hz6_88,
            5 => LfoFrequency::Hz9_67,
            6 => LfoFrequency::Hz48_1,
            7 => LfoFrequency::Hz72_2,
            _ => panic!("Invalid LFO frequency"),
        }
    }
}

impl From<LfoFrequency> for u8 {
    fn from(val: LfoFrequency) -> Self {
        match val {
            LfoFrequency::Hz3_98 => 0,
            LfoFrequency::Hz5_56 => 1,
            LfoFrequency::Hz6_02 => 2,
            LfoFrequency::Hz6_37 => 3,
            LfoFrequency::Hz6_88 => 4,
            LfoFrequency::Hz9_67 => 5,
            LfoFrequency::Hz48_1 => 6,
            LfoFrequency::Hz72_2 => 7,
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[allow(dead_code)]
pub enum Lfo {
    Disabled,
    Enabled(LfoFrequency),
}

impl From<u8> for Lfo {
    fn from(lfo: u8) -> Self {
        if lfo >> 3 > 0 {
            let frequency = lfo & 0b111;
            Lfo::Enabled(frequency.into())
        } else {
            Lfo::Disabled
        }
    }
}

impl From<Lfo> for u8 {
    fn from(val: Lfo) -> Self {
        match val {
            Lfo::Disabled => 0,
            Lfo::Enabled(frequency) => {
                let frequency: u8 = frequency.into();
                (1 << 3) as u8 | frequency
            },
        }
    }
}

impl Default for Lfo {
    fn default() -> Self {
        Lfo::Disabled
    }
}

impl Register for Lfo {
    const BASE_ADDRESS: u8 = 0x22;
}

impl GlobalRegister for Lfo {
    fn get_data(&self) -> u8 {
        match self {
            Lfo::Disabled => 0,
            Lfo::Enabled(frequency) => 0b1000 | *frequency as u8,
        }
    }
}
