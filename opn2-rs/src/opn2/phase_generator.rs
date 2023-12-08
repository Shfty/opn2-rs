use crate::rom::{PG_DETUNE, PG_LFO_SH1, PG_LFO_SH2};

use super::{Lfo, Registers};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhaseGenerator {
    pub fnum: u16,
    pub block: u8,
    pub kcode: u8,
    pub inc: [u32; 24],
    pub phase: [u32; 24],
    pub reset: [u8; 24],
    pub read: u32,
}

impl PhaseGenerator {
    pub fn cycle_2(&mut self) {
        self.read = self.phase[21] & 0x3ff;
    }

    pub fn generate(&mut self, cycles: usize, registers: &Registers) {
        // Mask increment
        let slot = cycles.wrapping_add(20) % 24;

        if self.reset[slot] != 0 {
            self.inc[slot] = 0
        }

        // Phase step
        let slot = cycles.wrapping_add(19) % 24;

        if self.reset[slot] != 0 || registers.mode_test_21[3] != 0 {
            self.phase[slot] = 0
        }

        self.phase[slot] = self.phase[slot].wrapping_add(self.inc[slot]);

        self.phase[slot] &= 0xfffff;
    }

    pub fn phase_calc_increment(
        &mut self,
        cycles: usize,
        channel: usize,
        registers: &Registers,
        lfo: &Lfo,
    ) {
        let chan = channel;
        let slot = cycles;

        let mut fnum = self.fnum as u32;
        let fnum_h = fnum >> 4;
        fnum <<= 1;

        let lfo = lfo.pm;
        let mut lfo_l = (lfo & 0xf) as usize;
        let pms = registers.freq_mod_sens[chan] as usize;
        let dt = registers.detune[slot];
        let dt_l = dt & 0x3;
        let mut detune = 0;
        let mut kcode = self.kcode;

        // Apply LFO
        if lfo_l & 0x8 != 0 {
            lfo_l ^= 0xf;
        }

        let mut fm =
            (fnum_h >> PG_LFO_SH1[pms][lfo_l]).wrapping_add(fnum_h >> PG_LFO_SH2[pms][lfo_l]);

        if pms > 5 {
            fm <<= pms as i32 - 5
        }

        fm >>= 2;

        if lfo & 0x10 != 0 {
            fnum = fnum.wrapping_sub(fm);
        } else {
            fnum = fnum.wrapping_add(fm);
        }

        fnum &= 0xfff;
        let mut basefreq = fnum << self.block >> 2;

        // Apply detune
        if dt_l != 0 {
            if kcode > 0x1c {
                kcode = 0x1c;
            }

            let block = kcode >> 2;
            let note = kcode & 0x3;
            let sum = block + 9 + ((dt_l == 3) as u8 | dt_l & 0x2);
            let sum_h = sum >> 1;
            let sum_l = sum & 0x1;
            detune = PG_DETUNE[((sum_l) << 2 | note) as usize] >> (9 - sum_h)
        }

        if dt & 0x4 != 0 {
            basefreq = (basefreq).wrapping_sub(detune)
        } else {
            basefreq = (basefreq).wrapping_add(detune)
        }

        basefreq &= 0x1ffff;
        self.inc[slot] = basefreq.wrapping_mul(registers.multiple[slot] as u32) >> 1;
        self.inc[slot] &= 0xfffff;
    }

    pub fn fnum_block(&mut self, slot: u32, channel: usize, registers: &Registers) {
        if registers.mode_ch3 != 0 {
            // Channel 3 special mode
            match slot {
                1 => {
                    // OP1
                    self.fnum = registers.fnum_ch3[1];
                    self.block = registers.block_ch3[1];
                    self.kcode = registers.kcode_ch3[1]
                }
                7 => {
                    // OP3
                    self.fnum = registers.fnum_ch3[0];
                    self.block = registers.block_ch3[0];
                    self.kcode = registers.kcode_ch3[0]
                }
                13 => {
                    // OP2
                    self.fnum = registers.fnum_ch3[2];
                    self.block = registers.block_ch3[2];
                    self.kcode = registers.kcode_ch3[2]
                }
                _ => {
                    // OP4
                    self.fnum = registers.fnum[channel.wrapping_add(1) % 6];
                    self.block = registers.block[channel.wrapping_add(1) % 6];
                    self.kcode = registers.kcode[channel.wrapping_add(1) % 6]
                }
            }
        } else {
            self.fnum = registers.fnum[channel.wrapping_add(1) % 6];
            self.block = registers.block[channel.wrapping_add(1) % 6];
            self.kcode = registers.kcode[channel.wrapping_add(1) % 6]
        }
    }
}
