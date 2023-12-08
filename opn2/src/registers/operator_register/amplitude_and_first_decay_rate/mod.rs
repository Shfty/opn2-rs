mod amplitude;
mod first_decay_rate;

pub use amplitude::*;
pub use first_decay_rate::*;

use super::{OperatorRegister, Register};

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct AmplitudeAndFirstDecayRate {
    amplitude: Amplitude,
    first_decay_rate: FirstDecayRate,
}

impl AmplitudeAndFirstDecayRate {
    pub fn new(am: Amplitude, d1r: FirstDecayRate) -> Self {
        AmplitudeAndFirstDecayRate {
            amplitude: am,
            first_decay_rate: d1r,
        }
    }
}

// Getters
impl AmplitudeAndFirstDecayRate {
    pub fn get_amplitude(&self) -> Amplitude {
        self.amplitude
    }

    pub fn get_first_decay_rate(&self) -> FirstDecayRate {
        self.first_decay_rate
    }
}

// Setters
impl AmplitudeAndFirstDecayRate {
    pub fn set_amplitude<T>(&mut self, amplitude: T)
    where
        T: Into<Amplitude>,
    {
        self.amplitude = amplitude.into();
    }

    pub fn set_first_decay_rate<T>(&mut self, first_decay_rate: T)
    where
        T: Into<FirstDecayRate>,
    {
        self.first_decay_rate = first_decay_rate.into();
    }
}

impl From<u8> for AmplitudeAndFirstDecayRate {
    fn from(am_and_d1r: u8) -> Self {
        let am = am_and_d1r >> 7;
        let d1r = am_and_d1r & 0b11111;
        AmplitudeAndFirstDecayRate::new(Amplitude::new(am), FirstDecayRate::new(d1r))
    }
}

impl From<AmplitudeAndFirstDecayRate> for u8 {
    fn from(val: AmplitudeAndFirstDecayRate) -> Self {
        (val.amplitude.get() << 7) | val.first_decay_rate.get()
    }
}

impl Register for AmplitudeAndFirstDecayRate {
    const BASE_ADDRESS: u8 = 0x60;
}

impl OperatorRegister for AmplitudeAndFirstDecayRate {
    fn get_data(&self) -> u8 {
        (*self).into()
    }
}
