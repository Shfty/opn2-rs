use super::{Channel, Port, PortChannel, Register};

mod channel_3_supplementary_frequency;
mod feedback_and_algorithm;
mod frequency;
mod stereo_and_lfo_sensitivity;

pub use channel_3_supplementary_frequency::*;
pub use feedback_and_algorithm::{Algorithm, Feedback, FeedbackAndAlgorithm};
pub use frequency::*;
pub use stereo_and_lfo_sensitivity::*;

/// Represents a 3-channel set of registers
pub trait ChannelRegister: Register {
    const CHANNEL_1_OFFSET: u8 = 0x0;
    const CHANNEL_2_OFFSET: u8 = 0x1;
    const CHANNEL_3_OFFSET: u8 = 0x2;

    const CHANNEL_1: u8 = Self::BASE_ADDRESS + Self::CHANNEL_1_OFFSET;
    const CHANNEL_2: u8 = Self::BASE_ADDRESS + Self::CHANNEL_2_OFFSET;
    const CHANNEL_3: u8 = Self::BASE_ADDRESS + Self::CHANNEL_3_OFFSET;

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

    fn address_of(channel: Channel) -> u8 {
        let port_channel: PortChannel = channel.into();

        match port_channel {
            PortChannel::PortChannel1 => Self::CHANNEL_1,
            PortChannel::PortChannel2 => Self::CHANNEL_2,
            PortChannel::PortChannel3 => Self::CHANNEL_3,
        }
    }

    fn get_data(&self) -> u8;

    /*
    fn apply(&self, opn2_sender: &OPN2CommandSender, channel: Channel) -> Result<(), Opn2CommandError> {
        opn2_sender.send(
            Self::port_of(channel).into(),
            Self::address_of(channel),
            self.get_data(),
        )?;

        Ok(())
    }
    */
}
