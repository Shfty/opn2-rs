use std::ops::{Deref, DerefMut};

use crate::{Commands, Gd3, Header, VgmCommand};

#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct VgmFile {
    pub header: Header,
    pub gd3: Gd3,
    pub commands: Commands,
}

impl Deref for VgmFile {
    type Target = Commands;

    fn deref(&self) -> &Self::Target {
        &self.commands
    }
}

impl DerefMut for VgmFile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.commands
    }
}

impl IntoIterator for VgmFile {
    type Item = VgmCommand;

    type IntoIter = std::vec::IntoIter<VgmCommand>;

    fn into_iter(self) -> Self::IntoIter {
        self.commands.into_iter()
    }
}

impl VgmFile {
    pub fn parse(path: &str) -> Result<Self, std::io::Error> {
        let vgm: Vec<u8> = std::fs::read(path)?;
        
        let header: Header = From::from(&vgm[0..256]);
        let gd3: Gd3 = From::from(&vgm[0x14 + header.gd3_offset as usize..]);
        let commands: Commands = From::from(&vgm[0x34 + header.vgm_data_offset as usize..]);

        Ok(VgmFile {
            header,
            gd3,
            commands
        })
    }
}
