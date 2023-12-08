mod release_rate;
mod second_amplitude;

pub use release_rate::*;
pub use second_amplitude::*;

use super::{OperatorRegister, Register};

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct SecondAmplitudeAndReleaseRate {
    second_amplitude: SecondAmplitude,
    release_rate: ReleaseRate,
}

impl SecondAmplitudeAndReleaseRate {
    pub fn new(d1l: SecondAmplitude, rr: ReleaseRate) -> Self {
        SecondAmplitudeAndReleaseRate {
            second_amplitude: d1l,
            release_rate: rr,
        }
    }
}

// Getters
impl SecondAmplitudeAndReleaseRate {
    pub fn get_second_amplitude(&self) -> SecondAmplitude {
        self.second_amplitude
    }

    pub fn get_release_rate(&self) -> ReleaseRate {
        self.release_rate
    }
}

// Setters
impl SecondAmplitudeAndReleaseRate {
    pub fn set_second_amplitude<T>(&mut self, second_amplitude: T)
    where
        T: Into<SecondAmplitude>,
    {
        self.second_amplitude = second_amplitude.into();
    }

    pub fn set_release_rate<T>(&mut self, release_rate: T)
    where
        T: Into<ReleaseRate>,
    {
        self.release_rate = release_rate.into();
    }
}

impl From<u8> for SecondAmplitudeAndReleaseRate {
    fn from(d1l_and_rr: u8) -> Self {
        let d1l = d1l_and_rr >> 4;
        let rr = d1l_and_rr & 0b1111;
        SecondAmplitudeAndReleaseRate::new(
            SecondAmplitude::new(d1l as u8),
            ReleaseRate::new(rr as u8),
        )
    }
}

impl From<SecondAmplitudeAndReleaseRate> for u8 {
    fn from(val: SecondAmplitudeAndReleaseRate) -> Self {
        (val.second_amplitude.get() << 4) | val.release_rate.get()
    }
}

impl Register for SecondAmplitudeAndReleaseRate {
    const BASE_ADDRESS: u8 = 0x80;
}

impl OperatorRegister for SecondAmplitudeAndReleaseRate {
    fn get_data(&self) -> u8 {
        (*self).into()
    }
}
