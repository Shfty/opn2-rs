use crate::rom::LFO_CYCLES;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Lfo {
    pub enable: u8,
    pub freq: usize,
    pub pm: u8,
    pub am: u8,
    pub cnt: u8,
    pub inc: u8,
    pub quotient: u8,
}

impl Lfo {
    pub fn cycle_0(&mut self) {
        self.pm = (self.cnt >> 2) as u8;

        if self.cnt & 0x40 != 0 {
            self.am = (self.cnt & 0x3f) as u8
        } else {
            self.am = (self.cnt ^ 0x3f) as u8
        }

        self.am = ((self.am) << 1) as u8
    }

    pub fn cycle_23(&mut self) {
        self.inc = (self.inc | 1) as u8;
    }

    pub fn update(&mut self) {
        if self.quotient as u32 & LFO_CYCLES[self.freq] == LFO_CYCLES[self.freq] {
            self.quotient = 0;
            self.cnt = self.cnt.wrapping_add(1)
        } else {
            self.quotient = (self.quotient + self.inc) as u8
        }

        self.cnt = (self.cnt & self.enable) as u8;
    }
}
