#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Algorithm {
    /// Distortion guitar, "high hat chopper", bass...
    Algorithm1,
    /// Harp, PSG sound
    Algorithm2,
    /// Bass, electric guitar, brass, piano, woods
    Algorithm3,
    /// Strings, folk guitar, chimes
    Algorithm4,
    /// Flute, bells, chorus, bass drum, snare drum, tom-tom
    Algorithm5,
    /// Brass, organ
    Algorithm6,
    /// Xylophone, tom-tom, organ, vibraphone, snare drum, bass drum
    Algorithm7,
    /// Pipe organ
    Algorithm8,
}

impl Default for Algorithm {
    fn default() -> Self {
        Algorithm::Algorithm1
    }
}

impl From<u8> for Algorithm {
    fn from(algorithm: u8) -> Self {
        match algorithm {
            0 => Algorithm::Algorithm1,
            1 => Algorithm::Algorithm2,
            2 => Algorithm::Algorithm3,
            3 => Algorithm::Algorithm4,
            4 => Algorithm::Algorithm5,
            5 => Algorithm::Algorithm6,
            6 => Algorithm::Algorithm7,
            7 => Algorithm::Algorithm8,
            _ => panic!("Algorithm should be a 3-bit number"),
        }
    }
}

impl From<Algorithm> for u8 {
    fn from(val: Algorithm) -> Self {
        match val {
            Algorithm::Algorithm1 => 0,
            Algorithm::Algorithm2 => 1,
            Algorithm::Algorithm3 => 2,
            Algorithm::Algorithm4 => 3,
            Algorithm::Algorithm5 => 4,
            Algorithm::Algorithm6 => 5,
            Algorithm::Algorithm7 => 6,
            Algorithm::Algorithm8 => 7,
        }
    }
}
