use crate::opn2::{
    traits::{Chip, Dac, ReadMode, StatusTime},
    Opn2,
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ym3438;

impl Dac for Ym3438 {
    fn output<C: Chip>(chip: &Opn2<C>, cycles: u32, test_dac: u32, out: i16) -> (i16, i16) {
        let out_en = (cycles & 3 != 0 || test_dac != 0) as i32 as u32;

        let mut mol = 0i16;
        if chip.ch.lock_l as i32 != 0 && out_en != 0 {
            mol = out;
        }

        let mut mor = 0i16;
        if chip.ch.lock_r as i32 != 0 && out_en != 0 {
            mor = out;
        }

        (mol, mor)
    }
}

impl StatusTime for Ym3438 {
    const STATUS_TIME: u32 = 40000000;
}

impl ReadMode for Ym3438 {
    const READ_MODE: bool = true;
}
