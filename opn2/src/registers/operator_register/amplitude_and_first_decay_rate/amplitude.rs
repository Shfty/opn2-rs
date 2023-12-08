#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Amplitude(u8);

impl Amplitude {
    pub fn new(am: u8) -> Self {
        From::from(am)
    }

    pub fn get(&self) -> u8 {
        (*self).into()
    }
}

impl From<u8> for Amplitude {
    fn from(am: u8) -> Self {
        Amplitude(am)
    }
}

impl From<Amplitude> for u8 {
    fn from(val: Amplitude) -> Self {
        val.0
    }
}
