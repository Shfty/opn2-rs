//! Trait for 

pub trait Opn2Trait {
    /// Reset emulated chip
    fn reset(&mut self);

    /// Advances emulated chip state by 1 internal clock (6 master clocks).
    /// Writes signed 9-bit MOL, MOR pin states to buffer.
    fn clock(&mut self) -> (i16, i16);

    /// Write 8-bit data to port.
    fn write(&mut self, port: u32, data: u8);

    /// Set TEST pin value.
    fn set_test_pin(&mut self, value: u32);

    /// Read TEST pin value.
    fn read_test_pin(&self) -> u32;

    /// Read IRQ pin value.
    fn read_irq_pin(&self) -> u32;

    /// Read chip status.
    fn read(&mut self, port: u32) -> u8;
}
