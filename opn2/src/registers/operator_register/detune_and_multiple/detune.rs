use std::fmt::Debug;


#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[allow(dead_code)]
pub enum Detune {
    Plus0,
    Plus1,
    Plus2,
    Plus3,
    Minus0,
    Minus1,
    Minus2,
    Minus3,
}

impl Debug for Detune {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Detune({})", match self {
            Detune::Plus0 => "+0",
            Detune::Plus1 => "+1",
            Detune::Plus2 => "+2",
            Detune::Plus3 => "+3",
            Detune::Minus0 => "-0",
            Detune::Minus1 => "-1",
            Detune::Minus2 => "-2",
            Detune::Minus3 => "-3",
        })
    }
}

impl Default for Detune {
    fn default() -> Self {
        Detune::Plus0
    }
}

impl From<Detune> for u8 {
    fn from(val: Detune) -> Self {
        match val {
            Detune::Plus0 => 0,
            Detune::Plus1 => 1,
            Detune::Plus2 => 2,
            Detune::Plus3 => 3,
            Detune::Minus0 => 4,
            Detune::Minus1 => 5,
            Detune::Minus2 => 6,
            Detune::Minus3 => 7,
        }
    }
}

impl From<u8> for Detune {
    fn from(dt: u8) -> Self {
        match dt {
            0 => Detune::Plus0,
            1 => Detune::Plus1,
            2 => Detune::Plus2,
            3 => Detune::Plus3,
            4 => Detune::Minus0,
            5 => Detune::Minus1,
            6 => Detune::Minus2,
            7 => Detune::Minus3,
            _ => panic!("Invalid detune {:?}", dt),
        }
    }
}