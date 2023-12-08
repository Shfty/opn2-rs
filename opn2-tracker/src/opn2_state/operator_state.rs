use dirty_guard::DirtyGuard;

use opn2::registers::{
    Amplitude, AmplitudeAndFirstDecayRate, AttackRate, Detune, DetuneAndMultiple, FirstDecayRate,
    Multiple, RateScaling, RateScalingAndAttackRate, ReleaseRate, SecondAmplitude,
    SecondAmplitudeAndReleaseRate, SecondDecayRate, SoftwareSoundGeneratorMode, TotalLevel,
};

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct OperatorState {
    detune_and_multiple: DirtyGuard<DetuneAndMultiple>,
    total_level: DirtyGuard<TotalLevel>,
    rate_scaling_and_attack_rate: DirtyGuard<RateScalingAndAttackRate>,
    amplitude_and_first_decay_rate: DirtyGuard<AmplitudeAndFirstDecayRate>,
    second_decay_rate: DirtyGuard<SecondDecayRate>,
    second_amplitude_and_release_rate: DirtyGuard<SecondAmplitudeAndReleaseRate>,
    software_sound_generator_mode: DirtyGuard<SoftwareSoundGeneratorMode>,
}

// Getters
#[allow(dead_code)]
impl OperatorState {
    pub fn get_detune(&self) -> Detune {
        self.detune_and_multiple.read().get_detune()
    }

    pub fn get_multiple(&self) -> Multiple {
        self.detune_and_multiple.read().get_multiple()
    }

    pub fn get_total_level(&self) -> TotalLevel {
        *self.total_level.read()
    }

    pub fn get_rate_scaling(&self) -> RateScaling {
        self.rate_scaling_and_attack_rate.read().get_rate_scaling()
    }

    pub fn get_attack_rate(&self) -> AttackRate {
        self.rate_scaling_and_attack_rate.read().get_attack_rate()
    }

    pub fn get_amplitude(&self) -> Amplitude {
        self.amplitude_and_first_decay_rate.read().get_amplitude()
    }

    pub fn get_first_decay_rate(&self) -> FirstDecayRate {
        self.amplitude_and_first_decay_rate
            .read()
            .get_first_decay_rate()
    }

    pub fn get_second_decay_rate(&self) -> SecondDecayRate {
        *self.second_decay_rate.read()
    }

    pub fn get_second_amplitude(&self) -> SecondAmplitude {
        self.second_amplitude_and_release_rate
            .read()
            .get_second_amplitude()
    }

    pub fn get_release_rate(&self) -> ReleaseRate {
        self.second_amplitude_and_release_rate
            .read()
            .get_release_rate()
    }

    pub fn get_software_sound_generator_mode(&self) -> SoftwareSoundGeneratorMode {
        *self.software_sound_generator_mode.read()
    }
}

// Setters
#[allow(dead_code)]
impl OperatorState {
    pub fn set_detune<T>(&mut self, dt: T)
    where
        T: Into<Detune>,
    {
        self.detune_and_multiple.write().set_detune(dt);
    }

    pub fn set_multiple<T>(&mut self, mul: T)
    where
        T: Into<Multiple>,
    {
        self.detune_and_multiple.write().set_multiple(mul);
    }

    pub fn set_total_level<T>(&mut self, tl: T)
    where
        T: Into<TotalLevel>,
    {
        self.total_level.write().set(tl.into());
    }

    pub fn set_rate_scaling<T>(&mut self, rs: T)
    where
        T: Into<RateScaling>,
    {
        self.rate_scaling_and_attack_rate
            .write()
            .set_rate_scaling(rs);
    }

    pub fn set_attack_rate<T>(&mut self, ar: T)
    where
        T: Into<AttackRate>,
    {
        self.rate_scaling_and_attack_rate
            .write()
            .set_attack_rate(ar);
    }

    pub fn set_amplitude<T>(&mut self, am: T)
    where
        T: Into<Amplitude>,
    {
        self.amplitude_and_first_decay_rate
            .write()
            .set_amplitude(am);
    }

    pub fn set_first_decay_rate<T>(&mut self, d1r: T)
    where
        T: Into<FirstDecayRate>,
    {
        self.amplitude_and_first_decay_rate
            .write()
            .set_first_decay_rate(d1r);
    }

    pub fn set_second_decay_rate<T>(&mut self, d2r: T)
    where
        T: Into<SecondDecayRate>,
    {
        self.second_decay_rate.write().set(d2r.into());
    }

    pub fn set_second_amplitude<T>(&mut self, d1l: T)
    where
        T: Into<SecondAmplitude>,
    {
        self.second_amplitude_and_release_rate
            .write()
            .set_second_amplitude(d1l);
    }

    pub fn set_release_rate<T>(&mut self, rr: T)
    where
        T: Into<ReleaseRate>,
    {
        self.second_amplitude_and_release_rate
            .write()
            .set_release_rate(rr);
    }

    pub fn set_software_sound_generator<T>(&mut self, ssg_eg: T)
    where
        T: Into<SoftwareSoundGeneratorMode>,
    {
        self.software_sound_generator_mode
            .write()
            .set(ssg_eg.into());
    }
}

// Builder-style methods
#[allow(dead_code)]
impl OperatorState {
    pub fn with_detune<T>(mut self, dt: T) -> Self
    where
        T: Into<Detune>,
    {
        self.set_detune(dt);
        self
    }

    pub fn with_multiple<T>(mut self, mul: T) -> Self
    where
        T: Into<Multiple>,
    {
        self.set_multiple(mul);
        self
    }

    pub fn with_total_level<T>(mut self, tl: T) -> Self
    where
        T: Into<TotalLevel>,
    {
        self.set_total_level(tl);
        self
    }

    pub fn with_rate_scaling<T>(mut self, rs: T) -> Self
    where
        T: Into<RateScaling>,
    {
        self.set_rate_scaling(rs);
        self
    }

    pub fn with_attack_rate<T>(mut self, ar: T) -> Self
    where
        T: Into<AttackRate>,
    {
        self.set_attack_rate(ar);
        self
    }

    pub fn with_amplitude<T>(mut self, am: T) -> Self
    where
        T: Into<Amplitude>,
    {
        self.set_amplitude(am);
        self
    }

    pub fn with_first_decay_rate<T>(mut self, d1r: T) -> Self
    where
        T: Into<FirstDecayRate>,
    {
        self.set_first_decay_rate(d1r);
        self
    }

    pub fn with_second_decay_rate<T>(mut self, d2r: T) -> Self
    where
        T: Into<SecondDecayRate>,
    {
        self.set_second_decay_rate(d2r);
        self
    }

    pub fn with_second_amplitude<T>(mut self, d1l: T) -> Self
    where
        T: Into<SecondAmplitude>,
    {
        self.set_second_amplitude(d1l);
        self
    }

    pub fn with_release_rate<T>(mut self, rr: T) -> Self
    where
        T: Into<ReleaseRate>,
    {
        self.set_release_rate(rr);
        self
    }

    pub fn with_software_sound_generator<T>(mut self, ssg_eg: T) -> Self
    where
        T: Into<SoftwareSoundGeneratorMode>,
    {
        self.set_software_sound_generator(ssg_eg);
        self
    }
}

// Application
/*
use crate::register::OperatorRegister;
use crate::register::Operator;
use crate::register::Channel;

impl OperatorState {
    pub fn apply(
        &mut self,
        opn2_sender: &OPN2CommandSender,
        channel: Channel,
        operator: Operator,
    ) -> Result<(), Opn2CommandError> {
        if let Some(detune_and_multiple) = self.detune_and_multiple.try_read() {
            detune_and_multiple.apply(opn2_sender, channel, operator)?;
        }

        if let Some(total_level) = self.total_level.try_read() {
            total_level.apply(opn2_sender, channel, operator)?;
        }

        if let Some(rate_scaling_and_attack_rate) = self.rate_scaling_and_attack_rate.try_read() {
            rate_scaling_and_attack_rate.apply(opn2_sender, channel, operator)?;
        }

        if let Some(amplitude_and_first_decay_rate) = self.amplitude_and_first_decay_rate.try_read()
        {
            amplitude_and_first_decay_rate.apply(opn2_sender, channel, operator)?;
        }

        if let Some(second_decay_rate) = self.second_decay_rate.try_read() {
            second_decay_rate.apply(opn2_sender, channel, operator)?;
        }

        if let Some(second_amplitude_and_release_rate) =
            self.second_amplitude_and_release_rate.try_read()
        {
            second_amplitude_and_release_rate.apply(opn2_sender, channel, operator)?;
        }

        if let Some(software_sound_generator_mode) = self.software_sound_generator_mode.try_read() {
            software_sound_generator_mode.apply(opn2_sender, channel, operator)?;
        }

        Ok(())
    }
}
*/
