#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum RateScaling {
    RateScaling1,
    RateScaling2,
    RateScaling3,
    RateScaling4,
}

impl Default for RateScaling {
    fn default() -> Self {
        RateScaling::RateScaling1
    }
}

impl From<u8> for RateScaling {
    fn from(rs: u8) -> Self {
        match rs {
            0 => RateScaling::RateScaling1,
            1 => RateScaling::RateScaling2,
            2 => RateScaling::RateScaling3,
            3 => RateScaling::RateScaling4,
            _ => panic!("Rate Scaling should be a 2-bit number"),
        }
    }
}

impl From<RateScaling> for u8 {
    fn from(val: RateScaling) -> Self {
        match val {
            RateScaling::RateScaling1 => 0,
            RateScaling::RateScaling2 => 1,
            RateScaling::RateScaling3 => 2,
            RateScaling::RateScaling4 => 3,
        }
    }
}
