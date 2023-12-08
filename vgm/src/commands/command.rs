use opn2::WaitSamples;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum VgmCommand {
    WriteYm2612(u32, u8, u8),
    GameGearPsgStereo(u8),
    WritePsg(u8),
    Wait(WaitSamples),
}
