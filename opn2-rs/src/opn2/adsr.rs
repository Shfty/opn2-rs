#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum Adsr {
    Attack,
    Decay,
    Sustain,
    Release,
}
