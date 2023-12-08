mod channel_state;
mod global_state;
mod operator_state;

pub use channel_state::*;
pub use global_state::*;
pub use operator_state::*;

/*
use super::{Channel, Opn2Command};
use Channel::{Channel1, Channel2, Channel3, Channel4, Channel5, Channel6};

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Opn2State {
    clock_rate: u32,
    global: GlobalState,
    channel_1: ChannelState,
    channel_2: ChannelState,
    channel_3: ChannelState,
    channel_4: ChannelState,
    channel_5: ChannelState,
    channel_6: ChannelState,
}

// Getters
#[allow(dead_code)]
impl Opn2State {
    pub fn get_channel(&self, channel: Channel) -> &ChannelState {
        match channel {
            Channel1 => &self.channel_1,
            Channel2 => &self.channel_2,
            Channel3 => &self.channel_3,
            Channel4 => &self.channel_4,
            Channel5 => &self.channel_5,
            Channel6 => &self.channel_6,
        }
    }

    pub fn get_channel_mut(&mut self, channel: Channel) -> &mut ChannelState {
        match channel {
            Channel1 => &mut self.channel_1,
            Channel2 => &mut self.channel_2,
            Channel3 => &mut self.channel_3,
            Channel4 => &mut self.channel_4,
            Channel5 => &mut self.channel_5,
            Channel6 => &mut self.channel_6,
        }
    }
}

// Setters
#[allow(dead_code)]
impl Opn2State {
    pub fn set_global(&mut self, global: GlobalState) {
        self.global = global;
    }

    pub fn set_channel(&mut self, channel: Channel, state: ChannelState) {
        match channel {
            Channel1 => self.channel_1 = state,
            Channel2 => self.channel_2 = state,
            Channel3 => self.channel_3 = state,
            Channel4 => self.channel_4 = state,
            Channel5 => self.channel_5 = state,
            Channel6 => self.channel_6 = state,
        }
    }
}

// Builder-style methods
#[allow(dead_code)]
impl Opn2State {
    pub fn with_global(mut self, global: GlobalState) -> Self {
        self.set_global(global);
        self
    }

    pub fn with_channel(mut self, channel: Channel, state: ChannelState) -> Self {
        self.set_channel(channel, state);
        self
    }
}

// Application
/*
#[allow(dead_code)]
impl Opn2State {
    pub fn apply(&mut self, opn2_sender: &OPN2CommandSender) -> Result<(), Opn2CommandError> {
        self.global.apply(opn2_sender)?;

        self.channel_1.apply(opn2_sender, Channel1)?;
        self.channel_2.apply(opn2_sender, Channel2)?;
        self.channel_3.apply(opn2_sender, Channel3)?;
        self.channel_4.apply(opn2_sender, Channel4)?;
        self.channel_5.apply(opn2_sender, Channel5)?;
        self.channel_6.apply(opn2_sender, Channel6)?;

        Ok(())
    }
}
*/

#[allow(dead_code)]
impl Opn2State {
    pub fn process(&mut self, opn2_command: Opn2Command) {
        match opn2_command {
            Opn2Command::SetClockRate(clock_rate) => self.clock_rate = clock_rate,
            Opn2Command::Wait(_) => (),
            Opn2Command::Lfo(lfo) => self.global.set_lfo(lfo),
            Opn2Command::TimerAMsb(timer_a_msb) => self.global.set_timer_a_msb(timer_a_msb),
            Opn2Command::TimerALsb(timer_a_lsb) => self.global.set_timer_a_lsb(timer_a_lsb),
            Opn2Command::TimerB(timer_b) => self.global.set_timer_b(timer_b),
            Opn2Command::TimersAndChannel3Mode(timers_and_channel_3_mode) => {
                self.global
                    .set_timers(timers_and_channel_3_mode.get_timers());
                self.global
                    .set_channel_3_mode(timers_and_channel_3_mode.get_channel_3_mode());
            }
            Opn2Command::KeyOnOff(key_on_off) => self.global.set_key_on_off(key_on_off),
            Opn2Command::DacEnable(dac_enable) => self.global.set_dac_enable(dac_enable),
            Opn2Command::DacAmplitude(dac_amplitude) => {
                self.global.set_dac_amplitude(dac_amplitude)
            }
            Opn2Command::DetuneAndMultiple(_, channel, operator, dt_mul) => {
                let operator = self.get_channel_mut(channel).get_operator_mut(operator);
                operator.set_detune(dt_mul.get_detune());
                operator.set_multiple(dt_mul.get_multiple())
            }
            Opn2Command::TotalLevel(_, channel, operator, tl) => {
                let operator = self.get_channel_mut(channel).get_operator_mut(operator);
                operator.set_total_level(tl);
            }
            Opn2Command::RateScalingAndAttackRate(_, channel, operator, rs_ar) => {
                let operator = self.get_channel_mut(channel).get_operator_mut(operator);
                operator.set_rate_scaling(rs_ar.get_rate_scaling());
                operator.set_attack_rate(rs_ar.get_attack_rate());
            }
            Opn2Command::AmplitudeAndFirstDecayRate(_, channel, operator, am_d1r) => {
                let operator = self.get_channel_mut(channel).get_operator_mut(operator);
                operator.set_amplitude(am_d1r.get_amplitude());
                operator.set_first_decay_rate(am_d1r.get_first_decay_rate());
            }
            Opn2Command::SecondDecayRate(_, channel, operator, d2r) => {
                let operator = self.get_channel_mut(channel).get_operator_mut(operator);
                operator.set_second_decay_rate(d2r);
            }
            Opn2Command::SecondAmplitudeAndReleaseRate(_, channel, operator, d1l_rr) => {
                let operator = self.get_channel_mut(channel).get_operator_mut(operator);
                operator.set_second_amplitude(d1l_rr.get_second_amplitude());
                operator.set_release_rate(d1l_rr.get_release_rate())
            }
            Opn2Command::SoftwareSoundGeneratorMode(_, channel, operator, ssg_eg) => {
                let operator = self.get_channel_mut(channel).get_operator_mut(operator);
                operator.set_software_sound_generator(ssg_eg);
            }
            Opn2Command::FrequencyLsb(_, channel, lsb) => {
                let channel = self.get_channel_mut(channel);
                channel.set_frequency_lsb(lsb);
            }
            Opn2Command::FrequencyMsb(_, channel, msb) => {
                let channel = self.get_channel_mut(channel);
                channel.set_frequency_msb(msb);
            }
            Opn2Command::Channel3SupplementaryFrequencyLsb(_, channel, lsb) => {
                let channel = self.get_channel_mut(channel);
                channel.set_channel_3_supplementary_frequency_lsb(lsb);
            }
            Opn2Command::Channel3SupplementaryFrequencyMsb(_, channel, msb) => {
                let channel = self.get_channel_mut(channel);
                channel.set_channel_3_supplementary_frequency_msb(msb);
            }
            Opn2Command::FeedbackAndAlgorithm(_, channel, fb_al) => {
                let channel = self.get_channel_mut(channel);
                channel.set_feedback(fb_al.get_feedback());
                channel.set_algorithm(fb_al.get_algorithm());
            }
            Opn2Command::StereoAndLfoSensitivity(_, channel, st_lfo) => {
                let channel = self.get_channel_mut(channel);
                channel.set_stereo_output(st_lfo.get_stereo_output());
                channel.set_amplitude_modulation_sensitivity(
                    st_lfo.get_amplitude_modulation_sensitivity(),
                );
                channel.set_frequency_modulation_sensitivity(
                    st_lfo.get_frequency_modulation_sensitivity(),
                );
            }
        }
    }
}
*/