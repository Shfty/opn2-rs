//! OPN2 implementation using `libnuked-opn2-sys`

pub use libnuked_opn2_sys::Ym3438;
use opn2_trait::Opn2Trait;

use std::ops::{Deref, DerefMut};

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Opn2(pub Ym3438);

impl Deref for Opn2 {
    type Target = Ym3438;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Opn2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Opn2Trait for Opn2 {
    /// Reset emulated chip
    fn reset(&mut self) {
        self.deref_mut().reset()
    }

    /// Advances emulated chip state by 1 internal clock (6 master clocks). Returns signed 9-bit MOL, MOR pin states.
    fn clock(&mut self) -> (i16, i16) {
        self.deref_mut().clock()
    }

    /// Write 8-bit data to port.
    fn write(&mut self, port: u32, data: u8) {
        self.deref_mut().write(port, data);
    }

    /// Set TEST pin value.
    fn set_test_pin(&mut self, value: u32) {
        self.deref_mut().set_test_pin(value);
    }

    /// Read TEST pin value.
    fn read_test_pin(&self) -> u32 {
        self.deref().read_test_pin()
    }

    /// Read IRQ pin value.
    fn read_irq_pin(&self) -> u32 {
        self.deref().read_irq_pin()
    }

    /// Read chip status.
    fn read(&mut self, port: u32) -> u8 {
        self.deref_mut().read(port)
    }
}
