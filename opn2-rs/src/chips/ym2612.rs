use crate::opn2::{
    traits::{Chip, Dac, ReadMode, StatusTime},
    Opn2,
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ym2612;

impl Dac for Ym2612 {
    fn output<C: Chip>(chip: &Opn2<C>, cycles: u32, test_dac: u32, mut out: i16) -> (i16, i16) {
        let out_en = (cycles & 3 == 3 || test_dac != 0) as i32 as u32;
        // YM2612 DAC emulation(not verified)
        let mut sign = (out as i32 >> 8) as i16;
        if out as i32 >= 0 {
            out += 1;
            sign += 1
        }

        let mol = if chip.ch.lock_l as i32 != 0 && out_en != 0 {
            out
        } else {
            sign
        };

        let mor = if chip.ch.lock_r as i32 != 0 && out_en != 0 {
            out
        } else {
            sign
        };

        // Amplify signal
        let mol = (mol as i32 * 3) as i16;
        let mor = (mor as i32 * 3) as i16;

        (mol, mor)
    }
}

impl StatusTime for Ym2612 {
    const STATUS_TIME: u32 = 300000;
}

impl ReadMode for Ym2612 {
    const READ_MODE: bool = false;
}
