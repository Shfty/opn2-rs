mod attack_rate;
mod rate_scaling;

pub use attack_rate::*;
pub use rate_scaling::*;

use super::{OperatorRegister, Register};

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct RateScalingAndAttackRate {
    rate_scaling: RateScaling,
    attack_rate: AttackRate,
}

impl RateScalingAndAttackRate {
    pub fn new(rs: RateScaling, ar: AttackRate) -> Self {
        RateScalingAndAttackRate {
            rate_scaling: rs,
            attack_rate: ar,
        }
    }
}

// Getters
impl RateScalingAndAttackRate {
    pub fn get_rate_scaling(&self) -> RateScaling {
        self.rate_scaling
    }

    pub fn get_attack_rate(&self) -> AttackRate {
        self.attack_rate
    }
}

// Setters
impl RateScalingAndAttackRate {
    pub fn set_rate_scaling<T>(&mut self, rate_scaling: T)
    where
        T: Into<RateScaling>,
    {
        self.rate_scaling = rate_scaling.into();
    }

    pub fn set_attack_rate<T>(&mut self, attack_rate: T)
    where
        T: Into<AttackRate>,
    {
        self.attack_rate = attack_rate.into();
    }
}

impl From<u8> for RateScalingAndAttackRate {
    fn from(rs_and_ar: u8) -> Self {
        let rs = rs_and_ar >> 6;
        let ar = rs_and_ar & 0b11111;
        RateScalingAndAttackRate::new(rs.into(), AttackRate::new(ar))
    }
}

impl From<RateScalingAndAttackRate> for u8 {
    fn from(val: RateScalingAndAttackRate) -> Self {
        let rs: u8 = val.rate_scaling.into();
        (rs << 6) | val.attack_rate.get()
    }
}

impl Register for RateScalingAndAttackRate {
    const BASE_ADDRESS: u8 = 0x50;
}

impl OperatorRegister for RateScalingAndAttackRate {
    fn get_data(&self) -> u8 {
        (*self).into()
    }
}
