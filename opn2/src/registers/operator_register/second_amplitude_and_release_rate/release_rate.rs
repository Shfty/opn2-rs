#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ReleaseRate(u8);

impl ReleaseRate {
    pub fn new(rr: u8) -> Self {
        From::from(rr)
    }

    pub fn get(&self) -> u8 {
        (*self).into()
    }
}

impl From<u8> for ReleaseRate {
    fn from(rr: u8) -> Self {
        ReleaseRate(rr)
    }
}

impl From<ReleaseRate> for u8 {
    fn from(val: ReleaseRate) -> Self {
        val.0
    }
}
