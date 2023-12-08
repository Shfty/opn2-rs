use super::Registers;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimerA {
    pub cnt: u16,
    pub reg: u16,
    pub load_lock: bool,
    pub load: bool,
    pub enable: bool,
    pub reset: bool,
    pub load_latch: bool,
    pub overflow_flag: bool,
    pub overflow: bool,
}

impl TimerA {
    pub fn clock(&mut self, cycles: u32, registers: &mut Registers) {
        let mut time: u16;
        let mut load = self.overflow;

        if cycles == 2 {
            // Lock load value
            load |= !self.load_lock && self.load;
            self.load_lock = self.load;

            registers.mode_kon_csm = if registers.mode_csm != 0 {
                // CSM KeyOn
                load as u8
            } else {
                0
            };
        }

        // Load counter
        time = if self.load_latch {
            self.reg
        } else {
            self.cnt
        };

        self.load_latch = load;

        // Increase counter
        if cycles == 1 && self.load_lock || registers.mode_test_21[2] != 0 {
            time = time.wrapping_add(1)
        }

        // Set overflow flag
        if self.reset {
            self.reset = false;
            self.overflow_flag = false;
        } else {
            self.overflow_flag |= self.overflow & self.enable;
        }

        self.overflow = time >> 10 != 0;
        self.cnt = time & 0x3ff;
    }
}
