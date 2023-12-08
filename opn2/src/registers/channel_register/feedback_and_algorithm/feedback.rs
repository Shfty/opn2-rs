#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Feedback {
    Feedback1,
    Feedback2,
    Feedback3,
    Feedback4,
    Feedback5,
    Feedback6,
    Feedback7,
    Feedback8,
}

impl Default for Feedback {
    fn default() -> Self {
        Feedback::Feedback1
    }
}

impl From<u8> for Feedback {
    fn from(fb: u8) -> Self {
        match fb {
            0 => Feedback::Feedback1,
            1 => Feedback::Feedback2,
            2 => Feedback::Feedback3,
            3 => Feedback::Feedback4,
            4 => Feedback::Feedback5,
            5 => Feedback::Feedback6,
            6 => Feedback::Feedback7,
            7 => Feedback::Feedback8,
            _ => panic!("Feedback should be a 3-bit number"),
        }
    }
}

impl From<Feedback> for u8 {
    fn from(val: Feedback) -> Self {
        match val {
            Feedback::Feedback1 => 0,
            Feedback::Feedback2 => 1,
            Feedback::Feedback3 => 2,
            Feedback::Feedback4 => 3,
            Feedback::Feedback5 => 4,
            Feedback::Feedback6 => 5,
            Feedback::Feedback7 => 6,
            Feedback::Feedback8 => 7,
        }
    }
}
