mod timer_a_lsb;
mod timer_a_msb;

pub use timer_a_lsb::*;
pub use timer_a_msb::*;

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct TimerA {
    msb: TimerAMsb,
    lsb: TimerALsb,
}

#[allow(dead_code)]
impl TimerA {
    pub fn new(msb: TimerAMsb, lsb: TimerALsb) -> Self {
        TimerA { msb, lsb }
    }
}

// Getters
#[allow(dead_code)]
impl TimerA {
    pub fn get_msb(&self) -> TimerAMsb {
        self.msb
    }

    pub fn get_lsb(&self) -> TimerALsb {
        self.lsb
    }
}

// Setters
impl TimerA {
    pub fn set_msb<T>(&mut self, msb: T)
    where
        T: Into<TimerAMsb>,
    {
        self.msb = msb.into();
    }

    pub fn set_lsb<T>(&mut self, lsb: T)
    where
        T: Into<TimerALsb>,
    {
        self.lsb = lsb.into();
    }
}

// Application
/*
use super::GlobalRegister;

#[allow(dead_code)]
impl TimerA {
    pub fn apply(&self, opn2_sender: &OPN2CommandSender) -> Result<(), Opn2CommandError> {
        self.msb.apply(opn2_sender)?;
        self.lsb.apply(opn2_sender)?;
        Ok(())
    }
}
*/
