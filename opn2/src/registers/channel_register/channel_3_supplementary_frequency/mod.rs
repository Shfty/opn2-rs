mod channel_3_supplementary_frequency_lsb;
mod channel_3_supplementary_frequency_msb;

pub use channel_3_supplementary_frequency_lsb::*;
pub use channel_3_supplementary_frequency_msb::*;

use super::ChannelRegister;

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Channel3SupplementaryFrequency {
    msb: Channel3SupplementaryFrequencyMsb,
    lsb: Channel3SupplementaryFrequencyLsb,
}

#[allow(dead_code)]
impl Channel3SupplementaryFrequency {
    pub fn new(
        msb: Channel3SupplementaryFrequencyMsb,
        lsb: Channel3SupplementaryFrequencyLsb,
    ) -> Self {
        Channel3SupplementaryFrequency { lsb, msb }
    }
}

// Getters
impl Channel3SupplementaryFrequency {
    pub fn get_msb(&self) -> Channel3SupplementaryFrequencyMsb {
        self.msb
    }

    pub fn get_lsb(&self) -> Channel3SupplementaryFrequencyLsb {
        self.lsb
    }
}

// Setters
impl Channel3SupplementaryFrequency {
    pub fn set_msb<T>(&mut self, msb: T)
    where
        T: Into<Channel3SupplementaryFrequencyMsb>,
    {
        self.msb = msb.into();
    }

    pub fn set_lsb<T>(&mut self, lsb: T)
    where
        T: Into<Channel3SupplementaryFrequencyLsb>,
    {
        self.lsb = lsb.into();
    }
}

// Application
/*
use crate::register::Channel;

impl Channel3SupplementaryFrequency {
    pub fn apply(&self, opn2_sender: &OPN2CommandSender, channel: Channel) -> Result<(), Opn2CommandError> {
        self.msb.apply(opn2_sender, channel)?;
        self.lsb.apply(opn2_sender, channel)?;
        Ok(())
    }
}
*/
