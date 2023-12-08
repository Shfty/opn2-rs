//! Functionality for converting VGM data into OPN2 commands.

use std::convert::{TryFrom, TryInto};

use crate::{VgmCommand, VgmFile};

use opn2::{registers::*, Opn2Command, Opn2Instruction};

impl TryFrom<VgmCommand> for Opn2Command {
    type Error = String;

    fn try_from(command: VgmCommand) -> Result<Self, Self::Error> {
        fn operator<T>(f: u8) -> Result<Operator, String>
        where
            T: OperatorRegister,
        {
            let address = (f - T::BASE_ADDRESS) - (f % 4);
            match f {
                _ if address == T::OPERATOR_1_OFFSET => Ok(Operator::Operator1),
                _ if address == T::OPERATOR_2_OFFSET => Ok(Operator::Operator2),
                _ if address == T::OPERATOR_3_OFFSET => Ok(Operator::Operator3),
                _ if address == T::OPERATOR_4_OFFSET => Ok(Operator::Operator4),
                f => Err(format!(
                    "Unrecognized operator offset {:?} for base address {:?}",
                    f,
                    T::BASE_ADDRESS
                )),
            }
        }

        fn port_channel<T>(f: u8) -> Result<PortChannel, String>
        where
            T: ChannelRegister,
        {
            let address = f - T::BASE_ADDRESS;
            match f {
                _ if address % 4 == T::CHANNEL_1_OFFSET => Ok(PortChannel::PortChannel1),
                _ if address % 4 == T::CHANNEL_2_OFFSET => Ok(PortChannel::PortChannel2),
                _ if address % 4 == T::CHANNEL_3_OFFSET => Ok(PortChannel::PortChannel3),
                f => Err(format!(
                    "Unrecognized channel offset {:?} for base address {:?}",
                    f,
                    T::BASE_ADDRESS
                )),
            }
        }

        fn operator_port_channel<T>(f: u8) -> Result<PortChannel, String>
        where
            T: OperatorRegister,
        {
            let address = f - T::BASE_ADDRESS;
            match f {
                _ if address % 4 == T::CHANNEL_1_OFFSET => Ok(PortChannel::PortChannel1),
                _ if address % 4 == T::CHANNEL_2_OFFSET => Ok(PortChannel::PortChannel2),
                _ if address % 4 == T::CHANNEL_3_OFFSET => Ok(PortChannel::PortChannel3),
                f => Err(format!(
                    "Unrecognized channel offset {:?} for base address {:?}",
                    f,
                    T::BASE_ADDRESS
                )),
            }
        }

        fn channel(port: u32, port_channel: PortChannel) -> Result<Channel, String> {
            match port {
                4000 => match port_channel {
                    PortChannel::PortChannel1 => Ok(Channel::Channel1),
                    PortChannel::PortChannel2 => Ok(Channel::Channel2),
                    PortChannel::PortChannel3 => Ok(Channel::Channel3),
                },
                4002 => match port_channel {
                    PortChannel::PortChannel1 => Ok(Channel::Channel4),
                    PortChannel::PortChannel2 => Ok(Channel::Channel5),
                    PortChannel::PortChannel3 => Ok(Channel::Channel6),
                },
                _ => Err(format!("Invalid port {:?}", port)),
            }
        }

        match command {
            VgmCommand::WriteYm2612(port, address, data) => match address {
                f if f == Lfo::BASE_ADDRESS => Ok(Opn2Command::Lfo(data.into())),
                f if f == TimerAMsb::BASE_ADDRESS => Ok(Opn2Command::TimerAMsb(data.into())),
                f if f == TimerALsb::BASE_ADDRESS => Ok(Opn2Command::TimerALsb(data.into())),
                f if f == TimerB::BASE_ADDRESS => Ok(Opn2Command::TimerB(data.into())),
                f if f == TimersAndChannel3Mode::BASE_ADDRESS => {
                    Ok(Opn2Command::TimersAndChannel3Mode(data.into()))
                }
                f if f == KeyOnOff::BASE_ADDRESS => Ok(Opn2Command::KeyOnOff(data.into())),
                f if f == DacEnable::BASE_ADDRESS => Ok(Opn2Command::DacEnable(data.into())),
                f if f == DacAmplitude::BASE_ADDRESS => Ok(Opn2Command::DacAmplitude(data.into())),
                f if (DetuneAndMultiple::BASE_ADDRESS..TotalLevel::BASE_ADDRESS).contains(&f) => {
                    let operator = operator::<DetuneAndMultiple>(f)?;
                    let port_channel = operator_port_channel::<DetuneAndMultiple>(f)?;
                    let channel = channel(port, port_channel)?;
                    Ok(Opn2Command::DetuneAndMultiple(
                        port.into(),
                        channel,
                        operator,
                        data.into(),
                    ))
                }
                f if (TotalLevel::BASE_ADDRESS..RateScalingAndAttackRate::BASE_ADDRESS)
                    .contains(&f) =>
                {
                    let operator = operator::<TotalLevel>(f)?;
                    let port_channel = operator_port_channel::<TotalLevel>(f)?;
                    let channel = channel(port, port_channel)?;
                    Ok(Opn2Command::TotalLevel(
                        port.into(),
                        channel,
                        operator,
                        data.into(),
                    ))
                }
                f if (RateScalingAndAttackRate::BASE_ADDRESS
                    ..AmplitudeAndFirstDecayRate::BASE_ADDRESS)
                    .contains(&f) =>
                {
                    let operator = operator::<RateScalingAndAttackRate>(f)?;
                    let port_channel = operator_port_channel::<RateScalingAndAttackRate>(f)?;
                    let channel = channel(port, port_channel)?;
                    Ok(Opn2Command::RateScalingAndAttackRate(
                        port.into(),
                        channel,
                        operator,
                        data.into(),
                    ))
                }
                f if (AmplitudeAndFirstDecayRate::BASE_ADDRESS..SecondDecayRate::BASE_ADDRESS)
                    .contains(&f) =>
                {
                    let operator = operator::<AmplitudeAndFirstDecayRate>(f)?;
                    let port_channel = operator_port_channel::<AmplitudeAndFirstDecayRate>(f)?;
                    let channel = channel(port, port_channel)?;
                    Ok(Opn2Command::AmplitudeAndFirstDecayRate(
                        port.into(),
                        channel,
                        operator,
                        data.into(),
                    ))
                }
                f if (SecondDecayRate::BASE_ADDRESS
                    ..SecondAmplitudeAndReleaseRate::BASE_ADDRESS)
                    .contains(&f) =>
                {
                    let operator = operator::<SecondDecayRate>(f)?;
                    let port_channel = operator_port_channel::<SecondDecayRate>(f)?;
                    let channel = channel(port, port_channel)?;
                    Ok(Opn2Command::SecondDecayRate(
                        port.into(),
                        channel,
                        operator,
                        data.into(),
                    ))
                }
                f if (SecondAmplitudeAndReleaseRate::BASE_ADDRESS
                    ..SoftwareSoundGeneratorMode::BASE_ADDRESS)
                    .contains(&f) =>
                {
                    let operator = operator::<SecondAmplitudeAndReleaseRate>(f)?;
                    let port_channel = operator_port_channel::<SecondAmplitudeAndReleaseRate>(f)?;
                    let channel = channel(port, port_channel)?;
                    Ok(Opn2Command::SecondAmplitudeAndReleaseRate(
                        port.into(),
                        channel,
                        operator,
                        data.into(),
                    ))
                }
                f if (SoftwareSoundGeneratorMode::BASE_ADDRESS..FrequencyLsb::BASE_ADDRESS)
                    .contains(&f) =>
                {
                    let operator = operator::<SoftwareSoundGeneratorMode>(f)?;
                    let port_channel = operator_port_channel::<SoftwareSoundGeneratorMode>(f)?;
                    let channel = channel(port, port_channel)?;
                    Ok(Opn2Command::SoftwareSoundGeneratorMode(
                        port.into(),
                        channel,
                        operator,
                        data.into(),
                    ))
                }
                f if (FrequencyLsb::BASE_ADDRESS..FrequencyMsb::BASE_ADDRESS).contains(&f) => {
                    let port_channel = port_channel::<FrequencyLsb>(f)?;
                    let channel = channel(port, port_channel)?;
                    Ok(Opn2Command::FrequencyLsb(port.into(), channel, data.into()))
                }
                f if (FrequencyMsb::BASE_ADDRESS
                    ..Channel3SupplementaryFrequencyMsb::BASE_ADDRESS)
                    .contains(&f) =>
                {
                    let port_channel = port_channel::<FrequencyMsb>(f)?;
                    let channel = channel(port, port_channel)?;
                    Ok(Opn2Command::FrequencyMsb(port.into(), channel, data.into()))
                }
                f if (Channel3SupplementaryFrequencyLsb::BASE_ADDRESS
                    ..Channel3SupplementaryFrequencyMsb::BASE_ADDRESS)
                    .contains(&f) =>
                {
                    let port_channel = port_channel::<Channel3SupplementaryFrequencyLsb>(f)?;
                    let channel = channel(port, port_channel)?;
                    Ok(Opn2Command::Channel3SupplementaryFrequencyLsb(
                        port.into(),
                        channel,
                        data.into(),
                    ))
                }
                f if (Channel3SupplementaryFrequencyMsb::BASE_ADDRESS
                    ..FeedbackAndAlgorithm::BASE_ADDRESS)
                    .contains(&f) =>
                {
                    let port_channel = port_channel::<Channel3SupplementaryFrequencyMsb>(f)?;
                    let channel = channel(port, port_channel)?;
                    Ok(Opn2Command::Channel3SupplementaryFrequencyMsb(
                        port.into(),
                        channel,
                        data.into(),
                    ))
                }
                f if (FeedbackAndAlgorithm::BASE_ADDRESS
                    ..StereoAndLfoSensitivity::BASE_ADDRESS)
                    .contains(&f) =>
                {
                    let port_channel = port_channel::<FeedbackAndAlgorithm>(f)?;
                    let channel = channel(port, port_channel)?;
                    Ok(Opn2Command::FeedbackAndAlgorithm(
                        port.into(),
                        channel,
                        data.into(),
                    ))
                }
                f if f >= StereoAndLfoSensitivity::BASE_ADDRESS => {
                    let port_channel = port_channel::<StereoAndLfoSensitivity>(f)?;
                    let channel = channel(port, port_channel)?;
                    Ok(Opn2Command::StereoAndLfoSensitivity(
                        port.into(),
                        channel,
                        data.into(),
                    ))
                }
                _ => panic!("Unhandled address 0x{:02x}", address),
            },
            VgmCommand::Wait(samples) => Ok(Opn2Command::Wait(samples.into())),
            c => Err(format!("Failed to convert {:?}", c)),
        }
    }
}

impl From<VgmFile> for Vec<Opn2Command> {
    fn from(vgm: VgmFile) -> Self {
        vgm.into_iter().flat_map(TryInto::try_into).collect()
    }
}

impl From<VgmFile> for Vec<Opn2Instruction> {
    fn from(vgm: VgmFile) -> Self {
        let commands: Vec<Opn2Command> = vgm.into();
        commands
            .into_iter()
            .flat_map(|command| {
                let raw_commands: Vec<Opn2Instruction> = command.into();
                raw_commands
            })
            .collect()
    }
}
