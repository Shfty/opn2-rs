mod frequency_lsb;
mod frequency_msb;

pub use frequency_lsb::*;
pub use frequency_msb::*;

use super::ChannelRegister;

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Frequency {
    msb: FrequencyMsb,
    lsb: FrequencyLsb,
}

impl Frequency {
    pub fn new(msb: FrequencyMsb, lsb: FrequencyLsb) -> Self {
        Frequency { msb, lsb }
    }
}

impl From<u16> for Frequency {
    fn from(freq: u16) -> Self {
        let msb = (freq >> 8) as u8;
        let lsb = (freq & 0b11111111) as u8;
        Frequency::new(msb.into(), lsb.into())
    }
}

impl From<Frequency> for u16 {
    fn from(val: Frequency) -> Self {
        let msb: u8 = val.msb.into();
        let lsb: u8 = val.lsb.into();
        ((msb as u16) << 8) | lsb as u16
    }
}

// Getters
impl Frequency {
    pub fn get_msb(&self) -> FrequencyMsb {
        self.msb
    }

    pub fn get_lsb(&self) -> FrequencyLsb {
        self.lsb
    }
}

// Setters
impl Frequency {
    pub fn set_msb<T>(&mut self, msb: T)
    where
        T: Into<FrequencyMsb>,
    {
        self.msb = msb.into();
    }

    pub fn set_lsb<T>(&mut self, lsb: T)
    where
        T: Into<FrequencyLsb>,
    {
        self.lsb = lsb.into();
    }
}

// Application
/*
use crate::{register::Channel};

#[allow(dead_code)]
impl Frequency {
    pub fn apply(&self, opn2_sender: &OPN2CommandSender, channel: Channel) -> Result<(), Opn2CommandError> {
        self.msb.apply(opn2_sender, channel)?;
        self.lsb.apply(opn2_sender, channel)?;
        Ok(())
    }
}
*/
