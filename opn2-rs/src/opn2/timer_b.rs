use super::Registers;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimerB {
    pub cnt: u16,
    pub subcnt: u8,
    pub reg: u16,
    pub load_lock: bool,
    pub load: bool,
    pub enable: bool,
    pub reset: bool,
    pub load_latch: bool,
    pub overflow_flag: bool,
    pub overflow: bool,
}

impl TimerB {
    pub fn clock(&mut self, cycles: u32, registers: &Registers) {
        let mut time: u16;
        let mut load = self.overflow;

        if cycles == 2 {
            // Lock load value
            load |= !self.load_lock && self.load;
            self.load_lock = self.load;
        }

        // Load counter
        time = if self.load_latch { self.reg } else { self.cnt };

        self.load_latch = load;

        // Increase counter
        if cycles == 1 {
            self.subcnt = self.subcnt.wrapping_add(1)
        }

        if self.subcnt == 0x10 && self.load_lock || registers.mode_test_21[2] != 0 {
            time = time.wrapping_add(1)
        }

        self.subcnt &= 0xf;

        // Set overflow flag
        if self.reset {
            self.reset = false;
            self.overflow_flag = false;
        } else {
            self.overflow_flag |= self.overflow & self.enable;
        }

        self.overflow = time >> 8 != 0;
        self.cnt = time & 0xff;
    }
}
