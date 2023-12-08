pub mod traits;

mod address;
mod adsr;
mod channel;
mod envelope_generator;
mod fm;
mod io;
mod lfo;
mod phase_generator;
mod registers;
mod timer_a;
mod timer_b;

pub use address::*;
pub use adsr::*;
pub use channel::*;
pub use envelope_generator::*;
pub use fm::*;
pub use io::*;
pub use lfo::*;
pub use phase_generator::*;
pub use registers::*;
pub use timer_a::*;
pub use timer_b::*;

use crate::rom::{CH_OFFSET, FN_NOTE, OP_OFFSET};

use traits::Chip;

use opn2_trait::Opn2Trait;

use std::marker::PhantomData;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Opn2<C>
where
    C: Chip,
{
    pub cycles: u32,
    pub channel: u32,
    pub mol: i16,
    pub mor: i16,

    pub io: Io,
    pub lfo: Lfo,
    pub phase_generator: PhaseGenerator,
    pub envelope_generator: EnvelopeGenerator,
    pub fm: Fm,
    pub ch: Channel,
    pub timer_a: TimerA,
    pub timer_b: TimerB,
    pub registers: Registers,

    _phantom: PhantomData<C>,
}

// Public API
impl<C> Opn2<C>
where
    C: Chip,
{
    pub fn reset(&mut self) {
        *self = Opn2::default();
    }

    pub fn clock(&mut self) -> (i16, i16) {
        let slot = self.cycles;
        self.lfo.inc = self.registers.mode_test_21[1];

        self.phase_generator.read >>= 1;
        self.envelope_generator.read[1] >>= 1;
        self.envelope_generator.cycle = self.envelope_generator.cycle.wrapping_add(1);

        // Cycle specific functions
        match self.cycles {
            0 => self.lfo.cycle_0(),
            1 => self.envelope_generator.cycle_1(),
            2 => {
                self.phase_generator.cycle_2();
                self.envelope_generator.cycle_2();
            }
            13 => self.envelope_generator.cycle_13(),
            23 => self.lfo.cycle_23(),
            _ => {}
        }

        self.envelope_generator
            .increment_timer(&self.registers, &self.io);

        self.io.clock();

        self.timer_a.clock(self.cycles, &mut self.registers);

        self.timer_b.clock(self.cycles, &self.registers);

        self.envelope_generator
            .key_on(self.cycles, self.channel, slot, &mut self.registers);

        self.ch_output();

        self.ch
            .generate(self.cycles, self.channel, &self.registers, &self.fm);

        self.fm
            .prepare(self.cycles as usize, self.channel as usize, &self.registers);

        self.fm.generate(
            self.cycles as usize,
            &self.registers,
            &self.phase_generator,
            &self.envelope_generator,
        );

        self.phase_generator
            .generate(self.cycles as usize, &self.registers);

        self.phase_generator.phase_calc_increment(
            self.cycles as usize,
            self.channel as usize,
            &self.registers,
            &self.lfo,
        );

        self.envelope_generator
            .adsr(self.cycles, &mut self.phase_generator);

        self.envelope_generator.generate(
            self.cycles as usize,
            self.channel as usize,
            &self.registers,
        );

        self.envelope_generator.ssg_eg(self.cycles, &self.registers);

        self.envelope_generator.prepare(
            self.cycles,
            self.channel as usize,
            &self.registers,
            &self.phase_generator,
            &self.lfo,
        );

        self.phase_generator
            .fnum_block(slot, self.channel as usize, &self.registers);

        self.lfo.update();

        self.do_reg_write();

        self.cycles = self.cycles.wrapping_add(1) % 24;
        self.channel = self.cycles % 6;

        if self.registers.status_time != 0 {
            self.registers.status_time = self.registers.status_time.wrapping_sub(1)
        };

        (self.mol, self.mor)
    }

    pub fn write(&mut self, mut port: u32, data: u8) {
        port &= 3;
        self.io.write_data = (port << 7 & 0x100 | data as u32) as u16;
        if port & 1 != 0 {
            // Data
            self.io.write_d = (self.io.write_d as i32 | 1) as u8
        } else {
            // Address
            self.io.write_a = (self.io.write_a as i32 | 1) as u8
        };
    }

    pub fn set_test_pin(&mut self, value: u32) {
        self.io.pin_test_in = (value & 1) as u8;
    }

    pub fn read_test_pin(&self) -> u32 {
        if self.registers.mode_test_2c[7] == 0 {
            return 0;
        }

        (self.cycles == 23) as u32
    }

    pub fn read_irq_pin(&self) -> u32 {
        (self.timer_a.overflow_flag as i32 | self.timer_b.overflow_flag as i32) as u32
    }

    pub fn read(&mut self, port: u32) -> u8 {
        if port & 3 == 0 || C::READ_MODE {
            if self.registers.mode_test_21[6] != 0 {
                // Read test data
                let slot = self.cycles.wrapping_add(18) % 24;

                let mut testdata = ((self.phase_generator.read & 0x1) << 15
                    | (self.envelope_generator.read[self.registers.mode_test_21[0] as usize] & 0x1)
                        << 14) as u16;

                if self.registers.mode_test_2c[4] != 0 {
                    testdata = (testdata as i32 | self.ch.read as i32 & 0x1ff) as u16
                } else {
                    testdata = (testdata as i32 | self.fm.out[slot as usize] as i32 & 0x3fff) as u16
                }

                if self.registers.mode_test_21[7] != 0 {
                    self.registers.status = (testdata as i32 & 0xff) as u8
                } else {
                    self.registers.status = (testdata as i32 >> 8) as u8
                }
            } else {
                self.registers.status = ((self.io.busy as i32) << 7
                    | (self.timer_b.overflow_flag as i32) << 1
                    | self.timer_a.overflow_flag as i32)
                    as u8
            }

            self.registers.status_time = C::STATUS_TIME;
        }

        if self.registers.status_time != 0 {
            return self.registers.status;
        }

        0
    }
}

impl From<u32> for Address {
    fn from(address: u32) -> Self {
        match address {
            0x21 => Address::LsiTest1,
            0x22 => Address::Lfo,
            0x24 => Address::TimerAMsb,
            0x25 => Address::TimerALsb,
            0x26 => Address::TimerB,
            0x27 => Address::TimersAndCh3Mode,
            0x28 => Address::KeyOnOff,
            0x2A => Address::DacData,
            0x2B => Address::DacEnable,
            0x2C => Address::LsiTest2,
            0x30 => Address::DetuneAndMultiple,
            0x40 => Address::TotalLevel,
            0x50 => Address::RateScaleAndAttackRate,
            0x60 => Address::FirstDecayAndAmp,
            0x70 => Address::SecondaryDecayRate,
            0x80 => Address::SecondaryAmpAndRelease,
            0x90 => Address::SsgEg,
            0xA0 => Address::Fnum,
            0xA4 => Address::BlockFreq,
            0xA8 => Address::Ch3Fnum,
            0xAC => Address::Ch3BlockFreq,
            0xB0 => Address::FeedbackAndAlgorithm,
            0xB4 => Address::StereoAndLfoSens,
            _ => Address::Invalid,
        }
    }
}

// Private API
impl<C> Opn2<C>
where
    C: Chip,
{
    fn do_reg_write(&mut self) {
        let mut slot = self.cycles % 12;
        let channel = self.channel;

        // Update registers
        if self.io.write_fm_data != 0 {
            // Slot
            if OP_OFFSET[slot as usize] == (self.io.address as i32 & 0x107) as u32 {
                if self.io.address as i32 & 0x8 != 0 {
                    // OP2, OP4
                    slot = slot.wrapping_add(12)
                }
                match Address::from((self.io.address as i32 & 0xf0) as u32) {
                    Address::DetuneAndMultiple => {
                        self.registers.multiple[slot as usize] = (self.io.data as i32 & 0xf) as u8;
                        if self.registers.multiple[slot as usize] == 0 {
                            self.registers.multiple[slot as usize] = 1;
                        } else {
                            self.registers.multiple[slot as usize] =
                                ((self.registers.multiple[slot as usize] as i32) << 1) as u8
                        }
                        self.registers.detune[slot as usize] =
                            (self.io.data as i32 >> 4 & 0x7) as u8;
                    }
                    Address::TotalLevel => {
                        self.registers.total_level[slot as usize] =
                            (self.io.data as i32 & 0x7f) as u8
                    }
                    Address::RateScaleAndAttackRate => {
                        self.registers.attack_rate[slot as usize] =
                            (self.io.data as i32 & 0x1f) as u8;
                        self.registers.rate_scale[slot as usize] =
                            (self.io.data as i32 >> 6 & 0x3) as u8;
                    }
                    Address::FirstDecayAndAmp => {
                        self.registers.decay_rate_first[slot as usize] =
                            (self.io.data as i32 & 0x1f) as u8;
                        self.registers.amplitude_modulation[slot as usize] =
                            (self.io.data as i32 >> 7 & 0x1) as u8;
                    }
                    Address::SecondaryDecayRate => {
                        self.registers.decay_rate_second[slot as usize] =
                            (self.io.data as i32 & 0x1f) as u8;
                    }
                    Address::SecondaryAmpAndRelease => {
                        self.registers.release_rate[slot as usize] =
                            (self.io.data as i32 & 0xf) as u8;
                        self.registers.secondary_amplitude[slot as usize] =
                            (self.io.data as i32 >> 4 & 0xf) as u8;
                        self.registers.secondary_amplitude[slot as usize] =
                            (self.registers.secondary_amplitude[slot as usize] as i32
                                | (self.registers.secondary_amplitude[slot as usize] as i32 + 1)
                                    & 0x10) as u8
                    }
                    Address::SsgEg => {
                        self.registers.ssg_eg[slot as usize] = (self.io.data as i32 & 0xf) as u8
                    }
                    _ => {}
                }
            }

            // Channel
            if CH_OFFSET[channel as usize] == (self.io.address as i32 & 0x103) as u32 {
                match Address::from((self.io.address as i32 & 0xfc) as u32) {
                    Address::Fnum => {
                        self.registers.fnum[channel as usize] = (self.io.data as i32 & 0xff
                            | (self.registers.block_freq as i32 & 0x7) << 8)
                            as u16;
                        self.registers.block[channel as usize] =
                            (self.registers.block_freq as i32 >> 3 & 0x7) as u8;
                        self.registers.kcode[channel as usize] =
                            (((self.registers.block[channel as usize] as i32) << 2) as u32
                                | FN_NOTE
                                    [(self.registers.fnum[channel as usize] as i32 >> 7) as usize])
                                as u8
                    }
                    Address::BlockFreq => {
                        self.registers.block_freq = (self.io.data as i32 & 0xff) as u8
                    }
                    Address::Ch3Fnum => {
                        self.registers.fnum_ch3[channel as usize] = (self.io.data as i32 & 0xff
                            | (self.registers.block_freq_ch3 as i32 & 0x7) << 8)
                            as u16;
                        self.registers.block_ch3[channel as usize] =
                            (self.registers.block_freq_ch3 as i32 >> 3 & 0x7) as u8;
                        self.registers.kcode_ch3[channel as usize] =
                            (((self.registers.block_ch3[channel as usize] as i32) << 2) as u32
                                | FN_NOTE[(self.registers.fnum_ch3[channel as usize] as i32 >> 7)
                                    as usize]) as u8
                    }
                    Address::Ch3BlockFreq => {
                        self.registers.block_freq_ch3 = (self.io.data as i32 & 0xff) as u8
                    }
                    Address::FeedbackAndAlgorithm => {
                        self.registers.connect[channel as usize] =
                            (self.io.data as i32 & 0x7) as u8;
                        self.registers.feedback[channel as usize] =
                            (self.io.data as i32 >> 3 & 0x7) as u8
                    }
                    Address::StereoAndLfoSens => {
                        self.registers.freq_mod_sens[channel as usize] =
                            (self.io.data as i32 & 0x7) as u8;
                        self.registers.amp_mod_sens[channel as usize] =
                            (self.io.data as i32 >> 4 & 0x3) as u8;
                        self.registers.pan_l[channel as usize] =
                            (self.io.data as i32 >> 7 & 0x1) as u8;
                        self.registers.pan_r[channel as usize] =
                            (self.io.data as i32 >> 6 & 0x1) as u8
                    }
                    _ => {}
                }
            }
        }

        if self.io.write_a_en as i32 != 0 || self.io.write_d_en as i32 != 0 {
            // Data
            if self.io.write_a_en != 0 {
                self.io.write_fm_data = 0;
            }

            if self.io.write_fm_address as i32 != 0 && self.io.write_d_en as i32 != 0 {
                self.io.write_fm_data = 1;
            }

            // Address
            if self.io.write_a_en != 0 {
                if self.io.write_data as i32 & 0xf0 != 0 {
                    // FM Write
                    self.io.address = self.io.write_data;
                    self.io.write_fm_address = 1;
                } else {
                    // SSG write
                    self.io.write_fm_address = 0;
                }
            }

            // FM Mode
            // Data
            if self.io.write_d_en as i32 != 0 && self.io.write_data as i32 & 0x100 == 0 {
                match Address::from(self.io.write_fm_mode_a as u32) {
                    Address::LsiTest1 => {
                        // LSI test 1
                        for i in 0..8 {
                            self.registers.mode_test_21[i] =
                                (self.io.write_data as i32 >> i & 0x1) as u8;
                        }
                    }
                    Address::Lfo => {
                        // LFO control
                        if self.io.write_data as i32 >> 3 & 0x1 != 0 {
                            self.lfo.enable = 0x7f;
                        } else {
                            self.lfo.enable = 0;
                        }
                        self.lfo.freq = (self.io.write_data & 0x7) as usize;
                    }
                    Address::TimerAMsb => {
                        self.timer_a.reg = (self.timer_a.reg as i32 & 0x3) as u16;
                        self.timer_a.reg = (self.timer_a.reg as i32
                            | (self.io.write_data as i32 & 0xff) << 2)
                            as u16
                    }
                    Address::TimerALsb => {
                        self.timer_a.reg = (self.timer_a.reg as i32 & 0x3fc) as u16;
                        self.timer_a.reg =
                            (self.timer_a.reg as i32 | self.io.write_data as i32 & 0x3) as u16
                    }
                    Address::TimerB => self.timer_b.reg = (self.io.write_data as i32 & 0xff) as u16,
                    Address::TimersAndCh3Mode => {
                        // CSM, Timer control
                        self.registers.mode_ch3 = ((self.io.write_data as i32 & 0xc0) >> 6) as u8;
                        self.registers.mode_csm =
                            (self.registers.mode_ch3 as i32 == 2) as i32 as u8;
                        self.timer_a.load = self.io.write_data & 0x1 != 0;
                        self.timer_a.enable = self.io.write_data >> 2 & 0x1 != 0;
                        self.timer_a.reset = self.io.write_data >> 4 & 0x1 != 0;
                        self.timer_b.load = self.io.write_data >> 1 & 0x1 != 0;
                        self.timer_b.enable = self.io.write_data >> 3 & 0x1 != 0;
                        self.timer_b.reset = self.io.write_data >> 5 & 0x1 != 0;
                    }
                    Address::KeyOnOff => {
                        // Key on/off
                        for i in 0..4 {
                            self.registers.mode_kon_operator[i] =
                                (self.io.write_data as i32 >> (4 + i) & 0x1) as u8;
                        }

                        if self.io.write_data as i32 & 0x3 == 0x3 {
                            // Invalid address
                            self.registers.mode_kon_channel = 0xff;
                        } else {
                            self.registers.mode_kon_channel = ((self.io.write_data as i32 & 0x3)
                                + (self.io.write_data as i32 >> 2 & 1) * 3)
                                as u8
                        }
                    }
                    Address::DacData => {
                        self.registers.dac_data = (self.registers.dac_data as i32 & 0x1) as i16;
                        self.registers.dac_data = (self.registers.dac_data as i32
                            | (self.io.write_data as i32 ^ 0x80) << 1)
                            as i16
                    }
                    Address::DacEnable => {
                        // DAC enable
                        self.registers.dac_enable = self.io.write_data >> 7 != 0;
                    }
                    Address::LsiTest2 => {
                        // LSI test 2
                        for i in 0..8 {
                            self.registers.mode_test_2c[i] =
                                (self.io.write_data as i32 >> i & 0x1) as u8;
                        }
                        self.registers.dac_data = (self.registers.dac_data as i32 & 0x1fe) as i16;
                        self.registers.dac_data = (self.registers.dac_data as i32
                            | self.registers.mode_test_2c[3] as i32)
                            as i16;
                        self.envelope_generator.custom_timer = (self.registers.mode_test_2c[7] == 0
                            && self.registers.mode_test_2c[6] as i32 != 0)
                            as i32
                            as u8
                    }
                    _ => {}
                }
            }
            // Address
            if self.io.write_a_en != 0 {
                self.io.write_fm_mode_a = (self.io.write_data as i32 & 0x1ff) as u16
            }
        }
        if self.io.write_fm_data != 0 {
            self.io.data = (self.io.write_data as i32 & 0xff) as u8
        };
    }

    fn ch_output(&mut self) {
        let cycles = self.cycles;
        let slot = self.cycles;
        let mut channel = self.channel;
        let test_dac = self.registers.mode_test_2c[5] as u32;
        self.ch.read = self.ch.lock;
        if slot < 12 {
            // Ch 4,5,6
            channel = channel.wrapping_add(1)
        }
        if cycles & 3 == 0 {
            if test_dac == 0 {
                // Lock value
                self.ch.lock = self.ch.out[channel as usize];
            }
            self.ch.lock_l = self.registers.pan_l[channel as usize];
            self.ch.lock_r = self.registers.pan_r[channel as usize];
        }

        // Ch 6
        let mut out;
        if cycles >> 2 == 1 && self.registers.dac_enable as i32 != 0 || test_dac != 0 {
            out = self.registers.dac_data;
            out = ((out as i32) << 7) as i16;
            out = (out as i32 >> 7) as i16;
        } else {
            out = self.ch.lock;
        }

        let (mol, mor) = C::output(self, cycles, test_dac, out);
        self.mol = mol;
        self.mor = mor;
    }
}

impl<C> Opn2Trait for Opn2<C>
where
    C: Chip + Send,
{
    /// Reset emulated chip
    fn reset(&mut self) {
        self.reset()
    }

    /// Advances emulated chip state by 1 internal clock (6 master clocks).
    /// Returns signed 9-bit MOL, MOR pin states.
    fn clock(&mut self) -> (i16, i16) {
        self.clock()
    }

    /// Write 8-bit data to port.
    fn write(&mut self, port: u32, data: u8) {
        self.write(port, data)
    }

    /// Set TEST pin value.
    fn set_test_pin(&mut self, value: u32) {
        self.set_test_pin(value)
    }

    /// Read TEST pin value.
    fn read_test_pin(&self) -> u32 {
        self.read_test_pin()
    }

    /// Read IRQ pin value.
    fn read_irq_pin(&self) -> u32 {
        self.read_irq_pin()
    }

    /// Read chip status.
    fn read(&mut self, port: u32) -> u8 {
        self.read(port)
    }
}
