#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct FirstDecayRate(u8);

impl FirstDecayRate {
    pub fn new(d1r: u8) -> Self {
        From::from(d1r)
    }

    pub fn get(&self) -> u8 {
        (*self).into()
    }
}

impl From<u8> for FirstDecayRate {
    fn from(d1r: u8) -> Self {
        FirstDecayRate(d1r)
    }
}

impl From<FirstDecayRate> for u8 {
    fn from(val: FirstDecayRate) -> Self {
        val.0
    }
}
