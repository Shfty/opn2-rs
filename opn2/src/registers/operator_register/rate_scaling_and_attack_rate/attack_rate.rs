#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct AttackRate(u8);

impl AttackRate {
    pub fn new(ar: u8) -> Self {
        From::from(ar)
    }

    pub fn get(&self) -> u8 {
        (*self).into()
    }
}

impl From<u8> for AttackRate {
    fn from(ar: u8) -> Self {
        AttackRate(ar)
    }
}

impl From<AttackRate> for u8 {
    fn from(val: AttackRate) -> Self {
        val.0
    }
}
