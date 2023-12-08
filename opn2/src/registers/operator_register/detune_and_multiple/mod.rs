mod detune;
mod multiple;

use std::fmt::Debug;

pub use detune::*;
pub use multiple::*;

use super::{OperatorRegister, Register};

#[derive(Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct DetuneAndMultiple {
    detune: Detune,
    multiple: Multiple,
}

impl DetuneAndMultiple {
    pub fn new(dt1: Detune, mul: Multiple) -> Self {
        DetuneAndMultiple {
            detune: dt1,
            multiple: mul,
        }
    }
}

// Getters
impl DetuneAndMultiple {
    pub fn get_detune(&self) -> Detune {
        self.detune
    }

    pub fn get_multiple(&self) -> Multiple {
        self.multiple
    }
}

// Setters
impl DetuneAndMultiple {
    pub fn set_detune<T>(&mut self, detune: T)
    where
        T: Into<Detune>,
    {
        self.detune = detune.into();
    }

    pub fn set_multiple<T>(&mut self, multiple: T)
    where
        T: Into<Multiple>,
    {
        self.multiple = multiple.into();
    }
}

impl From<u8> for DetuneAndMultiple {
    fn from(dt1_and_mul: u8) -> Self {
        let dt1 = (dt1_and_mul >> 4) & 0b111;
        let mul = dt1_and_mul & 0b1111;
        DetuneAndMultiple::new(dt1.into(), mul.into())
    }
}

impl From<DetuneAndMultiple> for u8 {
    fn from(val: DetuneAndMultiple) -> Self {
        ((val.detune as u8) << 4) | val.multiple as u8
    }
}

impl Debug for DetuneAndMultiple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("DetuneAndMultiple")
            .field(&self.detune)
            .field(&self.multiple)
            .finish()
    }
}

impl Register for DetuneAndMultiple {
    const BASE_ADDRESS: u8 = 0x30;
}

impl OperatorRegister for DetuneAndMultiple {
    fn get_data(&self) -> u8 {
        (*self).into()
    }
}
