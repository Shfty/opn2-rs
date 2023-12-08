#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct SecondAmplitude(u8);

impl SecondAmplitude {
    pub fn new(d1l: u8) -> Self {
        From::from(d1l)
    }

    pub fn get(&self) -> u8 {
        (*self).into()
    }
}

impl From<u8> for SecondAmplitude {
    fn from(d1l: u8) -> Self {
        SecondAmplitude(d1l)
    }
}

impl From<SecondAmplitude> for u8 {
    fn from(val: SecondAmplitude) -> Self {
        val.0
    }
}
