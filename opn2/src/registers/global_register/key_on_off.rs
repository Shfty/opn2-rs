use crate::registers::Channel;

use super::{GlobalRegister, Register};

use bitflags::bitflags;

bitflags! {
    /// Bitmask representing an active set of operators
    pub struct Operators: u8 {
        const OPERATOR_1 = 0b00010000;
        const OPERATOR_2 = 0b00100000;
        const OPERATOR_3 = 0b01000000;
        const OPERATOR_4 = 0b10000000;

        const OPERATOR_NONE = 0b00000000;
        const OPERATOR_ALL = Self::OPERATOR_1.bits | Self::OPERATOR_2.bits | Self::OPERATOR_3.bits | Self::OPERATOR_4.bits;
    }
}

impl Default for Operators {
    fn default() -> Self {
        Operators::OPERATOR_NONE
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct KeyOnOff {
    channel: Channel,
    operators: Operators,
}

#[allow(dead_code)]
impl KeyOnOff {
    pub fn new(channel: Channel, operators: Operators) -> Self {
        KeyOnOff { channel, operators }
    }

    pub fn get_channel(&self) -> Channel {
        self.channel
    }

    pub fn get_operators(&self) -> Operators {
        self.operators
    }
}

impl Default for KeyOnOff {
    fn default() -> Self {
        KeyOnOff::new(Channel::Channel1, Operators::OPERATOR_NONE)
    }
}

impl From<u8> for KeyOnOff {
    fn from(key_on_off: u8) -> Self {
        let channel = key_on_off & 0b111;

        let mut operators: Operators = Operators::OPERATOR_NONE;
        if (key_on_off >> 4) & 0b1 > 0 {
            operators |= Operators::OPERATOR_1;
        }
        if (key_on_off >> 5) & 0b1 > 0 {
            operators |= Operators::OPERATOR_2;
        }
        if (key_on_off >> 6) & 0b1 > 0 {
            operators |= Operators::OPERATOR_3;
        }
        if (key_on_off >> 7) & 0b1 > 0 {
            operators |= Operators::OPERATOR_4;
        }

        KeyOnOff::new(channel.into(), operators)
    }
}

impl From<KeyOnOff> for u8 {
    fn from(val: KeyOnOff) -> Self {
        let operators = val.operators.bits();
        let channel: u8 = val.channel.into();
        operators | channel
    }
}

impl Register for KeyOnOff {
    const BASE_ADDRESS: u8 = 0x28;
}

impl GlobalRegister for KeyOnOff {
    fn get_data(&self) -> u8 {
        (*self).into()
    }
}
