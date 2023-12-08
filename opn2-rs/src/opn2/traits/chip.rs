use super::{Dac, ReadMode, StatusTime};

pub trait Chip: Default + Dac + StatusTime + ReadMode {}
impl<T> Chip for T where T: Default + Dac + StatusTime + ReadMode {}
