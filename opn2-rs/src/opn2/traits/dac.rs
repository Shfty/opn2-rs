use crate::opn2::Opn2;

use super::Chip;

pub trait Dac {
    fn output<C: Chip>(chip: &Opn2<C>, cycles: u32, test_dac: u32, out: i16) -> (i16, i16);
}
