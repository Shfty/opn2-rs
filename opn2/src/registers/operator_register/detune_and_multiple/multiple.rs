use std::fmt::Debug;


#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[allow(dead_code)]
pub enum Multiple {
    Mul0_5,
    Mul1,
    Mul2,
    Mul3,
    Mul4,
    Mul5,
    Mul6,
    Mul7,
    Mul8,
    Mul9,
    Mul10,
    Mul11,
    Mul12,
    Mul13,
    Mul14,
    Mul15,
}

impl Debug for Multiple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Multiple({})", match self {
            Multiple::Mul0_5 => "0.5",
            Multiple::Mul1 => "1",
            Multiple::Mul2 => "2",
            Multiple::Mul3 => "3",
            Multiple::Mul4 => "4",
            Multiple::Mul5 => "5",
            Multiple::Mul6 => "6",
            Multiple::Mul7 => "7",
            Multiple::Mul8 => "8",
            Multiple::Mul9 => "9",
            Multiple::Mul10 => "10",
            Multiple::Mul11 => "11",
            Multiple::Mul12 => "12",
            Multiple::Mul13 => "13",
            Multiple::Mul14 => "14",
            Multiple::Mul15 => "15",
        })
    }
}

impl Default for Multiple {
    fn default() -> Self {
        Multiple::Mul1
    }
}

impl From<Multiple> for u8 {
    fn from(val: Multiple) -> Self {
        match val {
            Multiple::Mul0_5 => 0,
            Multiple::Mul1 => 1,
            Multiple::Mul2 => 2,
            Multiple::Mul3 => 3,
            Multiple::Mul4 => 4,
            Multiple::Mul5 => 5,
            Multiple::Mul6 => 6,
            Multiple::Mul7 => 7,
            Multiple::Mul8 => 8,
            Multiple::Mul9 => 9,
            Multiple::Mul10 => 10,
            Multiple::Mul11 => 11,
            Multiple::Mul12 => 12,
            Multiple::Mul13 => 13,
            Multiple::Mul14 => 14,
            Multiple::Mul15 => 15,
        }
    }
}

impl From<u8> for Multiple {
    fn from(dt: u8) -> Self {
        match dt {
            0 => Multiple::Mul0_5,
            1 => Multiple::Mul1,
            2 => Multiple::Mul2,
            3 => Multiple::Mul3,
            4 => Multiple::Mul4,
            5 => Multiple::Mul5,
            6 => Multiple::Mul6,
            7 => Multiple::Mul7,
            8 => Multiple::Mul8,
            9 => Multiple::Mul9,
            10 => Multiple::Mul10,
            11 => Multiple::Mul11,
            12 => Multiple::Mul12,
            13 => Multiple::Mul13,
            14 => Multiple::Mul14,
            15 => Multiple::Mul15,
            _ => panic!("Invalid multiple"),
        }
    }
}