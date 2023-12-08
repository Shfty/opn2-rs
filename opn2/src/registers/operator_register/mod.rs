mod amplitude_and_first_decay_rate;
mod detune_and_multiple;
mod rate_scaling_and_attack_rate;
mod second_amplitude_and_release_rate;
mod second_decay_rate;
mod software_sound_generator;
mod total_level;

pub use amplitude_and_first_decay_rate::*;
pub use detune_and_multiple::*;
pub use rate_scaling_and_attack_rate::*;
pub use second_amplitude_and_release_rate::*;
pub use second_decay_rate::*;
pub use software_sound_generator::*;
pub use total_level::*;

use super::{Channel, Port, PortChannel, Register};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Operator {
    Operator1,
    Operator2,
    Operator3,
    Operator4,
}

impl From<u8> for Operator {
    fn from(channel: u8) -> Self {
        match channel {
            0 => Operator::Operator1,
            1 => Operator::Operator2,
            2 => Operator::Operator3,
            3 => Operator::Operator4,
            _ => panic!("Invalid operator"),
        }
    }
}

/// Represents a 3 channel * 4 operator set of registers
pub trait OperatorRegister: Register {
    const OPERATOR_1_OFFSET: u8 = 0x0;
    const OPERATOR_2_OFFSET: u8 = 0x4;
    const OPERATOR_3_OFFSET: u8 = 0x8;
    const OPERATOR_4_OFFSET: u8 = 0xC;

    const CHANNEL_1_OFFSET: u8 = 0x0;
    const CHANNEL_2_OFFSET: u8 = 0x1;
    const CHANNEL_3_OFFSET: u8 = 0x2;

    const OPERATOR_1_CHANNEL_1: u8 =
        Self::BASE_ADDRESS + Self::OPERATOR_1_OFFSET + Self::CHANNEL_1_OFFSET;
    const OPERATOR_1_CHANNEL_2: u8 =
        Self::BASE_ADDRESS + Self::OPERATOR_1_OFFSET + Self::CHANNEL_2_OFFSET;
    const OPERATOR_1_CHANNEL_3: u8 =
        Self::BASE_ADDRESS + Self::OPERATOR_1_OFFSET + Self::CHANNEL_3_OFFSET;

    const OPERATOR_2_CHANNEL_1: u8 =
        Self::BASE_ADDRESS + Self::OPERATOR_2_OFFSET + Self::CHANNEL_1_OFFSET;
    const OPERATOR_2_CHANNEL_2: u8 =
        Self::BASE_ADDRESS + Self::OPERATOR_2_OFFSET + Self::CHANNEL_2_OFFSET;
    const OPERATOR_2_CHANNEL_3: u8 =
        Self::BASE_ADDRESS + Self::OPERATOR_2_OFFSET + Self::CHANNEL_3_OFFSET;

    const OPERATOR_3_CHANNEL_1: u8 =
        Self::BASE_ADDRESS + Self::OPERATOR_3_OFFSET + Self::CHANNEL_1_OFFSET;
    const OPERATOR_3_CHANNEL_2: u8 =
        Self::BASE_ADDRESS + Self::OPERATOR_3_OFFSET + Self::CHANNEL_2_OFFSET;
    const OPERATOR_3_CHANNEL_3: u8 =
        Self::BASE_ADDRESS + Self::OPERATOR_3_OFFSET + Self::CHANNEL_3_OFFSET;

    const OPERATOR_4_CHANNEL_1: u8 =
        Self::BASE_ADDRESS + Self::OPERATOR_4_OFFSET + Self::CHANNEL_1_OFFSET;
    const OPERATOR_4_CHANNEL_2: u8 =
        Self::BASE_ADDRESS + Self::OPERATOR_4_OFFSET + Self::CHANNEL_2_OFFSET;
    const OPERATOR_4_CHANNEL_3: u8 =
        Self::BASE_ADDRESS + Self::OPERATOR_4_OFFSET + Self::CHANNEL_3_OFFSET;

    fn port_of(channel: Channel) -> Port {
        match channel {
            Channel::Channel1 => Port::Port1,
            Channel::Channel2 => Port::Port1,
            Channel::Channel3 => Port::Port1,
            Channel::Channel4 => Port::Port2,
            Channel::Channel5 => Port::Port2,
            Channel::Channel6 => Port::Port2,
        }
    }

    fn address_of(channel: Channel, operator: Operator) -> u8 {
        let port_channel: PortChannel = channel.into();

        match port_channel {
            PortChannel::PortChannel1 => match operator {
                Operator::Operator1 => Self::OPERATOR_1_CHANNEL_1,
                Operator::Operator2 => Self::OPERATOR_2_CHANNEL_1,
                Operator::Operator3 => Self::OPERATOR_3_CHANNEL_1,
                Operator::Operator4 => Self::OPERATOR_4_CHANNEL_1,
            },
            PortChannel::PortChannel2 => match operator {
                Operator::Operator1 => Self::OPERATOR_1_CHANNEL_2,
                Operator::Operator2 => Self::OPERATOR_2_CHANNEL_2,
                Operator::Operator3 => Self::OPERATOR_3_CHANNEL_2,
                Operator::Operator4 => Self::OPERATOR_4_CHANNEL_2,
            },
            PortChannel::PortChannel3 => match operator {
                Operator::Operator1 => Self::OPERATOR_1_CHANNEL_3,
                Operator::Operator2 => Self::OPERATOR_2_CHANNEL_3,
                Operator::Operator3 => Self::OPERATOR_3_CHANNEL_3,
                Operator::Operator4 => Self::OPERATOR_4_CHANNEL_3,
            },
        }
    }

    fn get_data(&self) -> u8;

    /*
    fn apply(
        &self,
        opn2_sender: &OPN2CommandSender,
        channel: Channel,
        operator: Operator,
    ) -> Result<(), Opn2CommandError> {
        opn2_sender.send(
            Self::port_of(channel).into(),
            Self::address_of(channel, operator),
            self.get_data(),
        )?;

        Ok(())
    }
    */
}
