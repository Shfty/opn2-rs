use std::fmt::Debug;

use crate::{registers::*, WaitSamples};

use super::Opn2Command;

/// A raw command to be handled by an OPN2 driver
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Opn2Instruction {
    SetClockRate(u32),
    Write(u32, u8),
    Wait(WaitSamples),
}

impl Debug for Opn2Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Opn2Instruction::SetClockRate(clock_rate) => {
                f.debug_tuple("SetClockRate").field(clock_rate).finish()
            }
            Opn2Instruction::Write(port, data) => f
                .debug_struct("Write")
                .field("port", port)
                .field(
                    "data",
                    &format!("{:?} (0x{:02x}) ({:#010b})", data, data, data),
                )
                .finish(),
            Opn2Instruction::Wait(samples) => f.debug_tuple("Wait").field(samples).finish(),
        }
    }
}

impl From<Opn2Command> for Vec<Opn2Instruction> {
    fn from(command: Opn2Command) -> Vec<Opn2Instruction> {
        match command {
            Opn2Command::SetClockRate(clock_rate) => {
                vec![Opn2Instruction::SetClockRate(clock_rate)]
            }
            Opn2Command::Wait(samples) => vec![Opn2Instruction::Wait(samples.into())],
            Opn2Command::Lfo(lfo) => vec![
                Opn2Instruction::Write(4000, Lfo::BASE_ADDRESS),
                Opn2Instruction::Write(4001, lfo.into()),
            ],
            Opn2Command::TimerAMsb(msb) => vec![
                Opn2Instruction::Write(4000, TimerAMsb::BASE_ADDRESS),
                Opn2Instruction::Write(4001, msb.into()),
            ],
            Opn2Command::TimerALsb(lsb) => vec![
                Opn2Instruction::Write(4000, TimerALsb::BASE_ADDRESS),
                Opn2Instruction::Write(4001, lsb.into()),
            ],
            Opn2Command::TimerB(timer_b) => vec![
                Opn2Instruction::Write(4000, TimerB::BASE_ADDRESS),
                Opn2Instruction::Write(4001, timer_b.into()),
            ],
            Opn2Command::TimersAndChannel3Mode(tcm) => vec![
                Opn2Instruction::Write(4000, TimersAndChannel3Mode::BASE_ADDRESS),
                Opn2Instruction::Write(4001, tcm.into()),
            ],
            Opn2Command::KeyOnOff(k) => vec![
                Opn2Instruction::Write(4000, KeyOnOff::BASE_ADDRESS),
                Opn2Instruction::Write(4001, k.into()),
            ],
            Opn2Command::DacEnable(dac_en) => vec![
                Opn2Instruction::Write(4000, DacEnable::BASE_ADDRESS),
                Opn2Instruction::Write(4001, dac_en.into()),
            ],
            Opn2Command::DacAmplitude(dac_am) => vec![
                Opn2Instruction::Write(4000, DacAmplitude::BASE_ADDRESS),
                Opn2Instruction::Write(4001, dac_am.into()),
            ],
            Opn2Command::DetuneAndMultiple(port, channel, operator, dtm) => {
                let port: u32 = port.into();
                vec![
                    Opn2Instruction::Write(port, DetuneAndMultiple::address_of(channel, operator)),
                    Opn2Instruction::Write(port + 1, dtm.into()),
                ]
            }
            Opn2Command::TotalLevel(port, channel, operator, tl) => {
                let port: u32 = port.into();
                vec![
                    Opn2Instruction::Write(port, TotalLevel::address_of(channel, operator)),
                    Opn2Instruction::Write(port + 1, tl.into()),
                ]
            }
            Opn2Command::RateScalingAndAttackRate(port, channel, operator, rs_ar) => {
                let port: u32 = port.into();
                vec![
                    Opn2Instruction::Write(
                        port,
                        RateScalingAndAttackRate::address_of(channel, operator),
                    ),
                    Opn2Instruction::Write(port + 1, rs_ar.into()),
                ]
            }
            Opn2Command::AmplitudeAndFirstDecayRate(port, channel, operator, am_d1r) => {
                let port: u32 = port.into();
                vec![
                    Opn2Instruction::Write(
                        port,
                        AmplitudeAndFirstDecayRate::address_of(channel, operator),
                    ),
                    Opn2Instruction::Write(port + 1, am_d1r.into()),
                ]
            }
            Opn2Command::SecondDecayRate(port, channel, operator, d2r) => {
                let port: u32 = port.into();
                vec![
                    Opn2Instruction::Write(port, SecondDecayRate::address_of(channel, operator)),
                    Opn2Instruction::Write(port + 1, d2r.into()),
                ]
            }
            Opn2Command::SecondAmplitudeAndReleaseRate(port, channel, operator, d1l_rr) => {
                let port: u32 = port.into();
                vec![
                    Opn2Instruction::Write(
                        port,
                        SecondAmplitudeAndReleaseRate::address_of(channel, operator),
                    ),
                    Opn2Instruction::Write(port + 1, d1l_rr.into()),
                ]
            }
            Opn2Command::SoftwareSoundGeneratorMode(port, channel, operator, ssg_eg) => {
                let port: u32 = port.into();
                vec![
                    Opn2Instruction::Write(
                        port,
                        SoftwareSoundGeneratorMode::address_of(channel, operator),
                    ),
                    Opn2Instruction::Write(port + 1, ssg_eg.into()),
                ]
            }
            Opn2Command::FrequencyLsb(port, channel, lsb) => {
                let port: u32 = port.into();
                vec![
                    Opn2Instruction::Write(port, FrequencyLsb::address_of(channel)),
                    Opn2Instruction::Write(port + 1, lsb.into()),
                ]
            }
            Opn2Command::FrequencyMsb(port, channel, msb) => {
                let port: u32 = port.into();
                vec![
                    Opn2Instruction::Write(port, FrequencyMsb::address_of(channel)),
                    Opn2Instruction::Write(port + 1, msb.into()),
                ]
            }
            Opn2Command::Channel3SupplementaryFrequencyLsb(port, channel, lsb) => {
                let port: u32 = port.into();
                vec![
                    Opn2Instruction::Write(
                        port,
                        Channel3SupplementaryFrequencyLsb::address_of(channel),
                    ),
                    Opn2Instruction::Write(port + 1, lsb.into()),
                ]
            }
            Opn2Command::Channel3SupplementaryFrequencyMsb(port, channel, msb) => {
                let port: u32 = port.into();
                vec![
                    Opn2Instruction::Write(
                        port,
                        Channel3SupplementaryFrequencyMsb::address_of(channel),
                    ),
                    Opn2Instruction::Write(port + 1, msb.into()),
                ]
            }
            Opn2Command::FeedbackAndAlgorithm(port, channel, fb_al) => {
                let port: u32 = port.into();
                vec![
                    Opn2Instruction::Write(port, FeedbackAndAlgorithm::address_of(channel)),
                    Opn2Instruction::Write(port + 1, fb_al.into()),
                ]
            }
            Opn2Command::StereoAndLfoSensitivity(port, channel, st_lfo) => {
                let port: u32 = port.into();
                vec![
                    Opn2Instruction::Write(port, StereoAndLfoSensitivity::address_of(channel)),
                    Opn2Instruction::Write(port + 1, st_lfo.into()),
                ]
            }
        }
    }
}
