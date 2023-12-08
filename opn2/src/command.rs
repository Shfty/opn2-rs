use std::time::Duration;

use crate::registers::*;

/// A high-level abstraction over commands used to address an OPN2 chip
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Opn2Command {
    SetClockRate(u32),
    Wait(Duration),
    Lfo(Lfo),
    TimerAMsb(TimerAMsb),
    TimerALsb(TimerALsb),
    TimerB(TimerB),
    TimersAndChannel3Mode(TimersAndChannel3Mode),
    KeyOnOff(KeyOnOff),
    DacEnable(DacEnable),
    DacAmplitude(DacAmplitude),
    DetuneAndMultiple(Port, Channel, Operator, DetuneAndMultiple),
    TotalLevel(Port, Channel, Operator, TotalLevel),
    RateScalingAndAttackRate(Port, Channel, Operator, RateScalingAndAttackRate),
    AmplitudeAndFirstDecayRate(Port, Channel, Operator, AmplitudeAndFirstDecayRate),
    SecondDecayRate(Port, Channel, Operator, SecondDecayRate),
    SecondAmplitudeAndReleaseRate(Port, Channel, Operator, SecondAmplitudeAndReleaseRate),
    SoftwareSoundGeneratorMode(Port, Channel, Operator, SoftwareSoundGeneratorMode),
    FrequencyLsb(Port, Channel, FrequencyLsb),
    FrequencyMsb(Port, Channel, FrequencyMsb),
    Channel3SupplementaryFrequencyLsb(Port, Channel, Channel3SupplementaryFrequencyLsb),
    Channel3SupplementaryFrequencyMsb(Port, Channel, Channel3SupplementaryFrequencyMsb),
    FeedbackAndAlgorithm(Port, Channel, FeedbackAndAlgorithm),
    StereoAndLfoSensitivity(Port, Channel, StereoAndLfoSensitivity),
}

impl Opn2Command {
    pub fn get_port(&self) -> Option<Port> {
        match self {
            Opn2Command::SetClockRate(_) => None,
            Opn2Command::Wait(_) => None,
            Opn2Command::Lfo(_) => None,
            Opn2Command::TimerAMsb(_) => None,
            Opn2Command::TimerALsb(_) => None,
            Opn2Command::TimerB(_) => None,
            Opn2Command::TimersAndChannel3Mode(_) => None,
            Opn2Command::KeyOnOff(_) => None,
            Opn2Command::DacEnable(_) => None,
            Opn2Command::DacAmplitude(_) => None,
            Opn2Command::DetuneAndMultiple(port, _, _, _) => Some(*port),
            Opn2Command::TotalLevel(port, _, _, _) => Some(*port),
            Opn2Command::RateScalingAndAttackRate(port, _, _, _) => Some(*port),
            Opn2Command::AmplitudeAndFirstDecayRate(port, _, _, _) => Some(*port),
            Opn2Command::SecondDecayRate(port, _, _, _) => Some(*port),
            Opn2Command::SecondAmplitudeAndReleaseRate(port, _, _, _) => Some(*port),
            Opn2Command::SoftwareSoundGeneratorMode(port, _, _, _) => Some(*port),
            Opn2Command::FrequencyLsb(port, _, _) => Some(*port),
            Opn2Command::FrequencyMsb(port, _, _) => Some(*port),
            Opn2Command::Channel3SupplementaryFrequencyLsb(port, _, _) => Some(*port),
            Opn2Command::Channel3SupplementaryFrequencyMsb(port, _, _) => Some(*port),
            Opn2Command::FeedbackAndAlgorithm(port, _, _) => Some(*port),
            Opn2Command::StereoAndLfoSensitivity(port, _, _) => Some(*port),
        }
    }

    pub fn get_channel(&self) -> Option<Channel> {
        match self {
            Opn2Command::SetClockRate(_) => None,
            Opn2Command::Wait(_) => None,
            Opn2Command::Lfo(_) => None,
            Opn2Command::TimerAMsb(_) => None,
            Opn2Command::TimerALsb(_) => None,
            Opn2Command::TimerB(_) => None,
            Opn2Command::TimersAndChannel3Mode(_) => None,
            Opn2Command::KeyOnOff(_) => None,
            Opn2Command::DacEnable(_) => None,
            Opn2Command::DacAmplitude(_) => None,
            Opn2Command::DetuneAndMultiple(_, channel, _, _) => Some(*channel),
            Opn2Command::TotalLevel(_, channel, _, _) => Some(*channel),
            Opn2Command::RateScalingAndAttackRate(_, channel, _, _) => Some(*channel),
            Opn2Command::AmplitudeAndFirstDecayRate(_, channel, _, _) => Some(*channel),
            Opn2Command::SecondDecayRate(_, channel, _, _) => Some(*channel),
            Opn2Command::SecondAmplitudeAndReleaseRate(_, channel, _, _) => Some(*channel),
            Opn2Command::SoftwareSoundGeneratorMode(_, channel, _, _) => Some(*channel),
            Opn2Command::FrequencyLsb(_, channel, _) => Some(*channel),
            Opn2Command::FrequencyMsb(_, channel, _) => Some(*channel),
            Opn2Command::Channel3SupplementaryFrequencyLsb(_, channel, _) => Some(*channel),
            Opn2Command::Channel3SupplementaryFrequencyMsb(_, channel, _) => Some(*channel),
            Opn2Command::FeedbackAndAlgorithm(_, channel, _) => Some(*channel),
            Opn2Command::StereoAndLfoSensitivity(_, channel, _) => Some(*channel),
        }
    }

    pub fn get_operator(&self) -> Option<Operator> {
        match self {
            Opn2Command::SetClockRate(_) => None,
            Opn2Command::Wait(_) => None,
            Opn2Command::Lfo(_) => None,
            Opn2Command::TimerAMsb(_) => None,
            Opn2Command::TimerALsb(_) => None,
            Opn2Command::TimerB(_) => None,
            Opn2Command::TimersAndChannel3Mode(_) => None,
            Opn2Command::KeyOnOff(_) => None,
            Opn2Command::DacEnable(_) => None,
            Opn2Command::DacAmplitude(_) => None,
            Opn2Command::DetuneAndMultiple(_, _, operator, _) => Some(*operator),
            Opn2Command::TotalLevel(_, _, operator, _) => Some(*operator),
            Opn2Command::RateScalingAndAttackRate(_, _, operator, _) => Some(*operator),
            Opn2Command::AmplitudeAndFirstDecayRate(_, _, operator, _) => Some(*operator),
            Opn2Command::SecondDecayRate(_, _, operator, _) => Some(*operator),
            Opn2Command::SecondAmplitudeAndReleaseRate(_, _, operator, _) => Some(*operator),
            Opn2Command::SoftwareSoundGeneratorMode(_, _, operator, _) => Some(*operator),
            Opn2Command::FrequencyLsb(_, _, _) => None,
            Opn2Command::FrequencyMsb(_, _, _) => None,
            Opn2Command::Channel3SupplementaryFrequencyLsb(_, _, _) => None,
            Opn2Command::Channel3SupplementaryFrequencyMsb(_, _, _) => None,
            Opn2Command::FeedbackAndAlgorithm(_, _, _) => None,
            Opn2Command::StereoAndLfoSensitivity(_, _, _) => None,
        }
    }
}
