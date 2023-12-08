#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Io {
    pub write_data: u16,
    pub write_a: u8,
    pub write_d: u8,
    pub write_a_en: u8,
    pub write_d_en: u8,
    pub write_busy: u8,
    pub write_busy_cnt: u8,
    pub write_fm_address: u8,
    pub write_fm_data: u8,
    pub write_fm_mode_a: u16,
    pub address: u16,
    pub data: u8,
    pub pin_test_in: u8,
    pub pin_irq: u8,
    pub busy: u8,
}

impl Io {
    pub fn clock(&mut self) {
        // Write signal check
        self.write_a_en = (self.write_a & 0x3 == 0x1) as u8;
        self.write_d_en = (self.write_d & 0x3 == 0x1) as u8;
        self.write_a <<= 1;
        self.write_d <<= 1;

        // Busy counter
        self.busy = self.write_busy;
        self.write_busy_cnt += self.write_busy;
        self.write_busy =
            (self.write_busy != 0 && self.write_busy_cnt >> 5 == 0 || self.write_d_en != 0) as u8;
        self.write_busy_cnt &= 0x1f;
    }
}
