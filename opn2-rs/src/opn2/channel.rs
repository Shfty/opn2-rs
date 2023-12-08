use crate::rom::FM_ALGORITHM;

use super::{Fm, Registers};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Channel {
    pub acc: [i16; 6],
    pub out: [i16; 6],
    pub lock: i16,
    pub lock_l: u8,
    pub lock_r: u8,
    pub read: i16,
}

impl Channel {
    pub fn generate(&mut self, cycles: u32, channel: u32, registers: &Registers, fm: &Fm) {
        let slot = (cycles.wrapping_add(18) % 24) as usize;
        let channel = channel as usize;
        let op = slot / 6;
        let test_dac = registers.mode_test_2c[5] as u32;
        let mut acc = self.acc[channel];
        let mut add = test_dac as i16;

        if op == 0 && test_dac == 0 {
            acc = 0;
        }

        if FM_ALGORITHM[op][5][registers.connect[channel] as usize] != 0 && test_dac == 0 {
            add = (add as i32 + (fm.out[slot] as i32 >> 5)) as i16
        }

        let sum = (acc as i32 + add as i32) as i16;

        // Clamp
        let sum = if sum as i32 > 255 {
            255
        } else if (sum as i32) < -256 {
            -256
        } else {
            sum
        };

        if op == 0 || test_dac != 0 {
            self.out[channel] = self.acc[channel]
        }

        self.acc[channel] = sum;
    }
}
