mod channel_register;
mod global_register;
mod operator_register;

pub use channel_register::*;
pub use global_register::*;
pub use operator_register::*;

/// Represents a single register inside the OPN2
pub trait Register {
    const BASE_ADDRESS: u8;
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Channel {
    Channel1,
    Channel2,
    Channel3,
    Channel4,
    Channel5,
    Channel6,
}

impl From<u8> for Channel {
    fn from(channel: u8) -> Self {
        match channel {
            0 => Channel::Channel1,
            1 => Channel::Channel2,
            2 => Channel::Channel3,
            4 => Channel::Channel4,
            5 => Channel::Channel5,
            6 => Channel::Channel6,
            _ => panic!("Invalid channel"),
        }
    }
}

impl From<Channel> for u8 {
    fn from(val: Channel) -> Self {
        match val {
            Channel::Channel1 => 0,
            Channel::Channel2 => 1,
            Channel::Channel3 => 2,
            Channel::Channel4 => 4,
            Channel::Channel5 => 5,
            Channel::Channel6 => 6,
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum PortChannel {
    PortChannel1,
    PortChannel2,
    PortChannel3,
}

impl From<Channel> for PortChannel {
    fn from(channel: Channel) -> Self {
        match channel {
            Channel::Channel1 => PortChannel::PortChannel1,
            Channel::Channel2 => PortChannel::PortChannel2,
            Channel::Channel3 => PortChannel::PortChannel3,
            Channel::Channel4 => PortChannel::PortChannel1,
            Channel::Channel5 => PortChannel::PortChannel2,
            Channel::Channel6 => PortChannel::PortChannel3,
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Port {
    Port1,
    Port2,
}

impl From<u32> for Port {
    fn from(port: u32) -> Self {
        match port {
            4000 => Port::Port1,
            4002 => Port::Port2,
            _ => panic!("Invalid port"),
        }
    }
}

impl From<Port> for u32 {
    fn from(val: Port) -> Self {
        match val {
            Port::Port1 => 4000,
            Port::Port2 => 4002,
        }
    }
}
