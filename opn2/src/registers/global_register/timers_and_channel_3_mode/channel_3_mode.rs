#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[allow(dead_code)]
pub enum Channel3Mode {
    /// Normal channel behavior
    Normal,
    /// 4 separate frequencies for channel 3/6
    Special,
    /// Special mode + automatic key on/off for channel 3 based on Timer A
    SpecialCsm,
}

impl Default for Channel3Mode {
    fn default() -> Self {
        Channel3Mode::Normal
    }
}

impl From<u8> for Channel3Mode {
    fn from(mode: u8) -> Self {
        match mode {
            0 => Channel3Mode::Normal,
            1 => Channel3Mode::Special,
            2 => Channel3Mode::SpecialCsm,
            _ => panic!("Invalid channel 3 mode"),
        }
    }
}

impl From<Channel3Mode> for u8 {
    fn from(val: Channel3Mode) -> Self {
        match val {
            Channel3Mode::Normal => 0,
            Channel3Mode::Special => 1,
            Channel3Mode::SpecialCsm => 2,
        }
    }
}
