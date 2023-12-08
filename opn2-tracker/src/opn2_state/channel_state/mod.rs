mod grand_piano;
pub use grand_piano::*;

/*
use Operator::{Operator1, Operator2, Operator3, Operator4};

use dirty_guard::DirtyGuard;

use crate::{
    register::FeedbackAndAlgorithm,
    register::{
        Algorithm, AmplitudeModulationSensitivity, Channel3SupplementaryFrequency,
        Channel3SupplementaryFrequencyLsb, Channel3SupplementaryFrequencyMsb, Feedback, Frequency, FrequencyLsb, FrequencyMsb, FrequencyModulationSensitivity, Operator,
        StereoAndLfoSensitivity, StereoOutput,
    },
};

use super::OperatorState;

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ChannelState {
    feedback_and_algorithm: DirtyGuard<FeedbackAndAlgorithm>,
    stereo_and_lfo_sensitivity: DirtyGuard<StereoAndLfoSensitivity>,
    frequency: DirtyGuard<Frequency>,
    channel_3_supplementary_frequency: DirtyGuard<Channel3SupplementaryFrequency>,
    operator_1: OperatorState,
    operator_2: OperatorState,
    operator_3: OperatorState,
    operator_4: OperatorState,
}

// Getters
#[allow(dead_code)]
impl ChannelState {
    pub fn get_feedback(&self) -> Feedback {
        self.feedback_and_algorithm.read().get_feedback()
    }

    pub fn get_algorithm(&self) -> Algorithm {
        self.feedback_and_algorithm.read().get_algorithm()
    }

    pub fn get_stereo_output(&self) -> StereoOutput {
        self.stereo_and_lfo_sensitivity.read().get_stereo_output()
    }

    pub fn get_amplitude_modulation_sensitivity(&self) -> AmplitudeModulationSensitivity {
        self.stereo_and_lfo_sensitivity
            .read()
            .get_amplitude_modulation_sensitivity()
    }

    pub fn get_frequency_modulation_sensitivity(&self) -> FrequencyModulationSensitivity {
        self.stereo_and_lfo_sensitivity
            .read()
            .get_frequency_modulation_sensitivity()
    }

    pub fn get_frequency_msb(&self) -> FrequencyMsb {
        self.frequency.read().get_msb()
    }

    pub fn get_frequency_lsb(&self) -> FrequencyLsb {
        self.frequency.read().get_lsb()
    }

    pub fn get_channel_3_supplementary_frequency_msb(&self) -> Channel3SupplementaryFrequencyMsb {
        self.channel_3_supplementary_frequency.read().get_msb()
    }

    pub fn get_channel_3_supplementary_frequency_lsb(&self) -> Channel3SupplementaryFrequencyLsb {
        self.channel_3_supplementary_frequency.read().get_lsb()
    }

    pub fn get_operator(&self, operator: Operator) -> &OperatorState {
        match operator {
            Operator1 => &self.operator_1,
            Operator2 => &self.operator_2,
            Operator3 => &self.operator_3,
            Operator4 => &self.operator_4,
        }
    }
    pub fn get_operator_mut(&mut self, operator: Operator) -> &mut OperatorState {
        match operator {
            Operator1 => &mut self.operator_1,
            Operator2 => &mut self.operator_2,
            Operator3 => &mut self.operator_3,
            Operator4 => &mut self.operator_4,
        }
    }
}

// Setters
impl ChannelState {
    pub fn set_feedback<T>(&mut self, feedback: T)
    where
        T: Into<Feedback>,
    {
        self.feedback_and_algorithm.write().set_feedback(feedback);
    }

    pub fn set_algorithm<T>(&mut self, algorithm: T)
    where
        T: Into<Algorithm>,
    {
        self.feedback_and_algorithm.write().set_algorithm(algorithm)
    }

    pub fn set_stereo_output<T>(&mut self, stereo_output: T)
    where
        T: Into<StereoOutput>,
    {
        self.stereo_and_lfo_sensitivity
            .write()
            .set_stereo_output(stereo_output);
    }

    pub fn set_amplitude_modulation_sensitivity<T>(&mut self, ams: T)
    where
        T: Into<AmplitudeModulationSensitivity>,
    {
        self.stereo_and_lfo_sensitivity
            .write()
            .set_amplitude_modulation_sensitivity(ams);
    }

    pub fn set_frequency_modulation_sensitivity<T>(&mut self, fms: T)
    where
        T: Into<FrequencyModulationSensitivity>,
    {
        self.stereo_and_lfo_sensitivity
            .write()
            .set_frequency_modulation_sensitivity(fms);
    }

    pub fn set_frequency_msb<T>(&mut self, msb: T)
    where
        T: Into<FrequencyMsb>,
    {
        self.frequency.write().set_msb(msb);
    }

    pub fn set_frequency_lsb<T>(&mut self, lsb: T)
    where
        T: Into<FrequencyLsb>,
    {
        self.frequency.write().set_lsb(lsb);
    }

    pub fn set_channel_3_supplementary_frequency_msb<T>(&mut self, msb: T)
    where
        T: Into<Channel3SupplementaryFrequencyMsb>,
    {
        self.channel_3_supplementary_frequency.write().set_msb(msb);
    }

    pub fn set_channel_3_supplementary_frequency_lsb<T>(&mut self, lsb: T)
    where
        T: Into<Channel3SupplementaryFrequencyLsb>,
    {
        self.channel_3_supplementary_frequency.write().set_lsb(lsb);
    }

    pub fn set_operator<T>(&mut self, operator: Operator, state: T)
    where
        T: Into<OperatorState>,
    {
        match operator {
            Operator1 => self.operator_1 = state.into(),
            Operator2 => self.operator_2 = state.into(),
            Operator3 => self.operator_3 = state.into(),
            Operator4 => self.operator_4 = state.into(),
        }
    }
}

// Builder-style methods
#[allow(dead_code)]
impl ChannelState {
    pub fn with_feedback<T>(mut self, feedback: T) -> Self
    where
        T: Into<Feedback>,
    {
        self.set_feedback(feedback);
        self
    }

    pub fn with_algorithm<T>(mut self, algorithm: T) -> Self
    where
        T: Into<Algorithm>,
    {
        self.set_algorithm(algorithm);
        self
    }

    pub fn with_stereo_output<T>(mut self, stereo_output: T) -> Self
    where
        T: Into<StereoOutput>,
    {
        self.set_stereo_output(stereo_output);
        self
    }

    pub fn with_amplitude_modulation_sensitivity<T>(mut self, ams: T) -> Self
    where
        T: Into<AmplitudeModulationSensitivity>,
    {
        self.set_amplitude_modulation_sensitivity(ams);
        self
    }

    pub fn with_frequency_modulation_sensitivity<T>(mut self, fms: T) -> Self
    where
        T: Into<FrequencyModulationSensitivity>,
    {
        self.set_frequency_modulation_sensitivity(fms);
        self
    }

    pub fn with_frequency_msb<T>(mut self, msb: T) -> Self
    where
        T: Into<FrequencyMsb>,
    {
        self.set_frequency_msb(msb);
        self
    }

    pub fn with_frequency_lsb<T>(mut self, msb: T) -> Self
    where
        T: Into<FrequencyLsb>,
    {
        self.set_frequency_lsb(msb);
        self
    }

    pub fn with_channel_3_supplementary_frequency_msb<T>(mut self, msb: T) -> Self
    where
        T: Into<Channel3SupplementaryFrequencyMsb>,
    {
        self.set_channel_3_supplementary_frequency_msb(msb);
        self
    }

    pub fn with_channel_3_supplementary_frequency_lsb<T>(mut self, msb: T) -> Self
    where
        T: Into<Channel3SupplementaryFrequencyLsb>,
    {
        self.set_channel_3_supplementary_frequency_lsb(msb);
        self
    }

    pub fn with_operator<T>(mut self, operator: Operator, state: T) -> Self
    where
        T: Into<OperatorState>,
    {
        self.set_operator(operator, state);
        self
    }
}

// Application
use crate::register::ChannelRegister;
use crate::register::Channel;

impl ChannelState {
    pub fn apply(
        &mut self,
        opn2_sender: &OPN2CommandSender,
        channel: Channel,
    ) -> Result<(), Opn2CommandError> {
        self.operator_1.apply(opn2_sender, channel, Operator1)?;
        self.operator_2.apply(opn2_sender, channel, Operator2)?;
        self.operator_3.apply(opn2_sender, channel, Operator3)?;
        self.operator_4.apply(opn2_sender, channel, Operator4)?;

        if let Some(feedback_and_algorithm) = self.feedback_and_algorithm.try_read() {
            feedback_and_algorithm.apply(opn2_sender, channel)?;
        }

        if let Some(stereo_and_lfo_sensitivity) = self.stereo_and_lfo_sensitivity.try_read() {
            stereo_and_lfo_sensitivity.apply(opn2_sender, channel)?;
        }

        if let Some(frequency) = self.frequency.try_read() {
            frequency.apply(opn2_sender, channel)?;
        }

        if let Some(channel_3_supplementary_frequency) =
            self.channel_3_supplementary_frequency.try_read()
        {
            channel_3_supplementary_frequency.apply(opn2_sender, channel)?;
        }

        Ok(())
    }
}
*/