mod command;

pub use command::*;

use std::ops::{Deref, DerefMut};

#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Commands(Vec<VgmCommand>);

impl Deref for Commands {
    type Target = Vec<VgmCommand>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Commands {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Commands> for Vec<VgmCommand> {
    fn from(val: Commands) -> Self {
        val.0
    }
}

impl IntoIterator for Commands {
    type Item = VgmCommand;

    type IntoIter = std::vec::IntoIter<VgmCommand>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl From<&[u8]> for Commands {
    fn from(bytes: &[u8]) -> Self {
        let mut vgm = bytes.iter();

        let mut commands: Vec<VgmCommand> = vec![];
        while let Some(command) = vgm.next() {
            commands.push(match command {
                0x4F => {
                    VgmCommand::GameGearPsgStereo(*vgm.next().expect("No game gear stereo data"))
                }
                0x50 => VgmCommand::WritePsg(*vgm.next().expect("No data")),
                0x52 => VgmCommand::WriteYm2612(
                    4000,
                    *vgm.next().expect("No address"),
                    *vgm.next().expect("No data"),
                ),
                0x53 => VgmCommand::WriteYm2612(
                    4002,
                    *vgm.next().expect("No address"),
                    *vgm.next().expect("No data"),
                ),
                0x61 => {
                    let b0 = *vgm.next().expect("No wait byte") as u16;
                    let b1 = *vgm.next().expect("No wait byte") as u16;
                    let samples = (b1 << 8) | b0;
                    VgmCommand::Wait(samples.into())
                }
                0x62 => VgmCommand::Wait(735.into()),
                0x63 => VgmCommand::Wait(882.into()),
                0x66 => break,
                0x70 => VgmCommand::Wait(1.into()),
                0x71 => VgmCommand::Wait(2.into()),
                0x72 => VgmCommand::Wait(3.into()),
                0x73 => VgmCommand::Wait(4.into()),
                0x74 => VgmCommand::Wait(5.into()),
                0x75 => VgmCommand::Wait(6.into()),
                0x76 => VgmCommand::Wait(7.into()),
                0x77 => VgmCommand::Wait(8.into()),
                0x78 => VgmCommand::Wait(9.into()),
                0x79 => VgmCommand::Wait(10.into()),
                0x7A => VgmCommand::Wait(11.into()),
                0x7B => VgmCommand::Wait(12.into()),
                0x7C => VgmCommand::Wait(13.into()),
                0x7D => VgmCommand::Wait(14.into()),
                0x7E => VgmCommand::Wait(15.into()),
                0x7F => VgmCommand::Wait(16.into()),
                command => panic!("Unrecognized VGM command 0x{:02x}", command),
            });
        }

        Commands(commands)
    }
}
