use crate::rom::{EG_AM_SHIFT, EG_STEP_HI};

use super::{Adsr, Io, Lfo, PhaseGenerator, Registers};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EnvelopeGenerator {
    pub cycle: u8,
    pub cycle_stop: u8,
    pub shift: u8,
    pub shift_lock: u8,
    pub timer_low_lock: u8,
    pub timer: u16,
    pub timer_inc: u8,
    pub quotient: u16,
    pub custom_timer: u8,
    pub rate: u8,
    pub ksv: u8,
    pub inc: u8,
    pub ratemax: u8,
    pub sl: [u8; 2],
    pub lfo_am: u8,
    pub tl: [u8; 2],
    pub state: [u8; 24],
    pub level: [u16; 24],
    pub out: [u16; 24],
    pub kon: [u8; 24],
    pub kon_csm: [u8; 24],
    pub kon_latch: [u8; 24],
    pub csm_mode: [u8; 24],
    pub ssg_enable: [u8; 24],
    pub ssg_pgrst_latch: [u8; 24],
    pub ssg_repeat_latch: [u8; 24],
    pub ssg_hold_up_latch: [u8; 24],
    pub ssg_dir: [u8; 24],
    pub ssg_inv: [u8; 24],
    pub read: [u32; 2],
    pub read_inc: u8,
}

impl Default for EnvelopeGenerator {
    fn default() -> Self {
        EnvelopeGenerator {
            cycle: 0,
            cycle_stop: 0,
            shift: 0,
            shift_lock: 0,
            timer_low_lock: 0,
            timer: 0,
            timer_inc: 0,
            quotient: 0,
            custom_timer: 0,
            rate: 0,
            ksv: 0,
            inc: 0,
            ratemax: 0,
            sl: [0; 2],
            lfo_am: 0,
            tl: [0; 2],
            state: [Adsr::Release as u8; 24],
            level: [0x3ff; 24],
            out: [0x3ff; 24],
            kon: [0; 24],
            kon_csm: [0; 24],
            kon_latch: [0; 24],
            csm_mode: [0; 24],
            ssg_enable: [0; 24],
            ssg_pgrst_latch: [0; 24],
            ssg_repeat_latch: [0; 24],
            ssg_hold_up_latch: [0; 24],
            ssg_dir: [0; 24],
            ssg_inv: [0; 24],
            read: [0; 2],
            read_inc: 0,
        }
    }
}

impl EnvelopeGenerator {
    pub fn cycle_1(&mut self) {
        // Lock envelope generator timer value
        if self.quotient == 2 {
            self.shift_lock = if self.cycle_stop != 0 {
                0
            } else {
                (self.shift + 1) as u8
            };

            self.timer_low_lock = (self.timer & 0x3) as u8
        }

        self.quotient = self.quotient.wrapping_add(1);
        self.quotient = (self.quotient % 3) as u16;

        self.cycle = 0;
        self.cycle_stop = 1;
        self.shift = 0;

        self.timer_inc = (self.timer_inc as u16 | self.quotient >> 1) as u8;
        self.timer += self.timer_inc as u16;
        self.timer_inc = (self.timer >> 12) as u8;
        self.timer = (self.timer & 0xfff) as u16
    }

    pub fn cycle_2(&mut self) {
        self.read[1] = self.out[0] as u32;
    }

    pub fn cycle_13(&mut self) {
        self.cycle = 0;
        self.cycle_stop = 1;
        self.shift = 0;
        self.timer += self.timer_inc as u16;
        self.timer_inc = (self.timer >> 12) as u8;
        self.timer = (self.timer & 0xfff) as u16
    }

    pub fn increment_timer(&mut self, registers: &Registers, io: &Io) {
        self.timer &= !((registers.mode_test_21[5] as u16) << self.cycle as u16);

        #[allow(clippy::suspicious_operation_groupings)]
        if (self.timer >> self.cycle as u16 | io.pin_test_in as u16 & self.custom_timer as u16)
            & self.cycle_stop as u16
            != 0
        {
            self.shift = self.cycle;
            self.cycle_stop = 0;
        }
    }

    pub fn prepare(
        &mut self,
        cycles: u32,
        channel: usize,
        registers: &Registers,
        phase_generator: &PhaseGenerator,
        lfo: &Lfo,
    ) {
        let mut inc = 0;
        let slot = cycles as usize;
        let mut rate_sel: u8;

        // Prepare increment
        let mut rate = (((self.rate) << 1) + self.ksv) as u8;
        if rate > 0x3f {
            rate = 0x3f;
        }

        let sum = (((rate >> 2) + self.shift_lock) & 0xf) as u8;
        if self.rate != 0 && self.quotient == 2 {
            if (rate) < 48 {
                match sum {
                    12 => inc = 1,
                    13 => inc = (rate >> 1 & 0x1) as u8,
                    14 => inc = (rate & 0x1) as u8,
                    _ => {}
                }
            } else {
                inc = EG_STEP_HI[(rate & 0x3) as usize][self.timer_low_lock as usize]
                    .wrapping_add((rate >> 2) as u32)
                    .wrapping_sub(11) as u8;
                if inc > 4 {
                    inc = 4;
                }
            }
        }

        self.inc = inc;
        self.ratemax = (rate >> 1 == 0x1f) as u8;

        // Prepare rate & ksv
        rate_sel = self.state[slot];
        if self.kon[slot] != 0 && self.ssg_repeat_latch[slot] != 0
            || self.kon[slot] == 0 && self.kon_latch[slot] != 0
        {
            rate_sel = Adsr::Attack as u8
        }

        match rate_sel {
            0 => self.rate = registers.attack_rate[slot],
            1 => self.rate = registers.decay_rate_first[slot],
            2 => self.rate = registers.decay_rate_second[slot],
            3 => self.rate = ((registers.release_rate[slot]) << 1 | 0x1) as u8,
            _ => {}
        }

        self.ksv = (phase_generator.kcode >> (registers.rate_scale[slot] ^ 0x3)) as u8;

        if registers.amplitude_modulation[slot] != 0 {
            self.lfo_am = (lfo.am >> EG_AM_SHIFT[registers.amp_mod_sens[channel] as usize]) as u8
        } else {
            self.lfo_am = 0;
        }

        // Delay TL & SL value
        self.tl[1] = self.tl[0];
        self.tl[0] = registers.total_level[slot];
        self.sl[1] = self.sl[0];
        self.sl[0] = registers.secondary_amplitude[slot];
    }

    pub fn generate(&mut self, cycles: usize, channel: usize, registers: &Registers) {
        let slot = cycles.wrapping_add(23) % 24;
        let mut level = self.level[slot];

        if self.ssg_inv[slot] != 0 {
            // Inverse
            level = 512 - level;
        }

        if registers.mode_test_21[5] != 0 {
            level = 0;
        }

        level &= 0x3ff;

        // Apply AM LFO
        level += self.lfo_am as u16;

        // Apply TL
        if !(registers.mode_csm != 0 && channel == 3) {
            level += (self.tl[0] as u16) << 3;
        }

        if level > 0x3ff {
            level = 0x3ff;
        }

        self.out[slot] = level;
    }

    pub fn adsr(&mut self, cycles: u32, phase_generator: &mut PhaseGenerator) {
        let slot = (cycles.wrapping_add(22) % 24) as usize;
        let nkon = self.kon_latch[slot];
        let okon = self.kon[slot];
        let mut nextstate = self.state[slot];
        let mut inc = 0;
        self.read[0] = self.read_inc as u32;
        self.read_inc = (self.inc != 0) as u8;

        // Reset phase generator
        phase_generator.reset[slot] =
            (nkon != 0 && okon == 0 || self.ssg_pgrst_latch[slot] != 0) as u8;

        // KeyOn/Off
        let kon_event =
            (nkon != 0 && okon == 0 || okon != 0 && self.ssg_repeat_latch[slot] != 0) as u8;
        let koff_event = (okon != 0 && nkon == 0) as u8;
        let mut level = self.level[slot] as i16;
        let mut ssg_level = level;

        if self.ssg_inv[slot] != 0 {
            // Inverse
            ssg_level = 512 - level;
            ssg_level &= 0x3ff;
        }

        if koff_event != 0 {
            level = ssg_level
        }

        let eg_off = if self.ssg_enable[slot] != 0 {
            (level >> 9) as u8
        } else {
            (level & 0x3f0 == 0x3f0) as u8
        };

        let mut nextlevel = level;
        if kon_event != 0 {
            nextstate = Adsr::Attack as u8;
            // Instant attack
            if self.ratemax != 0 {
                nextlevel = 0;
            } else if self.state[slot] == Adsr::Attack as u8
                && level != 0
                && self.inc != 0
                && nkon != 0
            {
                inc = !(level) << self.inc >> 5;
            }
        } else {
            match self.state[slot] {
                0 => {
                    if level == 0 {
                        nextstate = Adsr::Decay as u8
                    } else if self.inc != 0 && self.ratemax == 0 && nkon != 0 {
                        inc = ((!(level)) << self.inc >> 5) as i16
                    }
                }
                1 => {
                    if level >> 5 == self.sl[1] as i16 {
                        nextstate = Adsr::Sustain as u8
                    } else if eg_off == 0 && self.inc != 0 {
                        inc = (1 << (self.inc - 1)) as i16;
                        if self.ssg_enable[slot] != 0 {
                            inc <<= 2;
                        }
                    }
                }
                2 | 3 => {
                    if eg_off == 0 && self.inc != 0 {
                        inc = (1 << (self.inc - 1)) as i16;
                        if self.ssg_enable[slot] != 0 {
                            inc <<= 2;
                        }
                    }
                }
                _ => {}
            }

            if nkon == 0 {
                nextstate = Adsr::Release as u8
            }
        }

        if self.kon_csm[slot] != 0 {
            nextlevel |= (self.tl[1] as i16) << 3;
        }

        // Envelope off
        if kon_event == 0
            && self.ssg_hold_up_latch[slot] == 0
            && self.state[slot] != Adsr::Attack as u8
            && eg_off != 0
        {
            nextstate = Adsr::Release as u8;
            nextlevel = 0x3ff;
        }

        nextlevel = (nextlevel + inc) as i16;
        self.kon[slot] = self.kon_latch[slot];
        self.level[slot] = (nextlevel & 0x3ff) as u16;
        self.state[slot] = nextstate;
    }

    pub fn ssg_eg(&mut self, cycles: u32, registers: &Registers) {
        let slot = cycles as usize;
        let mut direction = 0;
        self.ssg_pgrst_latch[slot] = 0;
        self.ssg_repeat_latch[slot] = 0;
        self.ssg_hold_up_latch[slot] = 0;
        self.ssg_inv[slot] = 0;

        if registers.ssg_eg[slot] & 0x8 != 0 {
            direction = self.ssg_dir[slot];
            if self.level[slot] & 0x200 != 0 {
                // Reset
                if registers.ssg_eg[slot] & 0x3 == 0 {
                    self.ssg_pgrst_latch[slot] = 1;
                }

                // Repeat
                if registers.ssg_eg[slot] & 0x1 == 0 {
                    self.ssg_repeat_latch[slot] = 1;
                }

                // Inverse
                if registers.ssg_eg[slot] & 0x3 == 0x2 {
                    direction ^= 1;
                }

                if registers.ssg_eg[slot] & 0x3 == 0x3 {
                    direction = 1;
                }
            }

            // Hold up
            if self.kon_latch[slot] != 0
                && (registers.ssg_eg[slot] & 0x7 == 0x5 || registers.ssg_eg[slot] & 0x7 == 0x3)
            {
                self.ssg_hold_up_latch[slot] = 1;
            }

            direction &= self.kon[slot];

            self.ssg_inv[slot] =
                (self.ssg_dir[slot] ^ registers.ssg_eg[slot] >> 2 & 0x1) & self.kon[slot]
        }

        self.ssg_dir[slot] = direction;
        self.ssg_enable[slot] = registers.ssg_eg[slot] >> 3 & 0x1;
    }

    pub fn key_on(&mut self, cycles: u32, channel: u32, slot: u32, registers: &mut Registers) {
        let slot = slot as usize;

        // Key On
        self.kon_latch[slot] = registers.mode_kon[slot];
        self.kon_csm[slot] = 0;

        if channel == 2 && registers.mode_kon_csm != 0 {
            // CSM Key On
            self.kon_latch[slot] = 1;
            self.kon_csm[slot] = 1;
        }

        if cycles == registers.mode_kon_channel as u32 {
            // OP1
            registers.mode_kon[channel as usize] = registers.mode_kon_operator[0];
            // OP2
            registers.mode_kon[channel.wrapping_add(12) as usize] = registers.mode_kon_operator[1];
            // OP3
            registers.mode_kon[channel.wrapping_add(6) as usize] = registers.mode_kon_operator[2];
            // OP4
            registers.mode_kon[channel.wrapping_add(18) as usize] = registers.mode_kon_operator[3]
        };
    }
}
