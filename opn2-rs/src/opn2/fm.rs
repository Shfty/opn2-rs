use crate::rom::{EXP_ROM, FM_ALGORITHM, LOGSIN_ROM};

use super::{EnvelopeGenerator, PhaseGenerator, Registers};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fm {
    pub op1: [[i16; 2]; 6],
    pub op2: [i16; 6],
    pub out: [i16; 24],
    pub modulation: [u16; 24],
}

impl Fm {
    pub fn prepare(&mut self, cycles: usize, channel: usize, registers: &Registers) {
        let slot = cycles.wrapping_add(6) % 24;

        let op = slot / 6;
        let connect = registers.connect[channel] as usize;
        let prevslot = cycles.wrapping_add(18) % 24;

        // Calculate modulation
        let mut mod2 = 0;
        let mut mod1 = mod2;

        if FM_ALGORITHM[op as usize][0][connect] != 0 {
            mod2 |= self.op1[channel][0];
        }

        if FM_ALGORITHM[op as usize][1][connect] != 0 {
            mod1 |= self.op1[channel][1];
        }

        if FM_ALGORITHM[op as usize][2][connect] != 0 {
            mod1 |= self.op2[channel];
        }

        if FM_ALGORITHM[op as usize][3][connect] != 0 {
            mod2 |= self.out[prevslot];
        }

        if FM_ALGORITHM[op as usize][4][connect] != 0 {
            mod1 |= self.out[prevslot];
        }

        let mut mod_0 = mod1 + mod2;

        if op == 0 {
            // Feedback
            mod_0 >>= 10 - registers.feedback[channel] as i16;
            if registers.feedback[channel] == 0 {
                mod_0 = 0;
            }
        } else {
            mod_0 >>= 1;
        }

        self.modulation[slot] = mod_0 as u16;

        let slot = (cycles.wrapping_add(18) % 24) as usize;

        // OP1
        if slot / 6 == 0 {
            self.op1[channel][1] = self.op1[channel][0];
            self.op1[channel][0] = self.out[slot]
        }

        // OP2
        if slot / 6 == 2 {
            self.op2[channel] = self.out[slot]
        };
    }

    pub fn generate(
        &mut self,
        cycles: usize,
        registers: &Registers,
        phase_generator: &PhaseGenerator,
        envelope_generator: &EnvelopeGenerator,
    ) {
        let slot = cycles.wrapping_add(19) % 24;

        // Calculate phase
        let phase =
            (self.modulation[slot] as u32).wrapping_add(phase_generator.phase[slot] >> 10) & 0x3ff;

        let quarter = if phase & 0x100 != 0 {
            (phase ^ 0xff) & 0xff
        } else {
            phase & 0xff
        } as usize;

        let mut level = LOGSIN_ROM[quarter];

        // Apply envelope
        level += (envelope_generator.out[slot] as u16) << 2;

        // Transform
        if level > 0x1fff {
            level = 0x1fff;
        }

        let mut output =
            ((EXP_ROM[(level & 0xff ^ 0xff) as usize] as i32 | 0x400) << 2 >> (level >> 8)) as i16;

        output = if phase & 0x200 != 0 {
            (!(output) ^ (registers.mode_test_21[4] as i16) << 13) + 1
        } else {
            output ^ (registers.mode_test_21[4] as i16) << 13
        };

        output <<= 2;
        output >>= 2;

        self.out[slot] = output;
    }
}
