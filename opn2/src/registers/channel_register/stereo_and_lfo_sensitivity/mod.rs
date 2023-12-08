mod amplitude_modulation_sensitivity;
mod frequency_modulation_sensitivity;
mod stereo_output;

pub use amplitude_modulation_sensitivity::*;
pub use frequency_modulation_sensitivity::*;
pub use stereo_output::*;

use super::{ChannelRegister, Register};

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct StereoAndLfoSensitivity {
    stereo_output: StereoOutput,
    amplitude_modulation_sensitivity: AmplitudeModulationSensitivity,
    frequency_modulation_sensitivity: FrequencyModulationSensitivity,
}

impl StereoAndLfoSensitivity {
    pub fn new(
        stereo_output: StereoOutput,
        ams: AmplitudeModulationSensitivity,
        fms: FrequencyModulationSensitivity,
    ) -> Self {
        StereoAndLfoSensitivity {
            stereo_output,
            amplitude_modulation_sensitivity: ams,
            frequency_modulation_sensitivity: fms,
        }
    }
}

// Getters
impl StereoAndLfoSensitivity {
    pub fn get_stereo_output(&self) -> StereoOutput {
        self.stereo_output
    }

    pub fn get_amplitude_modulation_sensitivity(&self) -> AmplitudeModulationSensitivity {
        self.amplitude_modulation_sensitivity
    }

    pub fn get_frequency_modulation_sensitivity(&self) -> FrequencyModulationSensitivity {
        self.frequency_modulation_sensitivity
    }
}

// Setters
impl StereoAndLfoSensitivity {
    pub fn set_stereo_output<T>(&mut self, stereo_output: T)
    where
        T: Into<StereoOutput>,
    {
        self.stereo_output = stereo_output.into();
    }

    pub fn set_amplitude_modulation_sensitivity<T>(
        &mut self,
        amplitude_modulation_sensitivity: T,
    )
    where
        T: Into<AmplitudeModulationSensitivity>,
    {
        self.amplitude_modulation_sensitivity = amplitude_modulation_sensitivity.into();
    }

    pub fn set_frequency_modulation_sensitivity<T>(
        &mut self,
        frequency_modulation_sensitivity: T,
    )
    where
        T: Into<FrequencyModulationSensitivity>,
    {
        self.frequency_modulation_sensitivity = frequency_modulation_sensitivity.into();
    }
}

impl From<u8> for StereoAndLfoSensitivity {
    fn from(st_lfo: u8) -> Self {
        let fms = st_lfo & 0b11;
        let ams = (st_lfo >> 3) & 0b111;
        let right = (st_lfo >> 6) & 0b1 > 0;
        let left = (st_lfo >> 7) & 0b1 > 0;

        StereoAndLfoSensitivity::new(StereoOutput::new(left, right), ams.into(), fms.into())
    }
}

impl From<StereoAndLfoSensitivity> for u8 {
    fn from(val: StereoAndLfoSensitivity) -> Self {
        let (left, right) = val.stereo_output.get();
        let b4 = (left as u8) << 1;
        let b4 = b4 | (right as u8);
        let b4 = b4 << 3;
        let b4 = b4 | (val.amplitude_modulation_sensitivity as u8);
        let b4 = b4 << 3;
        b4 | (val.frequency_modulation_sensitivity as u8)
    }
}

impl Register for StereoAndLfoSensitivity {
    const BASE_ADDRESS: u8 = 0xB4;
}

impl ChannelRegister for StereoAndLfoSensitivity {
    fn get_data(&self) -> u8 {
        (*self).into()
    }
}
